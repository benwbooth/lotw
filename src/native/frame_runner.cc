#include "native/frame_runner_c.h"

#include "ppu.h"

#include <csetjmp>
#include <condition_variable>
#include <mutex>
#include <new>
#include <thread>

namespace {

thread_local bool stop_requested = false;
thread_local std::jmp_buf *stop_jump = nullptr;

} // namespace

namespace lotw::native {

bool frame_runner_stop_requested() noexcept
{
    return stop_requested;
}

[[noreturn]] void exit_frame_runner_thread()
{
    if (stop_jump)
        std::longjmp(*stop_jump, 1);
    __builtin_unreachable();
}

} // namespace lotw::native

struct LotwFrameRunner {
    enum class State {
        Created,
        Running,
        Waiting,
        Done,
    };

    explicit LotwFrameRunner(PortFn entry_) : entry(entry_) {}

    PortFn entry = nullptr;
    Regs regs{};
    std::thread game_thread;
    mutable std::mutex mutex;
    std::condition_variable cv;
    State state = State::Created;
    bool stop = false;
    void (*previous_wait)(Regs *) = nullptr;

    void thread_main()
    {
        std::jmp_buf jump;
        active_runner = this;
        stop_requested = false;
        stop_jump = &jump;
        if (setjmp(jump) == 0)
            entry(&regs);
        {
            std::lock_guard<std::mutex> lock(mutex);
            state = State::Done;
        }
        cv.notify_all();
        active_runner = nullptr;
        stop_requested = false;
        stop_jump = nullptr;
    }

    void frame_wait(Regs *r)
    {
        Regs save = *r;
        std::unique_lock<std::mutex> lock(mutex);
        state = State::Waiting;
        cv.notify_all();
        cv.wait(lock, [this] { return state == State::Running || stop; });
        if (stop) {
            state = State::Done;
            cv.notify_all();
            stop_requested = true;
            return;
        }
        *r = save;
    }

    bool wait_until_parked(std::unique_lock<std::mutex> &lock)
    {
        cv.wait(lock, [this] { return state == State::Waiting || state == State::Done; });
        return state == State::Waiting;
    }

    static thread_local LotwFrameRunner *active_runner;
};

thread_local LotwFrameRunner *LotwFrameRunner::active_runner = nullptr;

static void frame_wait_trampoline(Regs *r)
{
    if (LotwFrameRunner::active_runner)
        LotwFrameRunner::active_runner->frame_wait(r);
}

extern "C" LotwFrameRunner *lotw_frame_runner_create(PortFn entry)
{
    if (!entry)
        return nullptr;
    return new (std::nothrow) LotwFrameRunner(entry);
}

extern "C" void lotw_frame_runner_destroy(LotwFrameRunner *runner)
{
    if (!runner)
        return;

    {
        std::lock_guard<std::mutex> lock(runner->mutex);
        runner->stop = true;
        if (runner->state == LotwFrameRunner::State::Waiting)
            runner->state = LotwFrameRunner::State::Running;
    }
    runner->cv.notify_all();

    if (runner->game_thread.joinable())
        runner->game_thread.join();

    if (nes_vblank_wait == frame_wait_trampoline)
        nes_vblank_wait = runner->previous_wait;

    delete runner;
}

extern "C" int lotw_frame_runner_start(LotwFrameRunner *runner)
{
    if (!runner)
        return 0;

    std::unique_lock<std::mutex> lock(runner->mutex);
    if (runner->state == LotwFrameRunner::State::Waiting)
        return 1;
    if (runner->state != LotwFrameRunner::State::Created)
        return 0;

    runner->previous_wait = nes_vblank_wait;
    nes_vblank_wait = frame_wait_trampoline;
    runner->state = LotwFrameRunner::State::Running;
    runner->game_thread = std::thread([runner] { runner->thread_main(); });
    return runner->wait_until_parked(lock) ? 1 : 0;
}

extern "C" int lotw_frame_runner_resume_until_wait(LotwFrameRunner *runner)
{
    if (!runner)
        return 0;

    std::unique_lock<std::mutex> lock(runner->mutex);
    if (runner->state == LotwFrameRunner::State::Done)
        return 0;
    if (runner->state != LotwFrameRunner::State::Waiting)
        return 0;

    runner->state = LotwFrameRunner::State::Running;
    lock.unlock();
    runner->cv.notify_all();

    lock.lock();
    return runner->wait_until_parked(lock) ? 1 : 0;
}

extern "C" int lotw_frame_runner_done(const LotwFrameRunner *runner)
{
    if (!runner)
        return 1;
    std::lock_guard<std::mutex> lock(runner->mutex);
    return runner->state == LotwFrameRunner::State::Done ? 1 : 0;
}

extern "C" Regs *lotw_frame_runner_regs(LotwFrameRunner *runner)
{
    return runner ? &runner->regs : nullptr;
}
