//! Python bindings (PyO3) for [`lotw::env::Env`] — the headless agent environment.
//!
//! ```python
//! import numpy as np, lotw_env
//! env = lotw_env.Lotw("rom/lotw.nes")
//! frame, ram, done = env.step(lotw_env.RIGHT)   # frame/ram are bytes
//! img = np.frombuffer(frame, np.uint8).reshape(lotw_env.FRAME_H, lotw_env.FRAME_W, 3)
//! env.reset_replay(bytes([lotw_env.START]) * 12)  # load a checkpoint (input prefix)
//! ```
//!
//! The policy observes only the **frame** (pixels). `ram()` / `state()` are the
//! *privileged* training signal (rewards, success checks) — keep them out of the
//! agent's observation. Save-states are replay-based: a checkpoint is an input
//! prefix and `reset_replay` reboots + fast-forwards (see `lotw::env`).

use lotw::env::{Env, FRAME_BYTES, FRAME_H, FRAME_W};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

/// A booted game you drive one frame at a time. Controller byte = hardware order
/// (bit0=A bit1=B bit2=Select bit3=Start bit4=Up bit5=Down bit6=Left bit7=Right).
// `unsendable`: Env wraps a corosensei coroutine + UnsafeCell (raw pointers), so
// it isn't Send. Each env is driven from one thread; parallel RL uses separate
// processes (each with its own env), so this is the right constraint.
#[pyclass(name = "Lotw", unsendable)]
struct PyEnv {
    inner: Env,
}

#[pymethods]
impl PyEnv {
    #[new]
    #[pyo3(signature = (rom_path, init_ram = true))]
    fn new(rom_path: &str, init_ram: bool) -> PyResult<Self> {
        Ok(PyEnv { inner: Env::from_path(rom_path, init_ram).map_err(PyRuntimeError::new_err)? })
    }

    /// Reboot to a fresh game (power-on).
    fn reset(&mut self) -> PyResult<()> {
        self.inner.reset().map_err(PyRuntimeError::new_err)
    }

    /// Reboot then fast-forward by replaying `inputs` (one controller byte per
    /// frame) without rendering — the cheap "load checkpoint k".
    fn reset_replay(&mut self, inputs: &[u8]) -> PyResult<()> {
        self.inner.reset_replay(inputs).map_err(PyRuntimeError::new_err)
    }

    /// Advance one frame (no render). Returns `done`.
    fn advance(&mut self, action: u8) -> bool {
        self.inner.advance(action)
    }

    /// Step one frame: returns `(frame_bytes, ram_bytes, done)`. `frame` is
    /// FRAME_H×FRAME_W×3 RGB row-major; `ram` is the 2 KiB work RAM.
    fn step<'py>(&mut self, py: Python<'py>, action: u8) -> (Bound<'py, PyBytes>, Bound<'py, PyBytes>, bool) {
        let done = self.inner.advance(action);
        let frame = self.inner.render();
        let frame_b = PyBytes::new(py, &frame);
        let ram_b = PyBytes::new(py, self.inner.ram());
        (frame_b, ram_b, done)
    }

    /// Render the current frame to bytes (FRAME_H×FRAME_W×3 RGB).
    fn render<'py>(&mut self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.inner.render())
    }

    /// The 2 KiB work RAM (privileged).
    fn ram<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.inner.ram())
    }

    /// Named game-state fields (privileged) for reward / success predicates.
    fn state<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let s = self.inner.state();
        let d = PyDict::new(py);
        d.set_item("character_index", s.character_index)?;
        d.set_item("map_screen_x", s.map_screen_x)?;
        d.set_item("map_screen_y", s.map_screen_y)?;
        d.set_item("player_x_tile", s.player_x_tile)?;
        d.set_item("player_x_fine", s.player_x_fine)?;
        d.set_item("player_y", s.player_y)?;
        d.set_item("scroll_pixel_x", s.scroll_pixel_x)?;
        Ok(d)
    }

    #[getter]
    fn frame_count(&self) -> usize {
        self.inner.frame_count()
    }

    #[getter]
    fn done(&self) -> bool {
        self.inner.is_done()
    }
}

#[pymodule]
fn lotw_env(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEnv>()?;
    m.add("FRAME_W", FRAME_W)?;
    m.add("FRAME_H", FRAME_H)?;
    m.add("FRAME_BYTES", FRAME_BYTES)?;
    // Hardware controller bits, for convenience.
    m.add("A", 1u8)?;
    m.add("B", 2u8)?;
    m.add("SELECT", 4u8)?;
    m.add("START", 8u8)?;
    m.add("UP", 16u8)?;
    m.add("DOWN", 32u8)?;
    m.add("LEFT", 64u8)?;
    m.add("RIGHT", 128u8)?;
    Ok(())
}
