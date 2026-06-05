//! Python-facing wrapper for the animation system.
//! Decouples the internal Rust representation from the public Python API and
//! mirrors the builder -> closed life cycle of the Rust types.

use crate::py_iteration_field_interface::PyIterationFieldInterface;
use fractal_core::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// Builder for an animation. Key frames are added in this open state; calling
/// [`get_animation_player`][AnimationRecorder.get_animation_player] freezes
/// every track and returns an immutable `AnimationPlayer` that applies the
/// animation to an `IterationFieldInterface`.
///
/// The constructor seeds time 0. Its defaults mirror those of
/// `IterationFieldInterface`, so an unconfigured animation starts on the same
/// frame as an unconfigured field interface.
///
/// Example:
/// ```python
/// from cinefractal import AnimationRecorder, IterationFieldInterface, FractalType, ColorSystem
///
/// anim = AnimationRecorder(exponent=2, log_strength=5.0)
/// anim.set_keyframe_fractal_seed_value(0.0, -0.7, 0.27)
/// anim.set_keyframe_extension(6.0, 1e-4)
/// animation = anim.get_animation_player()
///
/// ifi = IterationFieldInterface()
/// ifi.set_fractal_type(FractalType.julia())
/// ifi.set_colorization_information(ColorSystem.inferno(), False)
///
/// animation.apply_animation(3.0, ifi)
/// color = ifi.get_color_field()
/// ```
#[gen_stub_pyclass]
#[pyclass(name = "AnimationRecorder")]
pub struct PyAnimationRecorder {
    inner: AnimationRecorder,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyAnimationRecorder {
    /// Creates a new animation builder seeded at time 0.
    #[new]
    #[pyo3(signature = (
        exponent = 2,
        julia_start_real = 0.0,
        julia_start_imag = 0.0,
        center_point_real = 0.0,
        center_point_imag = 0.0,
        extension = 1.5,
        max_iterations = 1000,
        log_strength = 0.0,
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        exponent: u32,
        julia_start_real: f64,
        julia_start_imag: f64,
        center_point_real: f64,
        center_point_imag: f64,
        extension: f64,
        max_iterations: u16,
        log_strength: f32,
    ) -> Self {
        let config = AnimationStartConfiguration {
            exponent,
            seed_real: julia_start_real,
            seed_imag: julia_start_imag,
            center_point_real,
            center_point_imag,
            extension,
            max_iterations,
            log_strength,
        };
        Self {
            inner: AnimationRecorder::new(config),
        }
    }

    /// Adds a key frame for the iteration exponent. The exponent must be at
    /// least 2. The exponent is the exponent used in the escape time iteration formula.
    pub fn set_keyframe_exponent(&mut self, time: f32, exponent: u32) -> PyResult<()> {
        self.inner
            .set_keyframe_exponent(time, exponent)
            .map_err(PyValueError::new_err)
    }

    /// Adds a key frame for the fractal value. The Julia fractal is the only fractal, that
    /// has a start value as an additional parameter. One has to add real and imaginary parts.
    pub fn set_keyframe_fractal_seed_value(&mut self, time: f32, real: f64, imag: f64) {
        self.inner.set_keyframe_fractal_seed_value(time, real, imag);
    }

    /// Adds a key frame for the render center point. This is the point in the complex 
    /// number pane, that will be in the center of the image.
    pub fn set_keyframe_render_center_point(&mut self, time: f32, real: f64, imag: f64) {
        self.inner
            .set_keyframe_render_center_point(time, real, imag);
    }

    /// Adds a key frame for the extension (zoom). Interpolated in logarithmic
    /// space, so the extension must be strictly positive.
    pub fn set_keyframe_extension(&mut self, time: f32, extension: f64) -> PyResult<()> {
        self.inner
            .set_keyframe_extension(time, extension)
            .map_err(PyValueError::new_err)
    }

    /// Adds a key frame for the maximum number of iterations used in the escape iteration.
    /// Higher values mean more computation time but also more detailed fractal borders.
    pub fn set_keyframe_max_iterations(&mut self, time: f32, max_iter: u16) {
        self.inner.set_keyframe_max_iterations(time, max_iter);
    }

    /// Adds a key frame for the color log strength. The higher the log strength the more detailed
    /// the regions with low iteration values will be.
    pub fn set_keyframe_log_strength(&mut self, time: f32, log_strength: f32) {
        self.inner.set_keyframe_log_strength(time, log_strength);
    }

    /// Freezes the current state into an immutable, queryable
    /// `AnimationPlayer`. The builder is cloned internally, so it stays
    /// usable afterwards: you can keep adding key frames and get another player,
    /// or reuse it as a template for several players.
    pub fn get_animation_player(&self) -> PyAnimationPlayer {
        PyAnimationPlayer {
            inner: self.inner.clone().into_player(),
        }
    }
}

/// Immutable animation produced by `AnimationRecorder.get_animation_player()`.
/// Applies the animated values at a given time onto an `IterationFieldInterface`;
/// reading a color field or an iteration field afterwards is up to the caller.
#[gen_stub_pyclass]
#[pyclass(name = "AnimationPlayer")]
pub struct PyAnimationPlayer {
    inner: AnimationPlayer,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyAnimationPlayer {
    /// The end time of the animation: the latest key frame time across all
    /// tracks. Useful as the total duration when rendering or looping.
    pub fn max_time(&self) -> f32 {
        self.inner.max_time()
    }

    /// Applies all animated values at `time` to the given field interface.
    /// After this call the interface reflects the animated state and the usual
    /// `get_color_field` / `get_*_iteration_field` methods can be used.
    pub fn apply_animation(
        &self,
        time: f32,
        // pyo3-stub-gen has no `PyStubType` impl for `&mut`-references, so we
        // tell it the Python type explicitly (same module, no import needed).
        
        #[gen_stub(override_type(type_repr = "IterationFieldInterface"))]
        interface: &mut PyIterationFieldInterface,
    ) {
        self.inner.apply_animation(time, &mut interface.inner);
    }
}
