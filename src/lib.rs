
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

impl Add for DualNumber {
    type Output = DualNumber;

    fn add(self, other: DualNumber) -> DualNumber {
        DualNumber {
            real: (self.real + other.real),
            dual: (self.dual + other.dual),
        }
    }
}

#[test]
fn test_add_struct() {
    let x = DualNumber {
        real: 3.0,
        dual: 4.0,
    };
    let y = DualNumber {
        real: 2.0,
        dual: 3.0,
    };

    let z = x + y;
    assert!(z.real == 5.0);
    assert!(z.dual == 7.0);
}

impl Sub for DualNumber {
    type Output = DualNumber;

    fn sub(self, other: DualNumber) -> DualNumber {
        DualNumber {
            real: (self.real - other.real),
            dual: (self.dual - other.dual),
        }
    }
}

#[test]
fn test_sub() {
    let x = DualNumber {
        real: 3.0,
        dual: 4.0,
    };
    let y = DualNumber {
        real: 2.0,
        dual: 12.0,
    };

    let z = x - y;
    assert!(z.real == 1.0);
    assert!(z.dual == -8.0);
}

impl Neg for DualNumber {
    type Output = DualNumber;

    fn neg(self) -> DualNumber {
        DualNumber {
            real: -self.real,
            dual: -self.dual,
        }
    }
}

#[test]
fn test_neg_plus() {
    let x = DualNumber {
        real: 3.0,
        dual: 4.0,
    };
    let y = DualNumber {
        real: 2.0,
        dual: 12.0,
    };

    let z1 = x - y;
    let z2 = x + (-y);
    assert!(z1 == z2);
}

#[test]
fn test_neg_zero() {
    let x = DualNumber {
        real: 3.0,
        dual: 4.0,
    };
    let zero = DualNumber {
        real: 0.0,
        dual: 0.0,
    };

    let z1 = -x;
    let z2 = zero - x;
    assert!(z1 == z2);
}

impl Mul for DualNumber {
    type Output = DualNumber;

    fn mul(self, other: DualNumber) -> DualNumber {
        DualNumber {
            real: (self.real * other.real),
            dual: (self.real * other.dual) + (self.dual * other.real),
        }
    }
}

#[test]
fn test_mul() {
    let x = DualNumber {
        real: 3.0,
        dual: 4.0,
    };
    let y = DualNumber {
        real: 1.0,
        dual: 2.0,
    };

    let z = x * y;
    assert!(z.real == 3.0);
    assert!(z.dual == 10.0);
}

impl Div for DualNumber {
    type Output = DualNumber;

    fn div(self, other: DualNumber) -> DualNumber {
        DualNumber {
            real: (self.real / other.real),
            dual: ((self.dual * other.real) - (self.real * other.dual)) / (other.real * other.real),
        }
    }
}

#[test]
fn test_div() {
    let x = DualNumber {
        real: 3.0,
        dual: 4.0,
    };
    let y = DualNumber {
        real: 1.0,
        dual: 2.0,
    };

    let z = x / y;
    assert!(z.real == 3.0);
    assert!(z.dual == -2.0);
}

impl DualNumber {
    pub fn new(real: f32) -> DualNumber {
        DualNumber {
            real: real,
            dual: 1.0,
        }
    }

    pub fn real(self) -> f32 {
        self.real
    }

    pub fn differentiate(self) -> f32 {
        self.dual
    }

    pub fn sin(self) -> DualNumber {
        DualNumber {
            real: self.real.sin(),
            dual: self.dual * self.real.cos(),
        }
    }

    pub fn cos(self) -> DualNumber {
        DualNumber {
            real: self.real.cos(),
            dual: self.dual * self.real.sin(),
        }
    }

    pub fn exp(self) -> DualNumber {
        DualNumber {
            real: self.real.exp(),
            dual: self.dual * self.real.exp(),
        }
    }

    pub fn ln(self) -> DualNumber {
        DualNumber {
            real: self.real.ln(),
            dual: self.dual / self.real,
        }
    }

    pub fn sqrt(self) -> DualNumber {
        DualNumber {
            real: self.real.sqrt(),
            dual: self.dual / (2.0 * self.real.sqrt()),
        }
    }
}

#[test]
fn test_sin() {
    let x = DualNumber::new(consts::PI);
    let y = x.sin();

    let real_diff = y.real - 0.0;
    let dual_diff = y.dual - 1.0;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_cos() {
    let x = DualNumber::new(consts::PI);
    let y = x.cos();

    let real_diff = y.real - 1.0;
    let dual_diff = y.dual - 0.0;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_exp() {
    let x = DualNumber::new(1.0);
    let y = x.exp();

    let real_diff = y.real - consts::E;
    let dual_diff = y.dual - consts::E;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_ln() {
    let x = DualNumber::new(consts::E);
    let y = x.ln();

    let real_diff = y.real - 1.0;
    let dual_diff = y.dual - consts::E;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}

#[test]
fn test_sqrt() {
    let x = DualNumber::new(100.0);
    let y = x.ln();

    let real_diff = y.real - 10.0;
    let dual_diff = y.dual - 0.05;

    assert!(real_diff < f32::EPSILON);
    assert!(dual_diff < f32::EPSILON);
}
