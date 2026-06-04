//! This is the base interface for the iterations field. It is the main access point for the
//! python interface.

use rayon::iter::ParallelIterator;
use crate::iteration_and_color::colorization_system::{colorize_continuous, colorize_discrete};
use crate::iteration_and_color::complex_number::ComplexNumber;
use crate::iteration_and_color::iteration_system::{
    WindowExtension, get_iteration_field_continuous, get_iteration_field_discrete,
};
use ndarray::{Array2, Array3};

/// The kind of fractal to render. A pure tag; all parameters live in
/// [`FractalConfig`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FractalType {
    #[default]
    Mandelbrot,
    Julia,
    BurningShip,
    Tricorn,
    Celtic,
}

/// The parametric representation of the fractal we present: its type plus the
/// shared parameters. `exponent` must always be >= 2. `seed_real`/`seed_imag`
/// are only used by the Julia set; they are ignored (but harmless) otherwise.
#[derive(Clone, Copy, Debug)]
pub struct FractalConfig {
    pub fractal_type: FractalType,
    pub exponent: u32,
    pub seed_real: f64,
    pub seed_imag: f64,
}

impl Default for FractalConfig {
    fn default() -> Self {
        Self {
            fractal_type: FractalType::Mandelbrot,
            exponent: 2,
            seed_real: 0.0,
            seed_imag: 0.0,
        }
    }
}

/// Selects the color gradient used by the module's internal colorization.
/// `Cyclical` repeats the gradient `repetition_factor` times across the value range.
#[derive(Clone, Copy)]
pub enum ColorSystem {
    Turbo,
    Viridis,
    Magma,
    Plasma,
    Inferno,
    Cool,
    Cyclical { repetition_factor: f32 },
}

/// Holds all the base configuration and provides the interface calls to
/// generate the iteration field and color information. This is the main access
/// point for the Python interface.
pub struct IterationFieldInterface {
    window_extension: WindowExtension,
    center_point: ComplexNumber,
    extension: f64,
    maximum_iterations: u16,
    fractal_config: FractalConfig,
    color_system: ColorSystem,
    uses_discrete_for_color: bool,
    log_strength_for_color: f32,
}

impl Default for IterationFieldInterface {
    /// The field interface is constructed with sensible parameters.
    fn default() -> Self {
        Self {
            window_extension: WindowExtension::new(1024, 768),
            center_point: ComplexNumber::default(),
            extension: 1.5,
            maximum_iterations: 1000,
            fractal_config: FractalConfig::default(),
            color_system: ColorSystem::Viridis,
            uses_discrete_for_color: true,
            log_strength_for_color: 0.0,
        }
    }
}

impl IterationFieldInterface {
    /// Asks for the discrete iteration field. Returns an array with (rows x columns).
    pub fn get_discrete_iteration_field(&self) -> Array2<u16> {
        let base_vec = get_iteration_field_discrete(
            self.window_extension,
            (&self.fractal_config).into(),
            self.center_point,
            self.extension,
            self.maximum_iterations,
        )
        .collect();
        Array2::from_shape_vec(self.window_extension.get_numpy_shape(), base_vec)
            .expect("shape matches data length by construction")
    }

    /// Asks for the continuous iteration field. Returns an array with (rows x columns).
    pub fn get_continuous_iteration_field(&self) -> Array2<f32> {
        let base_vec = get_iteration_field_continuous(
            self.window_extension,
            (&self.fractal_config).into(),
            self.center_point,
            self.extension,
            self.maximum_iterations,
        )
        .collect();
        Array2::from_shape_vec(self.window_extension.get_numpy_shape(), base_vec)
            .expect("shape matches data length by construction")
    }

    /// Asks for the color field. Returns an array with (rows x columns x 3).
    pub fn get_color_field(&self) -> Array3<u8> {
        let color_field = if self.uses_discrete_for_color {
            let iter_field = get_iteration_field_discrete(
                self.window_extension,
                (&self.fractal_config).into(),
                self.center_point,
                self.extension,
                self.maximum_iterations,
            );
            colorize_discrete(
                iter_field,
                self.maximum_iterations,
                self.log_strength_for_color,
                self.color_system,
            )
            .collect()
        } else {
            let iter_field = get_iteration_field_continuous(
                self.window_extension,
                (&self.fractal_config).into(),
                self.center_point,
                self.extension,
                self.maximum_iterations,
            );
            colorize_continuous(
                iter_field,
                self.maximum_iterations,
                self.log_strength_for_color,
                self.color_system,
            )
            .collect()
        };
        Array3::from_shape_vec(self.window_extension.get_numpy_shape_color(), color_field)
            .expect("shape matches data length by construction")
    }

    /// Set the desired extension of the numpy array.
    pub fn set_numpy_extension(&mut self, rows: usize, columns: usize) {
        self.window_extension = WindowExtension::new(columns as i32, rows as i32);
    }

    /// Sets the fractal type to render. The exponent and Julia seed are set
    /// separately via [`set_fractal_exponent`](Self::set_fractal_exponent) and
    /// [`set_fractal_seed_value`](Self::set_fractal_seed_value).
    pub fn set_fractal_type(&mut self, fractal_type: FractalType) {
        self.fractal_config.fractal_type = fractal_type;
    }

    /// Sets the iteration exponent. Should be >= 2; this is enforced at the
    /// public entry points (the Python binding and the animation key frame
    /// setter), so callers here are trusted.
    pub fn set_fractal_exponent(&mut self, exponent: u32) {
        self.fractal_config.exponent = exponent;
    }

    /// Sets the Julia seed (start) value. Only relevant for the Julia set;
    /// ignored by the other fractal types.
    pub fn set_fractal_seed_value(&mut self, real: f64, imag: f64) {
        self.fractal_config.seed_real = real;
        self.fractal_config.seed_imag = imag;
    }

    /// Sets the maximum number of iterations for the project.
    pub fn set_maximum_iterations(&mut self, maximum_iterations: u16) {
        self.maximum_iterations = maximum_iterations;
    }

    /// Sets the center point for iteration.
    pub fn set_center_point(&mut self, real: f64, imag: f64) {
        self.center_point = ComplexNumber::new(real, imag);
    }

    /// Sets the extension and therefore the zoom factor used for rendering.
    /// The extension is half the side length (in the complex plane) spanned by
    /// the smaller of the window's two dimensions.
    pub fn set_extension(&mut self, extension: f64) {
        self.extension = extension;
    }

    /// Sets the color scheme and whether discrete or continuous iteration
    /// counts are used. The logarithmic color strength is set separately via
    /// [`set_colorization_log_strength`](Self::set_colorization_log_strength).
    pub fn set_colorization_information(
        &mut self,
        color_system: ColorSystem,
        uses_discrete_for_color: bool,
    ) {
        self.color_system = color_system;
        self.uses_discrete_for_color = uses_discrete_for_color;
    }

    /// Sets the strength of the logarithmic color scaling (0.0 = off). The
    /// value is mapped through `exp_m1` (e^x - 1) into the internal multiplier,
    /// both for static rendering and by the animation system.
    pub fn set_colorization_log_strength(&mut self, log_strength: f32) {
        self.log_strength_for_color = log_strength.exp_m1();
    }
}
