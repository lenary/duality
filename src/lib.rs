
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

extern crate num;
use num::{Float, Zero, One};

#[cfg(test)]
use std::f32::consts;
#[cfg(test)]
use std::f32;

/// Dual Numbers
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Dual<F> {
    real: F,
    dual: F,
}

pub type Dual32 = Dual<f32>;
pub type Dual64 = Dual<f64>;

impl<F: Copy + Float> Dual<F> {
    pub fn new(real: F, deriv: F) -> Dual<F> {
        Dual {
            real: real,
            dual: deriv,
        }
    }

    pub fn real(self) -> F {
        self.real
    }

    pub fn derivative(self) -> F {
        self.dual
    }
}

impl<F: Copy + Float> Zero for Dual<F> {
    fn zero() -> Dual<F> {
        Dual::new(F::zero(), F::zero())
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.dual.is_zero()
    }
}

impl<F: Copy + Float> One for Dual<F> {
    fn one() -> Dual<F> {
        Dual::new(One::one(), One::one())
    }
}

impl<F: Copy + Float> Add<Dual<F>> for Dual<F> {
    type Output = Dual<F>;

    fn add(self, other: Dual<F>) -> Dual<F> {
        Dual::new(self.real + other.real, self.dual + other.dual)
    }
}

#[test]
fn test_add_struct() {
    let x = Dual::new(3.0, 4.0);
    let y = Dual::new(2.0, 3.0);

    let z = x + y;
    assert!(z.real == 5.0);
    assert!(z.dual == 7.0);
}

impl<F: Copy + Float> Sub<Dual<F>> for Dual<F> {
    type Output = Dual<F>;

    fn sub(self, other: Dual<F>) -> Dual<F> {
        Dual::new(self.real - other.real, self.dual - other.dual)
    }
}

#[test]
fn test_sub() {
    let x = Dual::new(3.0, 4.0);
    let y = Dual::new(2.0, 12.0);

    let z = x - y;
    assert!(z.real == 1.0);
    assert!(z.dual == -8.0);
}

impl<F: Copy + Float + Neg<Output = F>> Neg for Dual<F> {
    type Output = Dual<F>;

    fn neg(self) -> Dual<F> {
        Dual::new(-self.real, -self.dual)
    }
}

#[test]
fn test_neg_plus() {
    let x = Dual::new(3.0, 4.0);
    let y = Dual::new(2.0, 12.0);

    let z1 = x - y;
    let z2 = x + (-y);
    assert!(z1 == z2);
}

#[test]
fn test_neg_zero() {
    let x = Dual::new(3.0, 4.0);
    let zero: Dual32 = Zero::zero();

    let z1 = -x;
    let z2 = zero - x;
    assert!(z1 == z2);
}

impl<F: Copy + Float> Mul<Dual<F>> for Dual<F> {
    type Output = Dual<F>;

    fn mul(self, other: Dual<F>) -> Dual<F> {
        Dual::new(self.real * other.real,
                  (self.real * other.dual) + (self.dual * other.real))
    }
}

#[test]
fn test_mul() {
    let x = Dual::new(3.0, 4.0);
    let y = Dual::new(1.0, 2.0);

    let z = x * y;
    assert!(z.real == 3.0);
    assert!(z.dual == 10.0);
}

impl<F: Copy + Float> Div<Dual<F>> for Dual<F> {
    type Output = Dual<F>;

    fn div(self, other: Dual<F>) -> Dual<F> {
        Dual::new(self.real / other.real,
                  ((self.dual * other.real) - (self.real * other.dual)) / (other.real * other.real))
    }
}

#[test]
fn test_div() {
    let x = Dual::new(3.0, 4.0);
    let y = Dual::new(1.0, 2.0);

    let z = x / y;
    assert!(z.real == 3.0);
    assert!(z.dual == -2.0);
}

impl<F: Copy + Float> Dual<F> {
    pub fn sin(self) -> Dual<F> {
        Dual::new(self.real.sin(), self.dual * self.real.cos())
    }

    pub fn cos(self) -> Dual<F> {
        Dual::new(self.real.cos(), (-F::one()) * self.dual * self.real.sin())
    }

    pub fn tan(self) -> Dual<F> {
        self.sin() / self.cos()
    }

    pub fn exp(self) -> Dual<F> {
        Dual::new(self.real.exp(), self.dual * self.real.exp())
    }

    pub fn ln(self) -> Dual<F> {
        Dual::new(self.real.ln(), self.dual / self.real)
    }

    pub fn sqrt(self) -> Dual<F> {
        let two = F::one() + F::one();
        Dual::new(self.real.sqrt(), self.dual / (two * self.real.sqrt()))
    }
}

impl<F: Copy + Float> From<F> for Dual<F> {
    fn from(real: F) -> Dual<F> {
        Dual::new(real, F::one())
    }
}

impl<F> fmt::Display for Dual<F>
    where F: Copy + Float + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.real < Zero::zero() {
            write!(f, "{}-{}ε", self.real, F::zero() - self.dual)
        } else {
            write!(f, "{}+{}ε", self.real, self.dual)
        }
    }
}

#[cfg(test)]
fn diff(x: f32, y: f32) -> f32 {
    (x - y).abs()
}

#[test]
fn test_sin() {
    let x: Dual32 = From::from(0.0);
    let y = x.sin();

    let real_diff = diff(y.real, 0.0);
    let dual_diff = diff(y.dual, 1.0);

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_cos() {
    let x: Dual32 = From::from(consts::PI);
    let y = x.cos();

    let real_diff = diff(y.real, -1.0);
    let dual_diff = diff(y.dual, 0.0);

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_tan() {
    let x: Dual32 = From::from(0.0);
    let y = x.tan();

    let real_diff = diff(y.real, 0.0);
    let dual_diff = diff(y.dual, 1.0);

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_exp() {
    let x: Dual32 = From::from(1.0);
    let y = x.exp();

    let real_diff = diff(y.real, consts::E);
    let dual_diff = diff(y.dual, consts::E);

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_ln() {
    let x: Dual32 = From::from(1.0);
    let y = x.ln();

    let real_diff = diff(y.real, 0.0);
    let dual_diff = diff(y.dual, 1.0);

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_sqrt() {
    let x: Dual32 = From::from(4.0);
    let y = x.sqrt();

    println!("{}", y);

    let real_diff = diff(y.real, 2.0);
    let dual_diff = diff(y.dual, 0.25);

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}
