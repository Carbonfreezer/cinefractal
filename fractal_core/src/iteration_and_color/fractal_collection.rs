//! This module has the most important fractals for escape time computations
//! - Mandelbrot
//! - Julia
//! - Burning Ship
//! - Celtic
//! - Tricorn

use crate::iteration_and_color::complex_number::ComplexNumber;
use crate::iteration_and_color::iteration_field_interface::{FractalConfig, FractalType};

/// The trait all iteration systems have to implement.
pub trait EscapeIteration: Clone + Copy + Send + Sync {
    /// Sets the complex number for the pixel we represent in the complex number plane.
    fn set_start_pixel_in_number_pane(&mut self, start_pixel_in_number: ComplexNumber);
    /// The starting value from where we start iterating the process.
    fn get_iteration_start_value(&self) -> ComplexNumber;
    /// Performs one iterative computation step on the complex number.
    fn get_iteration_step(&self, input: ComplexNumber) -> ComplexNumber;
    /// Gets the exponent of the iterator, needed to compute the smoothed color field value.
    fn get_smoothing_degree(&self) -> f64;
    /// Asks for the squared bail-out radius. This defaults to 4.0; implement it if it differs.
    fn get_squared_bail_out_radius(&self) -> f64 {
        4.0
    }
}

// We start with the holomorph and most common functions.

#[derive(Debug, Clone, Copy, Default)]
/// The classical Mandelbrot set
pub struct Mandelbrot {
    constant: ComplexNumber,
    exponent: u32,
}

impl Mandelbrot {
    pub fn new(exponent: u32) -> Mandelbrot {
        Self {
            exponent,
            ..Default::default()
        }
    }
}

impl EscapeIteration for Mandelbrot {
    fn set_start_pixel_in_number_pane(&mut self, start_pixel_in_number: ComplexNumber) {
        self.constant = start_pixel_in_number;
    }

    fn get_iteration_start_value(&self) -> ComplexNumber {
        ComplexNumber::new(0.0, 0.0)
    }

    fn get_iteration_step(&self, input: ComplexNumber) -> ComplexNumber {
        input.power(self.exponent) + self.constant
    }

    fn get_smoothing_degree(&self) -> f64 {
        self.exponent as f64
    }
}

#[derive(Debug, Clone, Copy, Default)]
/// The Julia set can be parametrized over a starting parameter.
pub struct Julia {
    constant: ComplexNumber,
    start_pixel: ComplexNumber,
    exponent: u32,
}

impl Julia {
    pub fn new(start_number: ComplexNumber, exponent: u32) -> Self {
        Self {
            constant: start_number,
            exponent,
            ..Default::default()
        }
    }
}

impl EscapeIteration for Julia {
    fn set_start_pixel_in_number_pane(&mut self, start_pixel_in_number: ComplexNumber) {
        self.start_pixel = start_pixel_in_number;
    }
    fn get_iteration_start_value(&self) -> ComplexNumber {
        self.start_pixel
    }
    fn get_iteration_step(&self, input: ComplexNumber) -> ComplexNumber {
        input.power(self.exponent) + self.constant
    }

    fn get_smoothing_degree(&self) -> f64 {
        self.exponent as f64
    }
}

#[derive(Debug, Clone, Copy, Default)]
/// Strange fractal, where the name is self explanatory.
pub struct BurningShip {
    constant: ComplexNumber,
    exponent: u32,
}

impl BurningShip {
    pub fn new(exponent: u32) -> Self {
        Self {
            exponent,
            ..Default::default()
        }
    }
}

impl EscapeIteration for BurningShip {
    fn set_start_pixel_in_number_pane(&mut self, start_pixel_in_number: ComplexNumber) {
        self.constant = start_pixel_in_number;
    }

    fn get_iteration_start_value(&self) -> ComplexNumber {
        ComplexNumber::new(0.0, 0.0)
    }

    fn get_iteration_step(&self, input: ComplexNumber) -> ComplexNumber {
        input.component_wise_abs().power(self.exponent) + self.constant
    }

    fn get_smoothing_degree(&self) -> f64 {
        self.exponent as f64
    }

    fn get_squared_bail_out_radius(&self) -> f64 {
        2f64.powf(16f64)
    }
}

#[derive(Debug, Clone, Copy, Default)]
/// Symmetric butterfly like structure.
pub struct Celtic {
    constant: ComplexNumber,
    exponent: u32,
}

impl Celtic {
    pub fn new(exponent: u32) -> Celtic {
        Self {
            exponent,
            ..Default::default()
        }
    }
}

impl EscapeIteration for Celtic {
    fn set_start_pixel_in_number_pane(&mut self, start_pixel_in_number: ComplexNumber) {
        self.constant = start_pixel_in_number;
    }

    fn get_iteration_start_value(&self) -> ComplexNumber {
        ComplexNumber::new(0.0, 0.0)
    }

    fn get_iteration_step(&self, input: ComplexNumber) -> ComplexNumber {
        let w = input.power(self.exponent);
        ComplexNumber::new(w.real.abs(), w.imag) + self.constant
    }

    fn get_smoothing_degree(&self) -> f64 {
        self.exponent as f64
    }

    fn get_squared_bail_out_radius(&self) -> f64 {
        2f64.powf(16f64)
    }
}

#[derive(Debug, Clone, Copy, Default)]
/// As the name suggests, this fractal (the "Mandelbar") shows a three-cornered, threefold-symmetric structure.
pub struct Tricorn {
    constant: ComplexNumber,
    exponent: u32,
}

impl Tricorn {
    pub fn new(exponent: u32) -> Self {
        Self {
            exponent,
            ..Default::default()
        }
    }
}

impl EscapeIteration for Tricorn {
    fn set_start_pixel_in_number_pane(&mut self, start_pixel_in_number: ComplexNumber) {
        self.constant = start_pixel_in_number;
    }

    fn get_iteration_start_value(&self) -> ComplexNumber {
        ComplexNumber::new(0.0, 0.0)
    }

    fn get_iteration_step(&self, input: ComplexNumber) -> ComplexNumber {
        input.conjugate().power(self.exponent) + self.constant
    }
    fn get_smoothing_degree(&self) -> f64 {
        self.exponent as f64
    }

    fn get_squared_bail_out_radius(&self) -> f64 {
        2f64.powf(16f64)
    }
}

#[derive(Clone, Copy)]
/// The iteration type is a means to avoid Box<dyn> and go over monomorphisation.
/// Hand-rolled static dispatch over EscapeIteration implementors; the enum_dispatch crate generates this pattern via macro.
pub enum AnyEscapeIteration {
    Mandelbrot(Mandelbrot),
    Julia(Julia),
    BurningShip(BurningShip),
    Celtic(Celtic),
    Tricorn(Tricorn),
}

impl AnyEscapeIteration {
    /// Generalization of the smoothing degree for all the sub types.
    pub fn get_smoothing_degree(&self) -> f64 {
        match &self {
            AnyEscapeIteration::Mandelbrot(mandelbrot) => mandelbrot.get_smoothing_degree(),
            AnyEscapeIteration::Julia(julia) => julia.get_smoothing_degree(),
            AnyEscapeIteration::BurningShip(ship) => ship.get_smoothing_degree(),
            AnyEscapeIteration::Tricorn(tricorn) => tricorn.get_smoothing_degree(),
            AnyEscapeIteration::Celtic(celtic) => celtic.get_smoothing_degree(),
        }
    }
}

impl From<&FractalConfig> for AnyEscapeIteration {
    /// Conversion function from the parameter type accessible from the outside to the internal representation.
    fn from(para: &FractalConfig) -> Self {
        let exponent = para.exponent;
        match para.fractal_type {
            FractalType::Mandelbrot => AnyEscapeIteration::Mandelbrot(Mandelbrot::new(exponent)),
            FractalType::Julia => AnyEscapeIteration::Julia(Julia::new(
                ComplexNumber::new(para.seed_real, para.seed_imag),
                exponent,
            )),
            FractalType::BurningShip => AnyEscapeIteration::BurningShip(BurningShip::new(exponent)),
            FractalType::Celtic => AnyEscapeIteration::Celtic(Celtic::new(exponent)),
            FractalType::Tricorn => AnyEscapeIteration::Tricorn(Tricorn::new(exponent)),
        }
    }
}
