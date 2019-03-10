use crate::Angle;

use std::fmt;

#[derive(Clone, Debug)]
pub struct ProperMotion {
    pub pmra: Angle, // radians per year .... rhc:  make these radians per second so internally all data is in SI?
    pub pmdec: Angle, // radians per year
}

impl ProperMotion {
    pub fn new(pmra: f64, pmdec: f64) -> ProperMotion {
        //! This produces a new ProperMotion struct.
        //! The arguments are the proper motion in RA and DEC, measured in milliarcseconds / yr.
        ProperMotion {
            pmra: Angle::new_from_mas(pmra),
            pmdec: Angle::new_from_mas(pmdec),
        }
    }
}

impl fmt::Display for ProperMotion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PMRA(mas/y) = {:0>2}, PMDEC(mas/y) = {:0>2}",
            self.pmra.mas(),
            self.pmdec.mas()
        )
    }
}
