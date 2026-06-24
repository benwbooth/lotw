//! Stackful-coroutine frame runner for the game loop.
//!
//! # Coroutine model
//!
//! The game is a translation of 6502 code structured as an infinite loop that
//! blocks at vblank waits deep inside nested calls. To suspend that whole call
//! stack at a frame boundary and resume it next frame, the game runs inside a
//! stackful coroutine on its own stack; [`frame_wait`] suspends it back to the
//! control loop, which does the per-frame hardware work and resumes it. Only one
//! side ever runs at a time (cooperative), all on a single OS thread.
//!
//! # Aliasing miscompile and the `black_box` barriers
//!
//! The whole game body holds a single `&mut Engine` live across every suspend
//! point. Under that borrow, an optimizing (release) build is entitled to
//! assume the `&mut` is *unaliased* for its entire lifetime — but it is in fact
//! aliased, because the control loop reads/writes the same `Engine` (through the
//! `UnsafeCell` raw pointers) while the coroutine is suspended. With nothing to
//! force the pointer to "escape", the optimizer concludes the game's writes are
//! never observed and elides them; the control loop's reads then see a blank
//! engine and render black. The [`std::hint::black_box`] calls scattered through
//! this module exist solely to force the engine/regs pointers to escape to
//! opaque code at each suspend/resume boundary, defeating that miscompile. They
//! are load-bearing: removing them reintroduces the blank-render bug in release
//! builds. (The volatile `byte()` RAM accesses used to mask this implicitly.)

use std::cell::{Cell, UnsafeCell};

use corosensei::{Coroutine, CoroutineResult, Yielder};

use crate::{Engine, RoutineContext, engine::RoutineFn};

thread_local! {
    /// Pointer to the running coroutine's [`Yielder`], published while the game
    /// coroutine body executes so [`frame_wait`] (called many frames deep) can
    /// suspend. Null when no coroutine is active (e.g. routines called directly
    /// from tests), in which case [`frame_wait`] is a no-op.
    static YIELDER: Cell<*const Yielder<(), ()>> = const { Cell::new(std::ptr::null()) };
}

/// Whether the frame runner has been asked to stop.
///
/// Shutdown now unwinds the coroutine stack on drop rather than signalling a
/// stop flag, so this always returns `false`. The in-loop callers below retain
/// their checks for clarity (and in case the stop mechanism returns), but the
/// checks are presently dead.
pub fn frame_runner_stop_requested() -> bool {
    // Shutdown now unwinds the coroutine stack on drop rather than signalling a
    // stop flag, so the in-loop checks that used this always observe `false`.
    false
}

/// Suspend the game coroutine back to the control loop until the next frame.
///
/// Called from arbitrarily deep within the game's call stack. `engine` is the
/// single long-lived `&mut Engine` the game body threads through every call;
/// `r` is the per-routine register/context block.
pub fn frame_wait(engine: &mut Engine, r: &mut RoutineContext) {
    // Look up the currently-published yielder for this thread. If there is no
    // active coroutine (e.g. a routine called directly from a test), suspension
    // is impossible, so behave as a no-op and return.
    let yielder = YIELDER.with(|y| y.get());
    if yielder.is_null() {
        return;
    }
    // The control loop may overwrite the shared RoutineContext while we are
    // suspended (e.g. vblank work), so save and restore the game's registers
    // across the suspension point.
    let saved = *r;
    // Force the engine pointer to escape to opaque code at every suspend. The
    // game holds a single `&mut Engine` live across all suspends; without this,
    // a release build assumes that `&mut` is unaliased and elides the game's
    // writes (the control loop's reads then see a blank engine and render
    // black). The volatile `byte()` RAM accesses used to mask this implicitly.
    // See the module-level docs for the full explanation. This barrier (before
    // the suspend) and its twin (after) are load-bearing — do not remove.
    std::hint::black_box(engine as *mut Engine);
    // Hand control back to the control loop. Safety: the yielder is valid for
    // the whole coroutine body, which is the only place this pointer is
    // non-null, and the coroutine runs on this thread.
    unsafe { (*yielder).suspend(()) };
    // Second escape barrier (post-resume): on resume the optimizer must again be
    // prevented from assuming the engine is unchanged by the control loop.
    std::hint::black_box(engine as *mut Engine);
    // Restore the game's registers, undoing any clobbering the control loop did
    // to the shared RoutineContext while we were parked.
    *r = saved;
}

/// Owns the game coroutine together with the heap-pinned [`Engine`] and
/// [`RoutineContext`] it mutates through raw pointers, and drives it one frame
/// at a time.
pub struct FrameRunner {
    /// The stackful coroutine running the game body. It borrows
    /// `engine`/`regs` through raw pointers, so it must drop *first* (its drop
    /// unwinds the suspended game stack); struct field declaration order is the
    /// drop order, so this field stays at the top.
    coro: Coroutine<(), (), ()>,
    /// Heap-pinned engine state. Boxed so its address is stable across moves of
    /// the `FrameRunner`, keeping the coroutine's raw pointer valid. `UnsafeCell`
    /// because both the coroutine and the control loop mutate it (never
    /// simultaneously — the coroutine is suspended when the loop touches it).
    engine: Box<UnsafeCell<Engine>>,
    /// Heap-pinned per-routine register/context block, boxed and `UnsafeCell`-d
    /// for the same reasons as `engine`.
    regs: Box<UnsafeCell<RoutineContext>>,
    /// Set once the coroutine body returns (the game finished); subsequent
    /// resume requests become no-ops.
    done: bool,
}

impl FrameRunner {
    /// Build a runner that will execute `entry` on a fresh coroutine stack,
    /// taking ownership of `engine` and giving it (and a default
    /// [`RoutineContext`]) a stable heap address. The coroutine is created but
    /// not started; call [`start`](Self::start) to run to the first frame wait.
    pub fn new(engine: Engine, entry: RoutineFn) -> Self {
        // Box the engine and a fresh register block so they have stable heap
        // addresses; the coroutine captures raw pointers into them below.
        let engine = Box::new(UnsafeCell::new(engine));
        let regs = Box::new(UnsafeCell::new(RoutineContext::default()));
        // Box gives the engine/regs a stable heap address that survives moving
        // the FrameRunner, so these raw pointers stay valid for the coroutine.
        //
        // Keep them as real pointers (not a `usize` round-trip): the pointers
        // alias `self.engine`/`self.regs` through the same `UnsafeCell`, and
        // casting through `usize` strips that provenance, so a release-optimized
        // build treats the coroutine's writes and `engine()`'s reads as
        // non-aliasing and the game's state never becomes visible to rendering.
        // A `usize` is `Send` but a raw pointer is not, so wrap to capture it.
        //
        // `Ptrs` is exactly that wrapper: it carries the two raw pointers into
        // the coroutine closure with their provenance intact, and the manual
        // `unsafe impl Send` is what lets the (Send-requiring) closure capture
        // them. Do not replace this with a `usize` round-trip — see above.
        struct Ptrs(*mut Engine, *mut RoutineContext);
        unsafe impl Send for Ptrs {}
        let ptrs = Ptrs(engine.get(), regs.get());
        // Construct the coroutine. Its body is the game's `entry` routine; the
        // closure receives the yielder, publishes it, runs the game, and tidies
        // up. The closure is not run until the coroutine is first resumed.
        let coro = Coroutine::new(move |yielder: &Yielder<(), ()>, _input: ()| {
            // Recover the engine/regs raw pointers captured (with provenance)
            // via the Send wrapper.
            let Ptrs(engine_ptr, regs_ptr) = ptrs;
            // Publish this coroutine's yielder so `frame_wait`, however deep in
            // the call stack, can find it; remember any previous value to
            // restore on exit (supports nesting / re-entrancy).
            let previous = YIELDER.with(|y| y.replace(yielder as *const _));
            // Run the game body. Safety: engine/regs outlive the coroutine (it
            // drops first), and the game body holds the only active borrow while
            // running.
            unsafe {
                entry(&mut *engine_ptr, &mut *regs_ptr);
            }
            // Game finished: restore the previously-published yielder so the
            // thread-local is left as we found it.
            YIELDER.with(|y| y.set(previous));
        });
        // Assemble the runner. Field order matters for drop order (see `coro`).
        Self {
            coro,
            engine,
            regs,
            done: false,
        }
    }

    /// Run the game until its first frame wait. Returns true if it parked,
    /// false if it ran to completion.
    pub fn start(&mut self) -> bool {
        // First resume kicks off the coroutine body from the top.
        self.step()
    }

    /// Resume the parked game until its next frame wait. Returns false once the
    /// game has finished.
    pub fn resume_until_wait(&mut self) -> bool {
        // Once the game has returned, there is nothing left to resume.
        if self.done {
            return false;
        }
        self.step()
    }

    /// Resume the coroutine once: run the game until it suspends or returns.
    /// Returns `true` if it parked at a frame wait, `false` if it finished.
    fn step(&mut self) -> bool {
        // Hand control to the game coroutine; it runs on its own stack until it
        // either suspends (yield) or returns.
        let result = self.coro.resume(());
        // The coroutine ran the game on its own stack and mutated the engine via
        // raw pointers; make those pointers escape through an opaque barrier so
        // the optimizer cannot assume the heap engine/regs are unchanged and
        // must reload them for `engine()`/`regs()` (otherwise a release build
        // renders a blank engine). Load-bearing — see module docs.
        std::hint::black_box((self.engine.get(), self.regs.get()));
        // Translate the coroutine result: a yield means parked (more frames to
        // come); a return means the game finished, so latch `done`.
        match result {
            CoroutineResult::Yield(()) => true,
            CoroutineResult::Return(()) => {
                self.done = true;
                false
            }
        }
    }

    /// Whether the game coroutine has run to completion.
    pub fn done(&self) -> bool {
        self.done
    }

    /// Shared access to the per-routine register block.
    pub fn regs(&self) -> &RoutineContext {
        // Safety: the game coroutine is suspended whenever the control loop
        // touches regs, so there is no concurrent borrow.
        unsafe { &*self.regs.get() }
    }

    /// Exclusive access to the per-routine register block.
    pub fn regs_mut(&mut self) -> &mut RoutineContext {
        // Safety: see `regs`.
        unsafe { &mut *self.regs.get() }
    }

    /// Shared access to the engine state (for rendering/inspection between
    /// frames).
    pub fn engine(&self) -> &Engine {
        // Safety: see `regs`.
        unsafe { &*self.engine.get() }
    }

    /// Exclusive access to the engine state (for the control loop's per-frame
    /// hardware work while the coroutine is suspended).
    pub fn engine_mut(&mut self) -> &mut Engine {
        // Safety: see `regs`.
        unsafe { &mut *self.engine.get() }
    }

    /// Run `f` with exclusive access to both the engine and register block at
    /// once, returning its result. Convenience for control-loop work that needs
    /// both; safe to call only while the coroutine is suspended.
    pub fn with_engine_regs<R>(
        &mut self,
        f: impl FnOnce(&mut Engine, &mut RoutineContext) -> R,
    ) -> R {
        // Safety: see `regs`.
        unsafe { f(&mut *self.engine.get(), &mut *self.regs.get()) }
    }
}

/// Suspend for exactly one frame. Thin alias for [`frame_wait`], named to match
/// the original 6502 call site.
pub fn wait_frame(engine: &mut Engine, r: &mut RoutineContext) {
    frame_wait(engine, r);
}

/// Spin (one [`wait_frame`] per iteration) until the engine's frame counter
/// reaches zero. The NMI handler decrements the counter each frame, so this is
/// the game's "wait N frames" primitive after the counter has been seeded.
pub fn wait_for_frame_counter(engine: &mut Engine, r: &mut RoutineContext) {
    while engine.state.frame_counter != 0 {
        // Yield this frame, then re-check the counter.
        wait_frame(engine, r);
        // Bail out early if a shutdown has been requested (currently never).
        if frame_runner_stop_requested() {
            return;
        }
    }
}

/// Spin until the pending VRAM (PPU) upload request clears, i.e. the queued
/// graphics work has been flushed during vblank.
pub fn wait_for_ppu_job_idle(engine: &mut Engine, r: &mut RoutineContext) {
    while engine.state.nmi_vram_req != 0 {
        // Yield this frame, then re-check the request flag.
        wait_frame(engine, r);
        // Bail out early on shutdown request (currently never).
        if frame_runner_stop_requested() {
            return;
        }
    }
}

/// Spin until the engine's general-purpose countdown timer reaches zero.
pub fn wait_for_countdown_timer(engine: &mut Engine, r: &mut RoutineContext) {
    while engine.state.countdown_timer != 0 {
        // Yield this frame, then re-check the timer.
        wait_frame(engine, r);
        // Bail out early on shutdown request (currently never).
        if frame_runner_stop_requested() {
            return;
        }
    }
}

/// Flush at most one piece of queued per-frame graphics work, in priority
/// order. Mirrors the original main loop's single-branch dispatch: a scroll-edge
/// column upload takes priority over a HUD refresh, which takes priority over a
/// palette buffer upload. The chosen flag is cleared before doing its work
/// (except the palette path, which keys off the frame counter and is reset
/// elsewhere).
pub fn commit_frame_work(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.state.main_loop_phase != 0 {
        // Highest priority: a room column scrolled into view needs uploading.
        engine.state.main_loop_phase = 0;
        crate::game::upload_scroll_edge_room_column(engine, r);
    } else if engine.state.hud_refresh_flag != 0 {
        // Next: the resource HUD changed and must be redrawn.
        engine.state.hud_refresh_flag = 0;
        crate::game::upload_resource_hud(engine, r);
    } else if engine.state.frame_counter != 0 {
        // Otherwise, if a frame is pending, push any palette changes.
        crate::game::upload_palette_buffer(engine, r);
    }
}

/// Read the controller, latch the button bitmask into the routine's `value`
/// register, and return it. The `as u8` truncates to the 8 NES button bits
/// before widening back to `i32` for the return value.
pub fn read_buttons(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
    // Sample the hardware controllers into engine state.
    crate::game::read_controllers(engine, r);
    // Latch the 8-bit button mask into the routine register.
    r.value = (engine.state.buttons as u8);
    // Return the same value as the routine's i32 result.
    (r.value as i32)
}

/// Render one frame of the scene (player, room objects, status items), flush the
/// queued frame work, wait for the frame to be presented, then read and return
/// the controller state. Used by interactive prompts that draw and poll input
/// together.
pub fn redraw_scene_and_read_buttons(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
    // Request a single frame to be presented.
    engine.state.frame_counter = 1;
    // Build the sprite list for this frame, layered in draw order.
    crate::game::draw_player_sprites(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    crate::game::draw_status_item_sprites(engine, r);
    // Flush queued graphics work, then block until the frame is presented.
    commit_frame_work(engine, r);
    wait_for_frame_counter(engine, r);
    // Sample and return the controller after the frame has gone out.
    read_buttons(engine, r)
}

/// Seed the frame counter with `frames` and block until it counts back down to
/// zero — i.e. wait `frames` frames. `frames` is truncated to a `u8` to match
/// the single-byte hardware counter.
pub fn set_frame_counter_and_wait(engine: &mut Engine, r: &mut RoutineContext, frames: i32) {
    // Seed the one-byte frame counter (truncating to its hardware width).
    engine.state.frame_counter = (frames as u8);
    // Block until the NMI handler has decremented it to zero.
    wait_for_frame_counter(engine, r);
}
