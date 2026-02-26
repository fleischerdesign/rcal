use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity {
    pub value: f64,
    pub dims: [i8; 8],
}

impl Quantity {
    pub fn scalar(value: f64) -> Self {
        Self {
            value,
            dims: [0; 8],
        }
    }

    pub fn is_scalar(&self) -> bool {
        self.dims == [0; 8]
    }

    pub fn is_angle(&self) -> bool {
        self.dims == ANGLE
    }

    pub fn pow(self, exp: f64) -> Result<Self, String> {
        if !self.is_scalar() && exp.fract() != 0.0 {
            return Err("Cannot raise non-scalar to non-integer power".into());
        }
        let mut new_dims = [0i8; 8];
        for (i, dim) in new_dims.iter_mut().enumerate() {
            *dim = (self.dims[i] as f64 * exp) as i8;
        }
        Ok(Self {
            value: self.value.powf(exp),
            dims: new_dims,
        })
    }
}

impl Add for Quantity {
    type Output = Result<Self, String>;
    fn add(self, rhs: Self) -> Self::Output {
        if self.dims != rhs.dims {
            return Err("Dimension mismatch".into());
        }
        Ok(Self {
            value: self.value + rhs.value,
            dims: self.dims,
        })
    }
}

impl Sub for Quantity {
    type Output = Result<Self, String>;
    fn sub(self, rhs: Self) -> Self::Output {
        if self.dims != rhs.dims {
            return Err("Dimension mismatch".into());
        }
        Ok(Self {
            value: self.value - rhs.value,
            dims: self.dims,
        })
    }
}

impl Mul for Quantity {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut dims = [0i8; 8];
        for (i, dim) in dims.iter_mut().enumerate() {
            *dim = self.dims[i] + rhs.dims[i];
        }
        Self {
            value: self.value * rhs.value,
            dims,
        }
    }
}

impl Div for Quantity {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let mut dims = [0i8; 8];
        for (i, dim) in dims.iter_mut().enumerate() {
            *dim = self.dims[i] - rhs.dims[i];
        }
        Self {
            value: self.value / rhs.value,
            dims,
        }
    }
}

impl Neg for Quantity {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            value: -self.value,
            dims: self.dims,
        }
    }
}

pub const LENGTH: [i8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];
pub const AREA: [i8; 8] = [2, 0, 0, 0, 0, 0, 0, 0];
pub const VOLUME: [i8; 8] = [3, 0, 0, 0, 0, 0, 0, 0];
pub const MASS: [i8; 8] = [0, 1, 0, 0, 0, 0, 0, 0];
pub const TIME: [i8; 8] = [0, 0, 1, 0, 0, 0, 0, 0];
pub const CURRENT: [i8; 8] = [0, 0, 0, 1, 0, 0, 0, 0];
pub const TEMPERATURE: [i8; 8] = [0, 0, 0, 0, 1, 0, 0, 0];
pub const AMOUNT_OF_SUBSTANCE: [i8; 8] = [0, 0, 0, 0, 0, 1, 0, 0];
pub const LUMINOUS_INTENSITY: [i8; 8] = [0, 0, 0, 0, 0, 0, 1, 0];
pub const ANGLE: [i8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];

pub const FREQUENCY: [i8; 8] = [0, 0, -1, 0, 0, 0, 0, 0];
pub const VELOCITY: [i8; 8] = [1, 0, -1, 0, 0, 0, 0, 0];
pub const ACCELERATION: [i8; 8] = [1, 0, -2, 0, 0, 0, 0, 0];
pub const FORCE: [i8; 8] = [1, 1, -2, 0, 0, 0, 0, 0];
pub const PRESSURE: [i8; 8] = [-1, 1, -2, 0, 0, 0, 0, 0];
pub const ENERGY: [i8; 8] = [2, 1, -2, 0, 0, 0, 0, 0];
pub const POWER: [i8; 8] = [2, 1, -3, 0, 0, 0, 0, 0];

pub const GRAVITATIONAL_CONSTANT: [i8; 8] = [3, -1, -2, 0, 0, 0, 0, 0];
pub const ACTION: [i8; 8] = [2, 1, -1, 0, 0, 0, 0, 0];
pub const BOLTZMANN_CONSTANT: [i8; 8] = [2, 1, -2, 0, -1, 0, 0, 0];
pub const AVOGADRO_CONSTANT: [i8; 8] = [0, 0, 0, 0, 0, -1, 0, 0];

const SMART_UNITS: &[(&str, [i8; 8])] = &[
    ("Hz", FREQUENCY),
    ("N", FORCE),
    ("Pa", PRESSURE),
    ("J", ENERGY),
    ("W", POWER),
    ("m s^-1", VELOCITY),
    ("m s^-2", ACCELERATION),
    ("m^2", AREA),
    ("m^3", VOLUME),
    ("rad", ANGLE),
];

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val_str = if self.value == 0.0 {
            "0".to_string()
        } else {
            format!("{}", self.value)
        };
        write!(f, "{}", val_str)?;

        if self.is_scalar() {
            return Ok(());
        }

        for (name, dims) in SMART_UNITS {
            if &self.dims == dims {
                return write!(f, " {}", name);
            }
        }

        let units = ["m", "kg", "s", "A", "K", "mol", "cd", "rad"];
        for (i, &unit) in units.iter().enumerate() {
            if self.dims[i] != 0 {
                write!(f, " {}", unit)?;
                if self.dims[i] != 1 {
                    write!(f, "^{}", self.dims[i])?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantity_mul() {
        let q1 = Quantity {
            value: 2.0,
            dims: LENGTH,
        };
        let q2 = Quantity {
            value: 3.0,
            dims: LENGTH,
        };
        let res = q1 * q2;
        assert_eq!(res.value, 6.0);
        assert_eq!(res.dims, AREA);
    }

    #[test]
    fn test_quantity_add_err() {
        let q1 = Quantity {
            value: 2.0,
            dims: LENGTH,
        };
        let q2 = Quantity {
            value: 3.0,
            dims: MASS,
        };
        assert!((q1 + q2).is_err());
    }

    #[test]
    fn test_scalar() {
        let q = Quantity::scalar(5.0);
        assert!(q.is_scalar());
        assert_eq!(q.value, 5.0);
    }
}
