use std::cell::{Cell, UnsafeCell};

use corosensei::{Coroutine, CoroutineResult, Yielder};

use crate::{Engine, RoutineContext, engine::RoutineFn};

// The game is a translation of 6502 code structured as an infinite loop that
// blocks at vblank waits deep inside nested calls. To suspend that whole call
// stack at a frame boundary and resume it next frame, the game runs inside a
// stackful coroutine on its own stack; `frame_wait` suspends it back to the
// control loop, which does the per-frame hardware work and resumes it. Only one
// side ever runs at a time (cooperative), all on a single OS thread.

thread_local! {
    // Pointer to the running coroutine's yielder, published while the game
    // coroutine body executes so `frame_wait` (called many frames deep) can
    // suspend. Null when no coroutine is active (e.g. routines called directly
    // from tests), in which case `frame_wait` is a no-op.
    static YIELDER: Cell<*const Yielder<(), ()>> = const { Cell::new(std::ptr::null()) };
}

pub fn frame_runner_stop_requested() -> bool {
    // Shutdown now unwinds the coroutine stack on drop rather than signalling a
    // stop flag, so the in-loop checks that used this always observe `false`.
    false
}

pub fn frame_wait(_engine: &mut Engine, r: &mut RoutineContext) {
    let yielder = YIELDER.with(|y| y.get());
    if yielder.is_null() {
        return;
    }
    // The control loop may overwrite the shared RoutineContext while we are
    // suspended (e.g. vblank work), so save and restore the game's registers
    // across the suspension point.
    let saved = *r;
    // Safety: the yielder is valid for the whole coroutine body, which is the
    // only place this pointer is non-null, and the coroutine runs on this
    // thread.
    unsafe { (*yielder).suspend(()) };
    *r = saved;
}

pub struct FrameRunner {
    // `coro` borrows `engine`/`regs` through raw pointers, so it must drop
    // first (its drop unwinds the suspended game stack); declaration order is
    // the drop order.
    coro: Coroutine<(), (), ()>,
    engine: Box<UnsafeCell<Engine>>,
    regs: Box<UnsafeCell<RoutineContext>>,
    done: bool,
}

impl FrameRunner {
    pub fn new(engine: Engine, entry: RoutineFn) -> Self {
        let engine = Box::new(UnsafeCell::new(engine));
        let regs = Box::new(UnsafeCell::new(RoutineContext::default()));
        // Box gives the engine/regs a stable heap address that survives moving
        // the FrameRunner, so these raw pointers stay valid for the coroutine.
        let engine_ptr = engine.get() as usize;
        let regs_ptr = regs.get() as usize;
        let coro = Coroutine::new(move |yielder: &Yielder<(), ()>, _input: ()| {
            let previous = YIELDER.with(|y| y.replace(yielder as *const _));
            // Safety: engine/regs outlive the coroutine (it drops first), and
            // the game body holds the only active borrow while running.
            unsafe {
                entry(
                    &mut *(engine_ptr as *mut Engine),
                    &mut *(regs_ptr as *mut RoutineContext),
                );
            }
            YIELDER.with(|y| y.set(previous));
        });
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
        self.step()
    }

    /// Resume the parked game until its next frame wait. Returns false once the
    /// game has finished.
    pub fn resume_until_wait(&mut self) -> bool {
        if self.done {
            return false;
        }
        self.step()
    }

    fn step(&mut self) -> bool {
        match self.coro.resume(()) {
            CoroutineResult::Yield(()) => true,
            CoroutineResult::Return(()) => {
                self.done = true;
                false
            }
        }
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn regs(&self) -> &RoutineContext {
        // Safety: the game coroutine is suspended whenever the control loop
        // touches regs, so there is no concurrent borrow.
        unsafe { &*self.regs.get() }
    }

    pub fn regs_mut(&mut self) -> &mut RoutineContext {
        // Safety: see `regs`.
        unsafe { &mut *self.regs.get() }
    }

    pub fn engine(&self) -> &Engine {
        // Safety: see `regs`.
        unsafe { &*self.engine.get() }
    }

    pub fn engine_mut(&mut self) -> &mut Engine {
        // Safety: see `regs`.
        unsafe { &mut *self.engine.get() }
    }

    pub fn with_engine_regs<R>(
        &mut self,
        f: impl FnOnce(&mut Engine, &mut RoutineContext) -> R,
    ) -> R {
        // Safety: see `regs`.
        unsafe { f(&mut *self.engine.get(), &mut *self.regs.get()) }
    }
}

pub fn wait_frame(engine: &mut Engine, r: &mut RoutineContext) {
    frame_wait(engine, r);
}

pub fn wait_for_frame_counter(engine: &mut Engine, r: &mut RoutineContext) {
    while engine.state.frame_counter != 0 {
        wait_frame(engine, r);
        if frame_runner_stop_requested() {
            return;
        }
    }
}

pub fn wait_for_ppu_job_idle(engine: &mut Engine, r: &mut RoutineContext) {
    while engine.state.nmi_vram_req != 0 {
        wait_frame(engine, r);
        if frame_runner_stop_requested() {
            return;
        }
    }
}

pub fn wait_for_countdown_timer(engine: &mut Engine, r: &mut RoutineContext) {
    while engine.state.countdown_timer != 0 {
        wait_frame(engine, r);
        if frame_runner_stop_requested() {
            return;
        }
    }
}

pub fn commit_frame_work(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.state.main_loop_phase != 0 {
        engine.state.main_loop_phase = 0;
        crate::game::upload_scroll_edge_room_column(engine, r);
    } else if engine.state.hud_refresh_flag != 0 {
        engine.state.hud_refresh_flag = 0;
        crate::game::upload_resource_hud(engine, r);
    } else if engine.state.frame_counter != 0 {
        crate::game::upload_palette_buffer(engine, r);
    }
}

pub fn read_buttons(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
    crate::game::read_controllers(engine, r);
    r.value = ((engine.state.buttons) as u8);
    ((r.value) as i32)
}

pub fn redraw_scene_and_read_buttons(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
    engine.state.frame_counter = 1;
    crate::game::draw_player_sprites(engine, r);
    crate::game::draw_room_object_sprites(engine, r);
    crate::game::draw_status_item_sprites(engine, r);
    commit_frame_work(engine, r);
    wait_for_frame_counter(engine, r);
    read_buttons(engine, r)
}

pub fn set_frame_counter_and_wait(engine: &mut Engine, r: &mut RoutineContext, frames: i32) {
    engine.state.frame_counter = ((frames) as u8);
    wait_for_frame_counter(engine, r);
}
