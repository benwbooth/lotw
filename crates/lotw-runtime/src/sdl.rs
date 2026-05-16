use lotw_port::video::Frame;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

const SDL_INIT_VIDEO: u32 = 0x0000_0020;
const SDL_WINDOWPOS_CENTERED: c_int = 0x2fff_0000;
const SDL_WINDOW_SHOWN: u32 = 0x0000_0004;
const SDL_RENDERER_SOFTWARE: u32 = 0x0000_0001;
const SDL_RENDERER_ACCELERATED: u32 = 0x0000_0002;
const SDL_RENDERER_PRESENTVSYNC: u32 = 0x0000_0004;
const SDL_TEXTUREACCESS_STREAMING: c_int = 1;
const SDL_PIXELFORMAT_RGB24: u32 = 0x1710_1803;
const SDL_QUIT: u32 = 0x100;

#[repr(C)]
struct SdlWindow {
    _private: [u8; 0],
}

#[repr(C)]
struct SdlRenderer {
    _private: [u8; 0],
}

#[repr(C)]
struct SdlTexture {
    _private: [u8; 0],
}

#[repr(C)]
struct SdlRect {
    x: c_int,
    y: c_int,
    w: c_int,
    h: c_int,
}

#[link(name = "SDL2")]
unsafe extern "C" {
    fn SDL_Init(flags: u32) -> c_int;
    fn SDL_Quit();
    fn SDL_GetError() -> *const c_char;
    fn SDL_CreateWindow(
        title: *const c_char,
        x: c_int,
        y: c_int,
        w: c_int,
        h: c_int,
        flags: u32,
    ) -> *mut SdlWindow;
    fn SDL_DestroyWindow(window: *mut SdlWindow);
    fn SDL_CreateRenderer(window: *mut SdlWindow, index: c_int, flags: u32) -> *mut SdlRenderer;
    fn SDL_DestroyRenderer(renderer: *mut SdlRenderer);
    fn SDL_CreateTexture(
        renderer: *mut SdlRenderer,
        format: u32,
        access: c_int,
        w: c_int,
        h: c_int,
    ) -> *mut SdlTexture;
    fn SDL_DestroyTexture(texture: *mut SdlTexture);
    fn SDL_UpdateTexture(
        texture: *mut SdlTexture,
        rect: *const SdlRect,
        pixels: *const c_void,
        pitch: c_int,
    ) -> c_int;
    fn SDL_RenderClear(renderer: *mut SdlRenderer) -> c_int;
    fn SDL_RenderCopy(
        renderer: *mut SdlRenderer,
        texture: *mut SdlTexture,
        srcrect: *const SdlRect,
        dstrect: *const SdlRect,
    ) -> c_int;
    fn SDL_RenderPresent(renderer: *mut SdlRenderer);
    fn SDL_PollEvent(event: *mut c_void) -> c_int;
    fn SDL_Delay(ms: u32);
}

pub fn run(frame: &Frame, scale: i32, frames: usize) -> Result<(), Box<dyn std::error::Error>> {
    let width = i32::try_from(frame.width)?;
    let height = i32::try_from(frame.height)?;
    let window_width = width
        .checked_mul(scale)
        .ok_or("scaled window width overflow")?;
    let window_height = height
        .checked_mul(scale)
        .ok_or("scaled window height overflow")?;
    let title = b"Legacy of the Wizard Rust Port\0";

    let sdl = SdlContext::init()?;
    let window = Window::create(
        title.as_ptr().cast(),
        window_width,
        window_height,
        SDL_WINDOW_SHOWN,
    )?;
    let renderer = Renderer::create(&window)?;
    let texture = Texture::create(&renderer, width, height)?;
    texture.update(frame)?;

    let mut frame_index = 0usize;
    loop {
        if poll_quit() {
            break;
        }
        renderer.present(&texture, window_width, window_height)?;
        frame_index += 1;
        if frames > 0 && frame_index >= frames {
            break;
        }
        unsafe {
            SDL_Delay(16);
        }
    }

    drop(texture);
    drop(renderer);
    drop(window);
    drop(sdl);
    Ok(())
}

struct SdlContext;

impl SdlContext {
    fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let rc = unsafe { SDL_Init(SDL_INIT_VIDEO) };
        if rc != 0 {
            return Err(sdl_error("SDL_Init").into());
        }
        Ok(Self)
    }
}

impl Drop for SdlContext {
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}

struct Window(*mut SdlWindow);

impl Window {
    fn create(
        title: *const c_char,
        width: c_int,
        height: c_int,
        flags: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ptr = unsafe {
            SDL_CreateWindow(
                title,
                SDL_WINDOWPOS_CENTERED,
                SDL_WINDOWPOS_CENTERED,
                width,
                height,
                flags,
            )
        };
        if ptr.is_null() {
            return Err(sdl_error("SDL_CreateWindow").into());
        }
        Ok(Self(ptr))
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.0);
        }
    }
}

struct Renderer(*mut SdlRenderer);

impl Renderer {
    fn create(window: &Window) -> Result<Self, Box<dyn std::error::Error>> {
        let mut ptr = unsafe {
            SDL_CreateRenderer(
                window.0,
                -1,
                SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC,
            )
        };
        if ptr.is_null() {
            ptr = unsafe { SDL_CreateRenderer(window.0, -1, SDL_RENDERER_SOFTWARE) };
        }
        if ptr.is_null() {
            return Err(sdl_error("SDL_CreateRenderer").into());
        }
        Ok(Self(ptr))
    }

    fn present(
        &self,
        texture: &Texture,
        width: c_int,
        height: c_int,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dst = SdlRect {
            x: 0,
            y: 0,
            w: width,
            h: height,
        };
        let clear_rc = unsafe { SDL_RenderClear(self.0) };
        if clear_rc != 0 {
            return Err(sdl_error("SDL_RenderClear").into());
        }
        let copy_rc = unsafe { SDL_RenderCopy(self.0, texture.0, ptr::null(), &dst) };
        if copy_rc != 0 {
            return Err(sdl_error("SDL_RenderCopy").into());
        }
        unsafe {
            SDL_RenderPresent(self.0);
        }
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyRenderer(self.0);
        }
    }
}

struct Texture(*mut SdlTexture);

impl Texture {
    fn create(
        renderer: &Renderer,
        width: c_int,
        height: c_int,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ptr = unsafe {
            SDL_CreateTexture(
                renderer.0,
                SDL_PIXELFORMAT_RGB24,
                SDL_TEXTUREACCESS_STREAMING,
                width,
                height,
            )
        };
        if ptr.is_null() {
            return Err(sdl_error("SDL_CreateTexture").into());
        }
        Ok(Self(ptr))
    }

    fn update(&self, frame: &Frame) -> Result<(), Box<dyn std::error::Error>> {
        let pitch = i32::try_from(frame.width.checked_mul(3).ok_or("pitch overflow")?)?;
        let rc =
            unsafe { SDL_UpdateTexture(self.0, ptr::null(), frame.rgb.as_ptr().cast(), pitch) };
        if rc != 0 {
            return Err(sdl_error("SDL_UpdateTexture").into());
        }
        Ok(())
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyTexture(self.0);
        }
    }
}

fn poll_quit() -> bool {
    let mut event = [0u8; 128];
    loop {
        let polled = unsafe { SDL_PollEvent(event.as_mut_ptr().cast()) };
        if polled == 0 {
            return false;
        }
        let event_type = u32::from_ne_bytes(event[0..4].try_into().expect("event type bytes"));
        if event_type == SDL_QUIT {
            return true;
        }
    }
}

fn sdl_error(prefix: &str) -> String {
    let ptr = unsafe { SDL_GetError() };
    if ptr.is_null() {
        return format!("{prefix} failed");
    }
    let message = unsafe { CStr::from_ptr(ptr) }.to_string_lossy();
    format!("{prefix} failed: {message}")
}
