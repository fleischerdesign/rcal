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
        self.dims == [0, 0, 0, 0, 0, 0, 0, 1]
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

pub const UNITS: &[(&str, Quantity)] = &[
    (
        "rad",
        Quantity {
            value: 1.0,
            dims: [0, 0, 0, 0, 0, 0, 0, 1],
        },
    ),
    (
        "deg",
        Quantity {
            value: std::f64::consts::PI / 180.0,
            dims: [0, 0, 0, 0, 0, 0, 0, 1],
        },
    ),
    (
        "m",
        Quantity {
            value: 1.0,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "cm",
        Quantity {
            value: 0.01,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "mm",
        Quantity {
            value: 0.001,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "um",
        Quantity {
            value: 1e-6,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "nm",
        Quantity {
            value: 1e-9,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "km",
        Quantity {
            value: 1000.0,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "kg",
        Quantity {
            value: 1.0,
            dims: [0, 1, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "g",
        Quantity {
            value: 0.001,
            dims: [0, 1, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "s",
        Quantity {
            value: 1.0,
            dims: [0, 0, 1, 0, 0, 0, 0, 0],
        },
    ),
    (
        "ms",
        Quantity {
            value: 0.001,
            dims: [0, 0, 1, 0, 0, 0, 0, 0],
        },
    ),
    (
        "us",
        Quantity {
            value: 1e-6,
            dims: [0, 0, 1, 0, 0, 0, 0, 0],
        },
    ),
    (
        "ns",
        Quantity {
            value: 1e-9,
            dims: [0, 0, 1, 0, 0, 0, 0, 0],
        },
    ),
    (
        "min",
        Quantity {
            value: 60.0,
            dims: [0, 0, 1, 0, 0, 0, 0, 0],
        },
    ),
    (
        "h",
        Quantity {
            value: 3600.0,
            dims: [0, 0, 1, 0, 0, 0, 0, 0],
        },
    ),
    (
        "Wh",
        Quantity {
            value: 3600.0,
            dims: [2, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "kWh",
        Quantity {
            value: 3_600_000.0,
            dims: [2, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "kmh",
        Quantity {
            value: 1000.0 / 3600.0,
            dims: [1, 0, -1, 0, 0, 0, 0, 0],
        },
    ),
    (
        "A",
        Quantity {
            value: 1.0,
            dims: [0, 0, 0, 1, 0, 0, 0, 0],
        },
    ),
    (
        "K",
        Quantity {
            value: 1.0,
            dims: [0, 0, 0, 0, 1, 0, 0, 0],
        },
    ),
    (
        "mol",
        Quantity {
            value: 1.0,
            dims: [0, 0, 0, 0, 0, 1, 0, 0],
        },
    ),
    (
        "cd",
        Quantity {
            value: 1.0,
            dims: [0, 0, 0, 0, 0, 0, 1, 0],
        },
    ),
    (
        "l",
        Quantity {
            value: 0.001,
            dims: [3, 0, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "bar",
        Quantity {
            value: 100_000.0,
            dims: [-1, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "atm",
        Quantity {
            value: 101_325.0,
            dims: [-1, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "inch",
        Quantity {
            value: 0.0254,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "ft",
        Quantity {
            value: 0.3048,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        },
    ),
    (
        "eV",
        Quantity {
            value: 1.602_176_634e-19,
            dims: [2, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "kJ",
        Quantity {
            value: 1000.0,
            dims: [2, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "MJ",
        Quantity {
            value: 1_000_000.0,
            dims: [2, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "N",
        Quantity {
            value: 1.0,
            dims: [1, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "J",
        Quantity {
            value: 1.0,
            dims: [2, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "W",
        Quantity {
            value: 1.0,
            dims: [2, 1, -3, 0, 0, 0, 0, 0],
        },
    ),
    (
        "mW",
        Quantity {
            value: 0.001,
            dims: [2, 1, -3, 0, 0, 0, 0, 0],
        },
    ),
    (
        "kW",
        Quantity {
            value: 1000.0,
            dims: [2, 1, -3, 0, 0, 0, 0, 0],
        },
    ),
    (
        "MW",
        Quantity {
            value: 1_000_000.0,
            dims: [2, 1, -3, 0, 0, 0, 0, 0],
        },
    ),
    (
        "Pa",
        Quantity {
            value: 1.0,
            dims: [-1, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "hPa",
        Quantity {
            value: 100.0,
            dims: [-1, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "kPa",
        Quantity {
            value: 1000.0,
            dims: [-1, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "MPa",
        Quantity {
            value: 1_000_000.0,
            dims: [-1, 1, -2, 0, 0, 0, 0, 0],
        },
    ),
    (
        "Hz",
        Quantity {
            value: 1.0,
            dims: [0, 0, -1, 0, 0, 0, 0, 0],
        },
    ),
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

        for (name, unit) in UNITS {
            if unit.value == 1.0 && self.dims == unit.dims {
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
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        };
        let q2 = Quantity {
            value: 3.0,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        };
        let res = q1 * q2;
        assert_eq!(res.value, 6.0);
        assert_eq!(res.dims, [2, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_quantity_add_err() {
        let q1 = Quantity {
            value: 2.0,
            dims: [1, 0, 0, 0, 0, 0, 0, 0],
        };
        let q2 = Quantity {
            value: 3.0,
            dims: [0, 1, 0, 0, 0, 0, 0, 0],
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
