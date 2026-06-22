use std::{
    cell::RefCell,
    cell::UnsafeCell,
    panic::{self, AssertUnwindSafe},
    sync::{Arc, Condvar, Mutex, Once},
    thread::{self, JoinHandle},
};

use crate::{Engine, RoutineContext, engine::RoutineFn};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RunnerState {
    Created,
    Running,
    Waiting,
    Done,
}

#[derive(Debug)]
struct SyncState {
    state: RunnerState,
    stop: bool,
}

#[derive(Debug)]
struct FrameRunnerStop;

static STOP_PANIC_HOOK: Once = Once::new();

fn install_stop_panic_hook() {
    STOP_PANIC_HOOK.call_once(|| {
        let previous = panic::take_hook();
        panic::set_hook(Box::new(move |info| {
            if info.payload().is::<FrameRunnerStop>() {
                return;
            }
            previous(info);
        }));
    });
}

#[derive(Clone)]
struct WaitSync {
    inner: Arc<(Mutex<SyncState>, Condvar)>,
}

thread_local! {
    static ACTIVE_WAIT: RefCell<Option<WaitSync>> = const { RefCell::new(None) };
    static STOP_REQUESTED: RefCell<bool> = const { RefCell::new(false) };
}

pub fn frame_runner_stop_requested() -> bool {
    STOP_REQUESTED.with(|stop| *stop.borrow())
}

pub fn frame_wait(_engine: &mut Engine, _r: &mut RoutineContext) {
    ACTIVE_WAIT.with(|active| {
        let Some(sync) = active.borrow().clone() else {
            return;
        };
        let (mutex, cv) = &*sync.inner;
        let mut state = mutex.lock().expect("frame runner mutex poisoned");
        state.state = RunnerState::Waiting;
        cv.notify_all();
        state = cv
            .wait_while(state, |state| {
                state.state != RunnerState::Running && !state.stop
            })
            .expect("frame runner mutex poisoned");
        let stop = state.stop;
        if stop {
            state.state = RunnerState::Done;
            cv.notify_all();
        }
        drop(state);
        if stop {
            STOP_REQUESTED.with(|stop| *stop.borrow_mut() = true);
            panic::panic_any(FrameRunnerStop);
        }
    });
}

pub struct FrameRunner {
    entry: RoutineFn,
    engine: Box<UnsafeCell<Engine>>,
    regs: Box<UnsafeCell<RoutineContext>>,
    sync: WaitSync,
    game_thread: Option<JoinHandle<()>>,
}

unsafe impl Send for FrameRunner {}
unsafe impl Sync for FrameRunner {}

impl FrameRunner {
    pub fn new(engine: Engine, entry: RoutineFn) -> Self {
        Self {
            entry,
            engine: Box::new(UnsafeCell::new(engine)),
            regs: Box::new(UnsafeCell::new(RoutineContext::default())),
            sync: WaitSync {
                inner: Arc::new((
                    Mutex::new(SyncState {
                        state: RunnerState::Created,
                        stop: false,
                    }),
                    Condvar::new(),
                )),
            },
            game_thread: None,
        }
    }

    pub fn start(&mut self) -> bool {
        install_stop_panic_hook();
        {
            let (mutex, _) = &*self.sync.inner;
            let mut state = mutex.lock().expect("frame runner mutex poisoned");
            if state.state == RunnerState::Waiting {
                return true;
            }
            if state.state != RunnerState::Created {
                return false;
            }
            state.state = RunnerState::Running;
        }

        let entry = self.entry;
        let engine = self.engine.get() as usize;
        let regs = self.regs.get() as usize;
        let sync = self.sync.clone();
        self.game_thread = Some(thread::spawn(move || {
            ACTIVE_WAIT.with(|active| *active.borrow_mut() = Some(sync.clone()));
            STOP_REQUESTED.with(|stop| *stop.borrow_mut() = false);
            // Safety: the runner only exposes engine/regs to the control thread
            // while this thread is parked in frame_wait or after it has exited.
            let result = panic::catch_unwind(AssertUnwindSafe(|| unsafe {
                entry(
                    &mut *(engine as *mut Engine),
                    &mut *(regs as *mut RoutineContext),
                );
            }));
            if let Err(payload) = result {
                if !payload.is::<FrameRunnerStop>() {
                    panic::resume_unwind(payload);
                }
            }
            let (mutex, cv) = &*sync.inner;
            let mut state = mutex.lock().expect("frame runner mutex poisoned");
            state.state = RunnerState::Done;
            cv.notify_all();
            ACTIVE_WAIT.with(|active| *active.borrow_mut() = None);
            STOP_REQUESTED.with(|stop| *stop.borrow_mut() = false);
        }));

        self.wait_until_parked()
    }

    pub fn resume_until_wait(&mut self) -> bool {
        {
            let (mutex, cv) = &*self.sync.inner;
            let mut state = mutex.lock().expect("frame runner mutex poisoned");
            if state.state == RunnerState::Done || state.state != RunnerState::Waiting {
                return false;
            }
            state.state = RunnerState::Running;
            cv.notify_all();
        }
        self.wait_until_parked()
    }

    pub fn done(&self) -> bool {
        let (mutex, _) = &*self.sync.inner;
        mutex.lock().expect("frame runner mutex poisoned").state == RunnerState::Done
    }

    pub fn regs(&self) -> &RoutineContext {
        // Safety: callers only read while the game thread is parked/done.
        unsafe { &*self.regs.get() }
    }

    pub fn regs_mut(&mut self) -> &mut RoutineContext {
        // Safety: callers only mutate while the game thread is parked/done.
        unsafe { &mut *self.regs.get() }
    }

    pub fn engine(&self) -> &Engine {
        // Safety: callers only read while the game thread is parked/done.
        unsafe { &*self.engine.get() }
    }

    pub fn engine_mut(&mut self) -> &mut Engine {
        // Safety: callers only mutate while the game thread is parked/done.
        unsafe { &mut *self.engine.get() }
    }

    pub fn with_engine_regs<R>(
        &mut self,
        f: impl FnOnce(&mut Engine, &mut RoutineContext) -> R,
    ) -> R {
        // Safety: callers only invoke this while the game thread is parked/done.
        unsafe { f(&mut *self.engine.get(), &mut *self.regs.get()) }
    }

    fn wait_until_parked(&self) -> bool {
        let (mutex, cv) = &*self.sync.inner;
        let state = mutex.lock().expect("frame runner mutex poisoned");
        let state = cv
            .wait_while(state, |state| {
                state.state != RunnerState::Waiting && state.state != RunnerState::Done
            })
            .expect("frame runner mutex poisoned");
        state.state == RunnerState::Waiting
    }
}

impl Drop for FrameRunner {
    fn drop(&mut self) {
        {
            let (mutex, cv) = &*self.sync.inner;
            let mut state = mutex.lock().expect("frame runner mutex poisoned");
            state.stop = true;
            if state.state == RunnerState::Waiting {
                state.state = RunnerState::Running;
            }
            cv.notify_all();
        }
        if let Some(handle) = self.game_thread.take() {
            let _ = handle.join();
        }
    }
}

pub fn wait_frame(engine: &mut Engine, r: &mut RoutineContext) {
    frame_wait(engine, r);
}

pub fn wait_for_frame_counter(engine: &mut Engine, r: &mut RoutineContext) {
    while engine.mem(0x36) != 0 {
        wait_frame(engine, r);
        if frame_runner_stop_requested() {
            return;
        }
    }
}

pub fn wait_for_ppu_job_idle(engine: &mut Engine, r: &mut RoutineContext) {
    while engine.mem(0x28) != 0 {
        wait_frame(engine, r);
        if frame_runner_stop_requested() {
            return;
        }
    }
}

pub fn wait_for_countdown_timer(engine: &mut Engine, r: &mut RoutineContext) {
    while engine.mem(0x8c) != 0 {
        wait_frame(engine, r);
        if frame_runner_stop_requested() {
            return;
        }
    }
}

pub fn commit_frame_work(engine: &mut Engine, r: &mut RoutineContext) {
    if engine.mem(0x3d) != 0 {
        engine.set_mem(0x3d, 0);
        crate::game::routine_0082(engine, r);
    } else if engine.mem(0x3c) != 0 {
        engine.set_mem(0x3c, 0);
        crate::game::routine_0092(engine, r);
    } else if engine.mem(0x36) != 0 {
        crate::game::routine_0075(engine, r);
    }
}

pub fn read_buttons(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
    crate::game::read_controllers(engine, r);
    r.value = engine.mem(0x20);
    r.value
}

pub fn redraw_scene_and_read_buttons(engine: &mut Engine, r: &mut RoutineContext) -> i32 {
    engine.set_mem(0x36, 0x01);
    crate::game::routine_0061(engine, r);
    crate::game::routine_0063(engine, r);
    crate::game::routine_0062(engine, r);
    commit_frame_work(engine, r);
    wait_for_frame_counter(engine, r);
    read_buttons(engine, r)
}

pub fn set_frame_counter_and_wait(engine: &mut Engine, r: &mut RoutineContext, frames: i32) {
    engine.set_mem(0x36, frames);
    wait_for_frame_counter(engine, r);
}
