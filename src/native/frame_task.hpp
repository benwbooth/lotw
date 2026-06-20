#ifndef LOTW_NATIVE_FRAME_TASK_HPP
#define LOTW_NATIVE_FRAME_TASK_HPP

#include <coroutine>
#include <cstdint>
#include <exception>
#include <utility>

namespace lotw::native {

struct FrameInput {
    std::uint8_t buttons = 0;
    bool frame_elapsed = false;
};

struct Wait {
    enum class Kind {
        ButtonsReleased,
        ButtonPressed,
        NextFrame,
    };

    Kind kind;
    std::uint8_t mask;

    static constexpr Wait buttons_released(std::uint8_t mask = 0xFF) noexcept
    {
        return {Kind::ButtonsReleased, mask};
    }

    static constexpr Wait button_pressed(std::uint8_t mask) noexcept
    {
        return {Kind::ButtonPressed, mask};
    }

    static constexpr Wait next_frame() noexcept
    {
        return {Kind::NextFrame, 0};
    }

    constexpr bool satisfied(FrameInput in) const noexcept
    {
        switch (kind) {
        case Kind::ButtonsReleased:
            return (in.buttons & mask) == 0;
        case Kind::ButtonPressed:
            return (in.buttons & mask) != 0;
        case Kind::NextFrame:
            return in.frame_elapsed;
        }
        return false;
    }
};

class FrameTask {
public:
    struct promise_type {
        Wait wait = Wait::buttons_released(0);
        bool has_wait = false;

        FrameTask get_return_object() noexcept
        {
            return FrameTask(std::coroutine_handle<promise_type>::from_promise(*this));
        }

        std::suspend_always initial_suspend() noexcept { return {}; }
        std::suspend_always final_suspend() noexcept { return {}; }

        std::suspend_always yield_value(Wait w) noexcept
        {
            wait = w;
            has_wait = true;
            return {};
        }

        void return_void() noexcept {}
        void unhandled_exception() { std::terminate(); }
    };

    FrameTask() = default;
    explicit FrameTask(std::coroutine_handle<promise_type> h) noexcept : h_(h) {}

    FrameTask(const FrameTask&) = delete;
    FrameTask& operator=(const FrameTask&) = delete;

    FrameTask(FrameTask&& other) noexcept : h_(std::exchange(other.h_, {})), started_(other.started_) {}

    FrameTask& operator=(FrameTask&& other) noexcept
    {
        if (this != &other) {
            destroy();
            h_ = std::exchange(other.h_, {});
            started_ = other.started_;
        }
        return *this;
    }

    ~FrameTask() { destroy(); }

    bool done() const noexcept
    {
        return !h_ || h_.done();
    }

    bool step(FrameInput input)
    {
        if (done())
            return true;

        if (!started_) {
            started_ = true;
            h_.resume();
            if (done())
                return true;
        }

        auto& promise = h_.promise();
        if (!promise.has_wait)
            return done();
        if (!promise.wait.satisfied(input))
            return false;

        promise.has_wait = false;
        h_.resume();
        return done();
    }

private:
    void destroy() noexcept
    {
        if (h_) {
            h_.destroy();
            h_ = {};
        }
    }

    std::coroutine_handle<promise_type> h_;
    bool started_ = false;
};

} // namespace lotw::native

#endif /* LOTW_NATIVE_FRAME_TASK_HPP */
