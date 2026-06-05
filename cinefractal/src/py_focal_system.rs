//! Python-facing wrapper for the focal (autofocus) system.
//! Decouples the internal Rust representation from the public Python API and
//! mirrors the builder -> closed life cycle used by the animation system:
//! `FocalSystemInterface` is the open, configurable builder; calling
//! [`create_collection_of_points`][FocalSystemInterface.create_collection_of_points]
//! runs the search and returns an immutable, queryable `FocalPointResult`.

use fractal_core::prelude::*;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

use crate::py_fractal_type::PyFractalType;

/// How focal points are ordered when a path through them is requested.
///
/// A pure tag, mirroring `FractalType`. Pick the construction strategy via the
/// static methods.
///
/// - `short_path`: a reasonably short tour (greedy TSP approximation), starting
///   at the point closest to the supplied start point. Good for smooth camera
///   travel.
/// - `descending_value`: most-interesting point first, down to the least
///   interesting. Good for a "highlight reel" ordering. The start point is
///   ignored for ordering but still prepended to the resulting path.
///
/// Example:
/// ```python
/// from cinefractal import PathGenerationCriterion
///
/// crit = PathGenerationCriterion.short_path()
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "PathGenerationCriterion", from_py_object)]
#[derive(Clone, Copy)]
pub struct PyPathGenerationCriterion {
    inner: PathGenerationCriterion,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyPathGenerationCriterion {
    /// Greedy short tour starting from the point nearest the start point.
    #[staticmethod]
    pub fn short_path() -> Self {
        Self {
            inner: PathGenerationCriterion::ShortPath,
        }
    }

    /// Points ordered from most to least interesting.
    #[staticmethod]
    pub fn descending_value() -> Self {
        Self {
            inner: PathGenerationCriterion::DescendingValuePath,
        }
    }

    fn __repr__(&self) -> String {
        let name = match self.inner {
            PathGenerationCriterion::ShortPath => "short_path",
            PathGenerationCriterion::DescendingValuePath => "descending_value",
        };
        format!("PathGenerationCriterion.{name}()")
    }
}

/// Builder for an autofocus search. The fractal, search region and evaluation
/// scale are configured through the micro setters; calling
/// [`create_collection_of_points`][Self.create_collection_of_points] runs the
/// (parallel) search and returns an immutable `FocalPointResult`.
///
/// Mirrors `AnimationRecorder`: this object stays usable after producing a
/// result, so you can tweak parameters and search again, or reuse it as a
/// template for several searches.
///
/// The constructor seeds the same defaults as an unconfigured
/// `IterationFieldInterface`, so an autofocus run lines up with the field you
/// intend to render.
///
/// Example:
/// ```python
/// from cinefractal import FocalSystemInterface, FractalType
///
/// focus = FocalSystemInterface(max_iterations=1000)
/// focus.set_fractal_type(FractalType.mandelbrot())
/// focus.set_center_point(-0.75, 0.0)
/// focus.set_extension(1.5)        # search radius
/// focus.set_evaluation_extension(1e-3)   # the render extension the points are judged at
///
/// result = focus.create_collection_of_points(50)
/// points = result.get_focal_points_with_evaluation()   # list of (re, im, score)
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "FocalSystemInterface")]
pub struct PyFocalSystemInterface {
    inner: FocalSystemInterface,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyFocalSystemInterface {
    /// Creates a new focal-system builder with default parameters:
    /// Mandelbrot (exponent 2), center 0, search extension 1.5,
    /// evaluation extension 1.5, no inner search radius, 1000 iterations.
    #[new]
    #[pyo3(signature = (
        center_point_real = 0.0,
        center_point_imag = 0.0,
        extension = 1.5,
        extension_min = 0.0,
        evaluation_extension = 1.5,
        max_iterations = 1000,
    ))]
    pub fn new(
        center_point_real: f64,
        center_point_imag: f64,
        extension: f64,
        extension_min: f64,
        evaluation_extension: f64,
        max_iterations: u16,
    ) -> PyResult<Self> {
        let mut inner = FocalSystemInterface::default();
        inner.set_center_point(center_point_real, center_point_imag);
        inner
            .set_extension(extension)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        inner
            .set_extension_min(extension_min)
            .map_err(pyo3::exceptions::PyValueError::new_err)?;
        inner.set_evaluation_extension(evaluation_extension);
        inner.set_maximum_iterations(max_iterations);
        Ok(Self { inner })
    }

    /// Sets the fractal type to search in (mandelbrot, julia, ...).
    #[pyo3(signature = (fractal_type))]
    pub fn set_fractal_type(&mut self, fractal_type: &PyFractalType) {
        self.inner.set_fractal_type(fractal_type.into());
    }

    /// Sets the iteration exponent. Must be at least 2. The exponent is the exponent used in 
    /// the escape time iteration formula.
    #[pyo3(signature = (exponent = 2))]
    pub fn set_fractal_exponent(&mut self, exponent: u32) -> PyResult<()> {
        if exponent < 2 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "exponent must be at least 2",
            ));
        }
        self.inner.set_fractal_exponent(exponent);
        Ok(())
    }

    /// Sets the Julia seed (start) value. Only relevant for the Julia set. The Julia fractal is the only fractal, that 
    /// has a start value as an additional parameter. One has to add real and imaginary parts.
    #[pyo3(signature = (real = 0.0, imag = 0.0))]
    pub fn set_fractal_seed_value(&mut self, real: f64, imag: f64) {
        self.inner.set_fractal_seed_value(real, imag);
    }

    /// Sets the maximum number of iterations used when sampling candidates.
    /// Higher values mean more computation time but also more detailed fractal borders.
    #[pyo3(signature = (maximum_iterations = 1000))]
    pub fn set_maximum_iterations(&mut self, maximum_iterations: u16) {
        self.inner.set_maximum_iterations(maximum_iterations);
    }

    /// Sets the center of the circular search region in the complex number pane.
    #[pyo3(signature = (real = 0.0, imag = 0.0))]
    pub fn set_center_point(&mut self, real: f64, imag: f64) {
        self.inner.set_center_point(real, imag);
    }

    /// Sets the outer search radius (and therefore the maximum distance of a
    /// candidate from the center). Must stay strictly greater than the current
    /// inner radius (`extension_min`).
    #[pyo3(signature = (extension = 1.5))]
    pub fn set_extension(&mut self, extension: f64) -> PyResult<()> {
        self.inner
            .set_extension(extension)
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    /// Sets the inner search radius. Candidates are scattered in the annulus
    /// between `extension_min` and `extension`. Defaults to 0 (full disc).
    /// Must satisfy `0 <= extension_min < extension`.
    #[pyo3(signature = (extension_min = 0.0))]
    pub fn set_extension_min(&mut self, extension_min: f64) -> PyResult<()> {
        self.inner
            .set_extension_min(extension_min)
            .map_err(pyo3::exceptions::PyValueError::new_err)
    }

    /// Sets the evaluation extension: the rendering extension (the view's
    /// half-width in the complex plane) at which a candidate's local detail is
    /// measured. It should match the extension you later render the points at --
    /// note that a smaller value means a deeper zoom. Independent of the search
    /// extension (`set_extension`), which is usually larger.
    #[pyo3(signature = (evaluation_extension = 1.5))]
    pub fn set_evaluation_extension(&mut self, evaluation_extension: f64) {
        self.inner.set_evaluation_extension(evaluation_extension);
    }

    /// Runs the search and returns the best `num_of_points` candidates as an
    /// immutable `FocalPointResult`. The heavy, parallel work releases the GIL.
    pub fn create_collection_of_points(&self, py: Python<'_>, num_of_points: usize) -> PyFocalPointResult {
        let inner = py.detach(|| self.inner.create_collection_of_points(num_of_points));
        PyFocalPointResult { inner }
    }
}

/// Immutable result of an autofocus search, produced by
/// `FocalSystemInterface.create_collection_of_points()`. Holds the best focal
/// points and can hand them back either unordered (with their scores) or as an
/// ordered path ready to drive an `AnimationRecorder`.
#[gen_stub_pyclass]
#[pyclass(name = "FocalPointResult")]
pub struct PyFocalPointResult {
    inner: FocalPointResult,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyFocalPointResult {
    /// All focal points with their evaluation, unordered.
    /// Returns a list of `(real, imag, score)` tuples.
    pub fn get_focal_points_with_evaluation(&self) -> Vec<(f64, f64, f64)> {
        self.inner.get_focal_points_with_evaluation()
    }

    /// An ordered path through the focal points, starting at the given point.
    /// Returns a list of `(real, imag)` tuples; the start point is the first
    /// element.
    #[pyo3(signature = (criterion, start_real = 0.0, start_imag = 0.0))]
    pub fn get_path_from_start_point(
        &self,
        criterion: &PyPathGenerationCriterion,
        start_real: f64,
        start_imag: f64,
    ) -> Vec<(f64, f64)> {
        self.inner
            .get_path_from_start_point(criterion.inner, start_real, start_imag)
    }

    /// Like `get_path_from_start_point`, but each element is additionally
    /// annotated with the cumulative distance travelled along the path.
    /// Returns a list of `(real, imag, cumulative_distance)` tuples.
    /// Useful for timing an animation (e.g. constant-speed camera travel).
    #[pyo3(signature = (criterion, start_real = 0.0, start_imag = 0.0))]
    pub fn get_path_from_start_point_with_distance(
        &self,
        criterion: &PyPathGenerationCriterion,
        start_real: f64,
        start_imag: f64,
    ) -> Vec<(f64, f64, f64)> {
        self.inner
            .get_path_from_start_point_with_distance(criterion.inner, start_real, start_imag)
    }
}
