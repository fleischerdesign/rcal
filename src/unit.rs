//! Dimensional analysis and unit management.

use std::ops::{Add, Div, Mul, Neg, Sub};

/// Represents the physical dimensions of a quantity using SI base units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Dimensions {
    pub length: i8,
    pub mass: i8,
    pub time: i8,
    pub current: i8,
    pub temperature: i8,
    pub amount: i8,
    pub intensity: i8,
    pub angle: i8,
}

impl Dimensions {
    /// Dimensionless scalar.
    pub const SCALAR: Self = Self {
        length: 0,
        mass: 0,
        time: 0,
        current: 0,
        temperature: 0,
        amount: 0,
        intensity: 0,
        angle: 0,
    };

    /// Returns true if the dimensions represent a scalar.
    pub fn is_scalar(&self) -> bool {
        *self == Self::SCALAR
    }

    /// Raises the dimensions to a power.
    pub fn pow(self, exp: i8) -> Self {
        Self {
            length: self.length * exp,
            mass: self.mass * exp,
            time: self.time * exp,
            current: self.current * exp,
            temperature: self.temperature * exp,
            amount: self.amount * exp,
            intensity: self.intensity * exp,
            angle: self.angle * exp,
        }
    }
}

impl std::ops::Add for Dimensions {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            length: self.length + rhs.length,
            mass: self.mass + rhs.mass,
            time: self.time + rhs.time,
            current: self.current + rhs.current,
            temperature: self.temperature + rhs.temperature,
            amount: self.amount + rhs.amount,
            intensity: self.intensity + rhs.intensity,
            angle: self.angle + rhs.angle,
        }
    }
}

impl std::ops::Sub for Dimensions {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            length: self.length - rhs.length,
            mass: self.mass - rhs.mass,
            time: self.time - rhs.time,
            current: self.current - rhs.current,
            temperature: self.temperature - rhs.temperature,
            amount: self.amount - rhs.amount,
            intensity: self.intensity - rhs.intensity,
            angle: self.angle - rhs.angle,
        }
    }
}

impl std::fmt::Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        let units = [
            ("m", self.length),
            ("kg", self.mass),
            ("s", self.time),
            ("A", self.current),
            ("K", self.temperature),
            ("mol", self.amount),
            ("cd", self.intensity),
            ("rad", self.angle),
        ];

        for (name, val) in units {
            if val != 0 {
                if !first {
                    write!(f, " ")?;
                }
                write!(f, "{}", name)?;
                if val != 1 {
                    write!(f, "^{}", val)?;
                }
                first = false;
            }
        }
        Ok(())
    }
}

/// A value paired with its physical dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity {
    /// The numeric value in SI base units.
    pub value: f64,
    /// The physical dimensions.
    pub dims: Dimensions,
}

impl Quantity {
    /// Creates a new dimensionless scalar.
    pub fn scalar(value: f64) -> Self {
        Self {
            value,
            dims: Dimensions::SCALAR,
        }
    }

    /// Returns true if the quantity is a scalar.
    pub fn is_scalar(&self) -> bool {
        self.dims.is_scalar()
    }

    /// Returns true if the quantity has the dimensions of an angle (radians).
    pub fn is_angle(&self) -> bool {
        self.dims == ANGLE
    }

    /// Raises the quantity to a power.
    pub fn pow(self, exp: f64) -> Result<Self, String> {
        if !self.is_scalar() && exp.fract() != 0.0 {
            return Err("Cannot raise non-scalar to non-integer power".into());
        }
        Ok(Self {
            value: self.value.powf(exp),
            dims: self.dims.pow(exp as i8),
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
        Self {
            value: self.value * rhs.value,
            dims: self.dims + rhs.dims,
        }
    }
}

impl Div for Quantity {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            value: self.value / rhs.value,
            dims: self.dims - rhs.dims,
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

// Fundamental Dimensions
pub const LENGTH: Dimensions = Dimensions {
    length: 1,
    ..Dimensions::SCALAR
};
pub const MASS: Dimensions = Dimensions {
    mass: 1,
    ..Dimensions::SCALAR
};
pub const TIME: Dimensions = Dimensions {
    time: 1,
    ..Dimensions::SCALAR
};
pub const CURRENT: Dimensions = Dimensions {
    current: 1,
    ..Dimensions::SCALAR
};
pub const TEMPERATURE: Dimensions = Dimensions {
    temperature: 1,
    ..Dimensions::SCALAR
};
pub const AMOUNT_OF_SUBSTANCE: Dimensions = Dimensions {
    amount: 1,
    ..Dimensions::SCALAR
};
pub const LUMINOUS_INTENSITY: Dimensions = Dimensions {
    intensity: 1,
    ..Dimensions::SCALAR
};
pub const ANGLE: Dimensions = Dimensions {
    angle: 1,
    ..Dimensions::SCALAR
};

// Derived Dimensions
pub const AREA: Dimensions = Dimensions {
    length: 2,
    ..Dimensions::SCALAR
};
pub const VOLUME: Dimensions = Dimensions {
    length: 3,
    ..Dimensions::SCALAR
};
pub const VELOCITY: Dimensions = Dimensions {
    length: 1,
    time: -1,
    ..Dimensions::SCALAR
};
pub const ACCELERATION: Dimensions = Dimensions {
    length: 1,
    time: -2,
    ..Dimensions::SCALAR
};
pub const FORCE: Dimensions = Dimensions {
    length: 1,
    mass: 1,
    time: -2,
    ..Dimensions::SCALAR
};
pub const ENERGY: Dimensions = Dimensions {
    length: 2,
    mass: 1,
    time: -2,
    ..Dimensions::SCALAR
};
pub const POWER: Dimensions = Dimensions {
    length: 2,
    mass: 1,
    time: -3,
    ..Dimensions::SCALAR
};
pub const PRESSURE: Dimensions = Dimensions {
    length: -1,
    mass: 1,
    time: -2,
    ..Dimensions::SCALAR
};
pub const FREQUENCY: Dimensions = Dimensions {
    time: -1,
    ..Dimensions::SCALAR
};

// Physical Constant Dimensions
pub const GRAVITATIONAL_CONSTANT: Dimensions = Dimensions {
    length: 3,
    mass: -1,
    time: -2,
    ..Dimensions::SCALAR
};
pub const ACTION: Dimensions = Dimensions {
    length: 2,
    mass: 1,
    time: -1,
    ..Dimensions::SCALAR
};
pub const BOLTZMANN_CONSTANT: Dimensions = Dimensions {
    length: 2,
    mass: 1,
    time: -2,
    temperature: -1,
    ..Dimensions::SCALAR
};
pub const AVOGADRO_CONSTANT: Dimensions = Dimensions {
    amount: -1,
    ..Dimensions::SCALAR
};

/// A named unit of measurement with a conversion factor and optional offset.
#[derive(Debug, Clone, Copy)]
pub struct Unit {
    pub name: &'static str,
    pub quantity: Quantity,
    pub offset: f64,
}

impl Unit {
    /// Creates a new unit definition.
    pub const fn new(name: &'static str, value: f64, dims: Dimensions, offset: f64) -> Self {
        Self {
            name,
            quantity: Quantity { value, dims },
            offset,
        }
    }

    /// Converts a value from SI base units to this unit.
    pub fn convert_from_si(self, val: f64) -> f64 {
        (val / self.quantity.value) - self.offset
    }

    /// Converts a value from this unit to SI base units.
    pub fn convert_to_si(self, val: f64) -> f64 {
        (val + self.offset) * self.quantity.value
    }
}

/// All available units in the system.
pub const UNITS: &[Unit] = &[
    Unit::new("rad", 1.0, ANGLE, 0.0),
    Unit::new("deg", std::f64::consts::PI / 180.0, ANGLE, 0.0),
    Unit::new("m", 1.0, LENGTH, 0.0),
    Unit::new("cm", 0.01, LENGTH, 0.0),
    Unit::new("mm", 0.001, LENGTH, 0.0),
    Unit::new("um", 1e-6, LENGTH, 0.0),
    Unit::new("nm", 1e-9, LENGTH, 0.0),
    Unit::new("km", 1000.0, LENGTH, 0.0),
    Unit::new("kg", 1.0, MASS, 0.0),
    Unit::new("g", 0.001, MASS, 0.0),
    Unit::new("mg", 1e-6, MASS, 0.0),
    Unit::new("s", 1.0, TIME, 0.0),
    Unit::new("ms", 0.001, TIME, 0.0),
    Unit::new("us", 1e-6, TIME, 0.0),
    Unit::new("ns", 1e-9, TIME, 0.0),
    Unit::new("min", 60.0, TIME, 0.0),
    Unit::new("h", 3600.0, TIME, 0.0),
    Unit::new("Wh", 3600.0, ENERGY, 0.0),
    Unit::new("kWh", 3_600_000.0, ENERGY, 0.0),
    Unit::new("kmh", 1000.0 / 3600.0, VELOCITY, 0.0),
    Unit::new("A", 1.0, CURRENT, 0.0),
    Unit::new("mA", 0.001, CURRENT, 0.0),
    Unit::new("K", 1.0, TEMPERATURE, 0.0),
    Unit::new("degC", 1.0, TEMPERATURE, 273.15),
    Unit::new("mol", 1.0, AMOUNT_OF_SUBSTANCE, 0.0),
    Unit::new("cd", 1.0, LUMINOUS_INTENSITY, 0.0),
    Unit::new("l", 0.001, VOLUME, 0.0),
    Unit::new("ha", 10000.0, AREA, 0.0),
    Unit::new("bar", 100_000.0, PRESSURE, 0.0),
    Unit::new("atm", 101_325.0, PRESSURE, 0.0),
    Unit::new("inch", 0.0254, LENGTH, 0.0),
    Unit::new("ft", 0.3048, LENGTH, 0.0),
    Unit::new("eV", 1.602_176_634e-19, ENERGY, 0.0),
    Unit::new("kJ", 1000.0, ENERGY, 0.0),
    Unit::new("MJ", 1_000_000.0, ENERGY, 0.0),
    Unit::new("N", 1.0, FORCE, 0.0),
    Unit::new("J", 1.0, ENERGY, 0.0),
    Unit::new("W", 1.0, POWER, 0.0),
    Unit::new("mW", 0.001, POWER, 0.0),
    Unit::new("kW", 1000.0, POWER, 0.0),
    Unit::new("MW", 1_000_000.0, POWER, 0.0),
    Unit::new("Pa", 1.0, PRESSURE, 0.0),
    Unit::new("hPa", 100.0, PRESSURE, 0.0),
    Unit::new("kPa", 1000.0, PRESSURE, 0.0),
    Unit::new("MPa", 1_000_000.0, PRESSURE, 0.0),
    Unit::new("Hz", 1.0, FREQUENCY, 0.0),
];

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_scalar() {
            return write!(f, "{}", self.value);
        }

        for unit in UNITS {
            if unit.quantity.value == 1.0 && self.dims == unit.quantity.dims && unit.offset == 0.0 {
                return write!(f, "{} {}", self.value, unit.name);
            }
        }

        write!(f, "{} {}", self.value, self.dims)
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
