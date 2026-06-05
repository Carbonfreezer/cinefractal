//! Python-facing wrapper for IterationFieldInterface.

use crate::py_color_system::PyColorSystem;
use crate::py_fractal_type::PyFractalType;
use fractal_core::prelude::*;
use numpy::{IntoPyArray, PyArray2, PyArray3};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

// Locally disabled  for CI.
// use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// Main interface for computing fractals. This is the entry point for generating images and
/// iteration fields. A base is setup with default parameters, that can be adjusted by individual methods.
///
/// Example:
/// ```python
/// from cinefractal import IterationFieldInterface, FractalType, ColorSystem
///
/// ifi = IterationFieldInterface()
/// ifi.set_numpy_extension(768, 1024)
/// ifi.set_fractal_type(FractalType.mandelbrot())
/// ifi.set_fractal_exponent(2)
/// ifi.set_colorization_information(ColorSystem.viridis(), True)
/// color = ifi.get_color_field()   # numpy array (rows, cols, 3), dtype=uint8
/// ```
// Locally disabled  for CI.
// #[gen_stub_pyclass]
#[pyclass(name = "IterationFieldInterface")]
pub struct PyIterationFieldInterface {
    pub(crate) inner: IterationFieldInterface,
}

// Locally disabled  for CI.
// #[gen_stub_pymethods]
#[pymethods]
impl PyIterationFieldInterface {
    /// Creates a new instance with default values:
    /// 1024×768, Mandelbrot (exponent 2), Viridis, discrete coloring, no log scaling.
    #[new]
    pub fn new() -> Self {
        Self {
            inner: IterationFieldInterface::default(),
        }
    }

    /// Sets the output size in pixels
    ///
    /// - rows = height
    /// - columns = width
    #[pyo3(signature=(rows, columns))]
    pub fn set_numpy_extension(&mut self, rows: usize, columns: usize) {
        self.inner.set_numpy_extension(rows, columns);
    }

    /// Sets the fractal type to render.
    ///
    /// - fractal_type: a `FractalType` (mandelbrot, julia, ...)
    #[pyo3(signature = (fractal_type))]
    pub fn set_fractal_type(&mut self, fractal_type: &PyFractalType) {
        self.inner.set_fractal_type(fractal_type.into());
    }

    /// Sets the iteration exponent. Must be at least 2. The exponent is the exponent 
    /// used in the escape time iteration formula.
    #[pyo3(signature = (exponent = 2))]
    pub fn set_fractal_exponent(&mut self, exponent: u32) -> PyResult<()> {
        if exponent < 2 {
            return Err(PyValueError::new_err("exponent must be at least 2"));
        }
        self.inner.set_fractal_exponent(exponent);
        Ok(())
    }

    /// Sets the Julia seed (start) value. The Julia fractal is the only fractal, that 
    /// has a start value as an additional parameter. One has to add real and imaginary parts.
    #[pyo3(signature = (real = 0.0, imag = 0.0))]
    pub fn set_fractal_seed_value(&mut self, real: f64, imag: f64) {
        self.inner.set_fractal_seed_value(real, imag);
    }

    /// Sets the maximum number of iterations used in the escape iteration.
    /// Higher values mean more computation time but also more detailed fractal borders.
    #[pyo3(signature=(maximum_iterations = 1000))]
    pub fn set_maximum_iterations(&mut self, maximum_iterations: u16) {
        self.inner.set_maximum_iterations(maximum_iterations);
    }

    /// Sets the center point in the complex plane. This is the point in the complex 
    /// number pane, that will be in the center of the image.
    #[pyo3(signature=(real = 0.0, imag = 0.0))]
    pub fn set_center_point(&mut self, real: f64, imag: f64) {
        self.inner.set_center_point(real, imag);
    }

    /// Sets the extension (inverse zoom factor) in the complex plane.
    #[pyo3(signature=(extension = 1.5))]
    pub fn set_extension(&mut self, extension: f64) {
        self.inner.set_extension(extension);
    }

    /// Sets the color scheme and discrete/continuous mode.
    ///
    /// - `color_system`: the desired color scheme
    /// - `uses_discrete_for_color`: True = discrete iteration counts, False = continuous
    ///
    /// The logarithmic color strength is set separately with
    /// `set_colorization_log_strength`.
    #[pyo3(signature = (color_system, uses_discrete_for_color = false))]
    pub fn set_colorization_information(
        &mut self,
        color_system: &PyColorSystem,
        uses_discrete_for_color: bool,
    ) {
        self.inner
            .set_colorization_information(color_system.into(), uses_discrete_for_color);
    }

    /// Sets the strength of the logarithmic color scaling (0.0 = off).
    ///
    /// Note: when an animation is applied to this interface, this value is
    /// driven by the animation's log strength track and overwritten each frame.
    #[pyo3(signature = (log_strength = 0.0))]
    pub fn set_colorization_log_strength(&mut self, log_strength: f32) {
        self.inner.set_colorization_log_strength(log_strength);
    }

    /// Returns the discrete iteration field (shape: rows × cols, dtype: uint16).
    pub fn get_discrete_iteration_field<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray2<u16>> {
        let raw = py.detach(|| self.inner.get_discrete_iteration_field());
        raw.into_pyarray(py)
    }

    /// Returns the continuous iteration field (shape: rows × cols, dtype: float32).
    pub fn get_continuous_iteration_field<'py>(
        &self,
        py: Python<'py>,
    ) -> Bound<'py, PyArray2<f32>> {
        let raw = py.detach(|| self.inner.get_continuous_iteration_field());
        raw.into_pyarray(py)
    }

    /// Returns the RGB color image (shape: rows × cols × 3, dtype: uint8).
    pub fn get_color_field<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray3<u8>> {
        let raw = py.detach(|| self.inner.get_color_field());
        raw.into_pyarray(py)
    }
}
