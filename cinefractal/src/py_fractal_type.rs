//! Python-facing wrapper for FractalType.
//! Decouples the internal Rust representation from the public Python API.

use fractal_core::prelude::*;
use pyo3::prelude::*;
// Locally disabled  for CI.
// use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// The kind of fractal to render. A pure tag -- the exponent and the Julia seed
/// value are set separately on the `IterationFieldInterface` via
/// `set_fractal_exponent` and `set_fractal_seed_value`. The latter is
/// only relevant for the julia set visualization.
///
/// Supported types:
/// - mandelbrot
/// - julia (uses the seed value)
/// - burning_ship
/// - tricorn
/// - celtic
///
/// Example:
/// ```python
/// from cinefractal import FractalType
///
/// ft = FractalType.julia()
/// ```
// Locally disabled  for CI.
// #[gen_stub_pyclass]
#[pyclass(name = "FractalType", from_py_object)]
#[derive(Clone)]
pub struct PyFractalType {
    pub(crate) inner: FractalType,
}

// Locally disabled  for CI.
// #[gen_stub_pymethods]
#[pymethods]
impl PyFractalType {
    #[staticmethod]
    pub fn mandelbrot() -> Self {
        Self {
            inner: FractalType::Mandelbrot,
        }
    }

    #[staticmethod]
    pub fn julia() -> Self {
        Self {
            inner: FractalType::Julia,
        }
    }

    #[staticmethod]
    pub fn burning_ship() -> Self {
        Self {
            inner: FractalType::BurningShip,
        }
    }

    #[staticmethod]
    pub fn tricorn() -> Self {
        Self {
            inner: FractalType::Tricorn,
        }
    }

    #[staticmethod]
    pub fn celtic() -> Self {
        Self {
            inner: FractalType::Celtic,
        }
    }

    fn __repr__(&self) -> String {
        let name = match self.inner {
            FractalType::Mandelbrot => "mandelbrot",
            FractalType::Julia => "julia",
            FractalType::BurningShip => "burning_ship",
            FractalType::Tricorn => "tricorn",
            FractalType::Celtic => "celtic",
        };
        format!("FractalType.{name}()")
    }
}

impl From<&PyFractalType> for FractalType {
    fn from(py: &PyFractalType) -> Self {
        py.inner
    }
}
