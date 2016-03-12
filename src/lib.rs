
use std::ops::*;

#[cfg(test)]
use std::f32::consts;
#[cfg(test)]
use std::f32;

/// Dual Numbers
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct DualNumber {
    real: f32,
    dual: f32,
}

impl DualNumber {
    pub fn new(real: f32, deriv: f32) -> DualNumber {
        DualNumber {
            real: real,
            dual: deriv,
        }
    }

    pub fn real(self) -> f32 {
        self.real
    }

    pub fn derivative(self) -> f32 {
        self.dual
    }
}

impl Default for DualNumber {
    fn default() -> DualNumber {
        DualNumber::new(0.0, 0.0)
    }
}

impl From<f32> for DualNumber {
    fn from(n: f32) -> DualNumber {
        DualNumber::new(n, 0.0)
    }
}

impl Add for DualNumber {
    type Output = DualNumber;

    fn add(self, other: DualNumber) -> DualNumber {
        DualNumber::new(self.real + other.real, self.dual + other.dual)
    }
}

#[test]
fn test_add_struct() {
    let x = DualNumber::new(3.0, 4.0);
    let y = DualNumber::new(2.0, 3.0);

    let z = x + y;
    assert!(z.real == 5.0);
    assert!(z.dual == 7.0);
}

impl Sub for DualNumber {
    type Output = DualNumber;

    fn sub(self, other: DualNumber) -> DualNumber {
        DualNumber::new(self.real - other.real, self.dual - other.dual)
    }
}

#[test]
fn test_sub() {
    let x = DualNumber::new(3.0, 4.0);
    let y = DualNumber::new(2.0, 12.0);

    let z = x - y;
    assert!(z.real == 1.0);
    assert!(z.dual == -8.0);
}

impl Neg for DualNumber {
    type Output = DualNumber;

    fn neg(self) -> DualNumber {
        DualNumber::new(-self.real, -self.dual)
    }
}

#[test]
fn test_neg_plus() {
    let x = DualNumber::new(3.0, 4.0);
    let y = DualNumber::new(2.0, 12.0);

    let z1 = x - y;
    let z2 = x + (-y);
    assert!(z1 == z2);
}

#[test]
fn test_neg_zero() {
    let x = DualNumber::new(3.0, 4.0);
    let zero: DualNumber = Default::default();

    let z1 = -x;
    let z2 = zero - x;
    assert!(z1 == z2);
}

impl Mul for DualNumber {
    type Output = DualNumber;

    fn mul(self, other: DualNumber) -> DualNumber {
        DualNumber::new(self.real * other.real,
                        (self.real * other.dual) + (self.dual * other.real))
    }
}

#[test]
fn test_mul() {
    let x = DualNumber::new(3.0, 4.0);
    let y = DualNumber::new(1.0, 2.0);

    let z = x * y;
    assert!(z.real == 3.0);
    assert!(z.dual == 10.0);
}

impl Div for DualNumber {
    type Output = DualNumber;

    fn div(self, other: DualNumber) -> DualNumber {
        DualNumber::new(self.real / other.real,
                        ((self.dual * other.real) - (self.real * other.dual)) /
                        (other.real * other.real))
    }
}

#[test]
fn test_div() {
    let x = DualNumber::new(3.0, 4.0);
    let y = DualNumber::new(1.0, 2.0);

    let z = x / y;
    assert!(z.real == 3.0);
    assert!(z.dual == -2.0);
}

impl DualNumber {
    pub fn sin(self) -> DualNumber {
        DualNumber::new(self.real.sin(), self.dual * self.real.cos())
    }

    pub fn cos(self) -> DualNumber {
        DualNumber::new(self.real.cos(), self.dual * self.real.sin())
    }

    pub fn exp(self) -> DualNumber {
        DualNumber::new(self.real.exp(), self.dual * self.real.exp())
    }

    pub fn ln(self) -> DualNumber {
        DualNumber::new(self.real.ln(), self.dual / self.real)
    }

    pub fn sqrt(self) -> DualNumber {
        DualNumber::new(self.real.sqrt(), self.dual / (2.0 * self.real.sqrt()))
    }
}

#[test]
fn test_sin() {
    let x: DualNumber = From::from(consts::PI);
    let y = x.sin();

    let real_diff = y.real - 0.0;
    let dual_diff = y.dual - 1.0;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_cos() {
    let x: DualNumber = From::from(consts::PI);
    let y = x.cos();

    let real_diff = y.real - 1.0;
    let dual_diff = y.dual - 0.0;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_exp() {
    let x: DualNumber = From::from(1.0);
    let y = x.exp();

    let real_diff = y.real - consts::E;
    let dual_diff = y.dual - consts::E;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_ln() {
    let x: DualNumber = From::from(consts::E);
    let y = x.ln();

    let real_diff = y.real - 1.0;
    let dual_diff = y.dual - consts::E;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_sqrt() {
    let x: DualNumber = From::from(100.0);
    let y = x.ln();

    let real_diff = y.real - 10.0;
    let dual_diff = y.dual - 0.05;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}
