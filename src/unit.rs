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
        for i in 0..8 {
            new_dims[i] = (self.dims[i] as f64 * exp) as i8;
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
        for i in 0..8 {
            dims[i] = self.dims[i] + rhs.dims[i];
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
        for i in 0..8 {
            dims[i] = self.dims[i] - rhs.dims[i];
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

impl std::fmt::Display for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val_str = if self.value == 0.0 {
            "0".to_string()
        } else {
            format!("{}", self.value)
        };
        write!(f, "{}", val_str)?;
        let units = ["m", "kg", "s", "A", "K", "mol", "cd", "rad"];
        for i in 0..8 {
            if self.dims[i] != 0 {
                write!(f, " {}", units[i])?;
                if self.dims[i] != 1 {
                    write!(f, "^{}", self.dims[i])?;
                }
            }
        }
        Ok(())
    }
}
