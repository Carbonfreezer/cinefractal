//! Python-facing wrapper for ColorSystem.
//! Decouples the internal Rust representation from the public Python API.

use fractal_core::prelude::*;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// Color scheme used to render the fractal. Supports different base colors.
///
/// - Turbo
/// - Viridis
/// - Magma
/// - Plasma
/// - Inferno
/// - Cool
/// - cyclical : This is a rainbow like color scale with a repetition factor.
///
/// Example:
/// ```python
/// cs = ColorSystem.viridis()
/// cs = ColorSystem.cyclical(repetition_factor=3.0)
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "ColorSystem", from_py_object)]
#[derive(Clone)]
pub struct PyColorSystem {
    pub(crate) inner: ColorSystem,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyColorSystem {
    #[staticmethod]
    pub fn turbo() -> Self {
        Self {
            inner: ColorSystem::Turbo,
        }
    }

    #[staticmethod]
    pub fn viridis() -> Self {
        Self {
            inner: ColorSystem::Viridis,
        }
    }

    #[staticmethod]
    pub fn magma() -> Self {
        Self {
            inner: ColorSystem::Magma,
        }
    }

    #[staticmethod]
    pub fn plasma() -> Self {
        Self {
            inner: ColorSystem::Plasma,
        }
    }

    #[staticmethod]
    pub fn inferno() -> Self {
        Self {
            inner: ColorSystem::Inferno,
        }
    }

    #[staticmethod]
    pub fn cool() -> Self {
        Self {
            inner: ColorSystem::Cool,
        }
    }

    /// Cyclical color scheme with a configurable repetition factor.
    ///
    /// - `repetition_factor`: Repetition factor for cyclic color.
    #[staticmethod]
    #[pyo3(signature=(repetition_factor = 3.0))]
    pub fn cyclical(repetition_factor: f32) -> Self {
        Self {
            inner: ColorSystem::Cyclical { repetition_factor },
        }
    }

    fn __repr__(&self) -> String {
        match self.inner {
            ColorSystem::Turbo => "ColorSystem.turbo()".to_string(),
            ColorSystem::Viridis => "ColorSystem.viridis()".to_string(),
            ColorSystem::Magma => "ColorSystem.magma()".to_string(),
            ColorSystem::Plasma => "ColorSystem.plasma()".to_string(),
            ColorSystem::Inferno => "ColorSystem.inferno()".to_string(),
            ColorSystem::Cool => "ColorSystem.cool()".to_string(),
            ColorSystem::Cyclical { repetition_factor } => {
                format!("ColorSystem.cyclical(repetition_factor={repetition_factor})")
            }
        }
    }
}

impl From<&PyColorSystem> for ColorSystem {
    fn from(py: &PyColorSystem) -> Self {
        py.inner
    }
}
