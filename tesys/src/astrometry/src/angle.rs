use crate::dms;
use crate::hms;
use std::clone::Clone;
use std::fmt;

use std::cmp::Ordering;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::ARCSEC_PER_DEG;
use crate::DEG_PER_RAD;
use crate::MAS_PER_DEG;
use crate::MAS_PER_RAD;

/// A library for manipulating angles.
/// Angles are stored in radians
/// # Examples
/// ```
/// use crate Angle;
/// use angle::Angle;
/// let a: Angle = Angle::new(45.0);
/// assert_eq!(45.0, a.deg());
/// ```
pub struct Angle {
    _angle: f64, // Measured in radians
}

impl Angle {
    pub fn new(val: f64) -> Angle {
        // Creates a new instance of the object given an angle in degrees.
        Angle {
            _angle: val / DEG_PER_RAD,
        }
    }

    pub fn new_from_arcsec(val: f64) -> Angle {
        //! Creates a new instance of the object given an angle in arcseconds.
        Angle::new(val / ARCSEC_PER_DEG)
    }

    pub fn new_from_mas(val: f64) -> Angle {
        //! Creates a new instance of the object given an angle in arcseconds.
        Angle::new(val / (MAS_PER_DEG))
    }

    pub fn new_from_hms(val: hms::HMS) -> Angle {
        // Creates a new instance of the object given an angle in degrees.
        Angle { _angle: val.into() }
    }

    pub fn new_from_asin(val: f64) -> Angle {
        Angle { _angle: val.asin() }
    }

    pub fn new_from_acos(val: f64) -> Angle {
        Angle { _angle: val.acos() }
    }

    pub fn new_from_atan(val: f64) -> Angle {
        Angle { _angle: val.atan() }
    }

    pub fn set(&mut self, val: f64) {
        self._angle = val;
    }

    pub fn to_hms(&self) -> hms::HMS {
        let hms = hms::HMS::new_from_rad(&self._angle);
        return hms;
    }

    pub fn to_dms(&self) -> dms::DMS {
        let dms = dms::DMS::new_from_rad(&self._angle);
        return dms;
    }

    pub fn deg(&self) -> f64 {
        self._angle * DEG_PER_RAD
    }

    pub fn mas(&self) -> f64 {
        self._angle * MAS_PER_RAD
    }

    pub fn sin(&self) -> f64 {
        self._angle.sin()
    }

    pub fn cos(&self) -> f64 {
        self._angle.cos()
    }

    pub fn tan(&self) -> f64 {
        self._angle.tan()
    }
}

impl fmt::Debug for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:2.2} rad", self._angle)
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:2.2}°", self._angle * DEG_PER_RAD)
    }
}

impl Into<hms::HMS> for Angle {
    fn into(self) -> hms::HMS {
        return self.to_hms();
    }
}

impl Into<dms::DMS> for Angle {
    fn into(self) -> dms::DMS {
        return self.to_dms();
    }
}

impl Into<f64> for Angle {
    fn into(self) -> f64 {
        self._angle
    }
}

impl Into<i8> for Angle {
    fn into(self) -> i8 {
        self._angle as i8
    }
}

impl From<i8> for Angle {
    fn from(val: i8) -> Self {
        Angle::new(val as f64)
    }
}

impl From<i32> for Angle {
    fn from(val: i32) -> Self {
        Angle::new(val as f64)
    }
}

impl From<f64> for Angle {
    fn from(val: f64) -> Self {
        Angle::new(val)
    }
}

/* Operations */
impl AddAssign<Angle> for Angle {
    fn add_assign(&mut self, rhs: Angle) {
        self._angle = self._angle + rhs._angle;
    }
}

impl AddAssign<f64> for Angle {
    fn add_assign(&mut self, rhs: f64) {
        self._angle = self._angle + rhs;
    }
}

impl AddAssign<i32> for Angle {
    fn add_assign(&mut self, rhs: i32) {
        self._angle = self._angle + (rhs as f64);
    }
}

impl SubAssign<Angle> for Angle {
    fn sub_assign(&mut self, rhs: Angle) {
        self._angle = self._angle - rhs._angle;
    }
}

impl SubAssign<f64> for Angle {
    fn sub_assign(&mut self, rhs: f64) {
        self._angle = self._angle - rhs;
    }
}

impl SubAssign<i32> for Angle {
    fn sub_assign(&mut self, rhs: i32) {
        self._angle = self._angle - (rhs as f64);
    }
}

impl Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Angle {
        Angle {
            _angle: self._angle + rhs._angle,
        }
    }
}

impl Add<f64> for Angle {
    type Output = Angle;

    fn add(self, rhs: f64) -> Angle {
        Angle {
            _angle: self._angle + rhs,
        }
    }
}

impl Add<i32> for Angle {
    type Output = Angle;

    fn add(self, rhs: i32) -> Angle {
        Angle {
            _angle: self._angle + (rhs as f64),
        }
    }
}

impl Sub<Angle> for Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Angle {
        Angle {
            _angle: self._angle - rhs._angle,
        }
    }
}

impl Sub<f64> for Angle {
    type Output = Angle;

    fn sub(self, rhs: f64) -> Angle {
        Angle {
            _angle: self._angle - rhs,
        }
    }
}

impl Sub<i32> for Angle {
    type Output = Angle;

    fn sub(self, rhs: i32) -> Angle {
        Angle {
            _angle: self._angle - (rhs as f64),
        }
    }
}

impl Mul<f64> for Angle {
    type Output = Angle;

    fn mul(self, rhs: f64) -> Angle {
        Angle {
            _angle: self._angle * rhs,
        }
    }
}

impl Mul<i32> for Angle {
    type Output = Angle;

    fn mul(self, rhs: i32) -> Angle {
        Angle {
            _angle: self._angle * (rhs as f64),
        }
    }
}

impl Div<f64> for Angle {
    type Output = Angle;

    fn div(self, rhs: f64) -> Angle {
        Angle {
            _angle: self._angle / rhs,
        }
    }
}

impl Div<i32> for Angle {
    type Output = Angle;

    fn div(self, rhs: i32) -> Angle {
        Angle {
            _angle: self._angle / (rhs as f64),
        }
    }
}

impl Rem<Angle> for Angle {
    type Output = Angle;

    fn rem(self, rhs: Angle) -> Self {
        Angle::new(self._angle % rhs._angle)
    }
}

impl Rem<f64> for Angle {
    type Output = Angle;

    fn rem(self, rhs: f64) -> Self {
        Angle::new(self._angle % rhs)
    }
}

impl Rem<i32> for Angle {
    type Output = Angle;

    fn rem(self, rhs: i32) -> Self {
        Angle::new(self._angle % rhs as f64)
    }
}

impl Clone for Angle {
    fn clone(&self) -> Angle {
        Angle {
            _angle: self._angle,
        }
    }
}

impl Eq for Angle {}

impl PartialEq for Angle {
    fn eq(&self, other: &Angle) -> bool {
        self._angle == other._angle
    }
}

impl Ord for Angle {
    fn cmp(&self, other: &Angle) -> Ordering {
        self._angle.partial_cmp(&other._angle).unwrap()
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
