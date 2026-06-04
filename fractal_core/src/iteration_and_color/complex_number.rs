//! This is a module that represents complex numbers based on f64 for fractal computations.

use std::ops::{Add, AddAssign, Mul, Sub};

/// The classical complex number of a real part and an imaginary part.
#[derive(Debug, Clone, Copy, Default)]
pub struct ComplexNumber {
    pub real: f64,
    pub imag: f64,
}

impl ComplexNumber {
    /// Creates a complex number from the components.
    pub fn new(real: f64, imag: f64) -> ComplexNumber {
        ComplexNumber { real, imag }
    }

    /// Gets the squared magnitude of the complex number.
    pub fn sq_magnitude(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    /// A simple recursive style power function explicitly programmed for the most important cases.
    pub fn power(&self, exponent: u32) -> ComplexNumber {
        match exponent {
            0 => ComplexNumber::new(1.0, 0.0),
            1 => *self,
            2 => *self * *self,
            3 => *self * *self * *self,
            _ if exponent & 0b1 == 0 => {
                let half = self.power(exponent >> 1);
                half * half
            }
            _ => *self * self.power(exponent - 1),
        }
    }

    /// The component wise absolute value needed for the burning ship fractal.
    pub fn component_wise_abs(&self) -> ComplexNumber {
        ComplexNumber {
            real: self.real.abs(),
            imag: self.imag.abs(),
        }
    }

    /// The complex conjugate (the imaginary part is negated).
    pub fn conjugate(&self) -> ComplexNumber {
        ComplexNumber {
            real: self.real,
            imag: -self.imag,
        }
    }
}

impl Add for ComplexNumber {
    type Output = ComplexNumber;

    fn add(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl AddAssign for ComplexNumber {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl Sub for ComplexNumber {
    type Output = ComplexNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Mul for ComplexNumber {
    type Output = ComplexNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        ComplexNumber {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}
