use crate::chrono::{DateTime, Local, TimeZone, Utc};
use crate::frame::CanTransformTo;
use crate::frame::Frame;
use crate::Angle;
use crate::AzEl;
use crate::CoordinateTransform;
use crate::Epoch;
use crate::Location;
use crate::ProperMotion;
use crate::SkyCoordinate;

use crate::datetime::*;
use std::f64;
use std::fmt;

use crate::frames::fk5::FK5;

#[derive(Clone, Debug)]
pub struct ICRS {
    equinox: DateTime<Utc>,
}

impl SkyCoordinate<ICRS> {
    pub fn new(_ra_deg: f64, _dec_deg: f64) -> SkyCoordinate<ICRS> {
        let ra = Angle::new(_ra_deg);
        let dec = Angle::new(_dec_deg);
        SkyCoordinate {
            coords: vec![ra, dec],
            _pm: ProperMotion::new(0., 0.),
            _epoch: Epoch::now(),
            _frame: ICRS::new(),
        }
    }

    pub fn with_epoch(mut self, _epoch: Epoch) -> Self {
        self._epoch = _epoch;
        self
    }

    pub fn with_proper_motion(mut self, _pm: ProperMotion) -> Self {
        self._pm = _pm;
        self
    }

    pub fn to_current_sky_position(&self, _loc: Location) -> AzEl {
        let dt = Local::now();
        self.to_sky_position(dt, _loc)
    }

    pub fn to_epoch(&self, _ep: Epoch) -> SkyCoordinate<ICRS> {
        let ra = self.coords[0].clone();
        let dec = self.coords[1].clone();
        let dtyr = -((self._epoch.clone() - _ep.clone()).num_milliseconds() as f64 / 1000.0)
            / SECONDS_PER_SIDEREAL_YEAR;
        let coords = vec![
            ra + (self._pm.pmra.clone() * dtyr),
            dec + (self._pm.pmdec.clone() * dtyr),
        ];
        SkyCoordinate {
            coords: coords,
            _pm: self._pm.clone(),
            _epoch: _ep,
            _frame: ICRS::new(),
        }
    }

    pub fn to_sky_position(&self, _dt: DateTime<Local>, _loc: Location) -> AzEl {
        let epoch = Epoch::new(_dt.with_timezone(&Utc));
        let tmpcoord = self.to_epoch(epoch);
        let mut ha =
            get_sidereal_time(_dt.with_timezone(&Utc), _loc.clone()) - tmpcoord.coords[0].clone();
        ha = if ha < Angle::new(0.0) {
            ha + 2.0 * f64::consts::PI
        } else {
            ha
        };
        ha = if ha > Angle::new(360.0) {
            ha - 2.0 * f64::consts::PI
        } else {
            ha
        };
        println!("HA: {}", ha.to_hms());

        let az = Angle::new_from_atan(
            ha.sin() / (ha.cos() * _loc.lat.sin() - tmpcoord.coords[1].tan() * _loc.lat.cos()),
        );
        let el = Angle::new_from_asin(
            _loc.lat.sin() * tmpcoord.coords[1].sin()
                + _loc.lat.cos() * tmpcoord.coords[1].cos() * ha.cos(),
        );
        //let az = Angle::new(- ( - _loc.lat.sin() * tmpcoord.dec.cos() * ha.cos()).atan2(tmpcoord.dec.cos() * ha.sin() ));

        AzEl { az: az, el: el }
    }

    pub fn epoch(&self) -> Epoch {
        self._epoch.clone()
    }

    pub fn ra(&self) -> Angle {
        self.coords[0].clone()
    }

    pub fn dec(&self) -> Angle {
        self.coords[1].clone()
    }

    pub fn set_ra(&mut self, _an: Angle) {
        self.coords[0] = _an;
    }

    pub fn set_dec(&mut self, _an: Angle) {
        self.coords[1] = _an;
    }
}

impl Frame for ICRS {
    fn new() -> Self {
        ICRS {
            equinox: Utc.ymd(2000, 1, 1).and_hms(11, 59, 28),
        }
    }
}

impl fmt::Display for SkyCoordinate<ICRS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ra_hms = self.coords[0].to_hms();
        let dec_dms = self.coords[1].to_dms();
        let ra_neg_str = if ra_hms.neg { "-" } else { "" };
        let dec_neg_str = if dec_dms.neg { "-" } else { "" };
        write!(
            f,
            "(ICRS: {}): RA = {}{:0>2}:{:0>2}:{:2.2}, DEC = {}{:0>2}:{:0>2}:{:2.2}",
            self.epoch(),
            ra_neg_str,
            ra_hms.h,
            ra_hms.m,
            ra_hms.s,
            dec_neg_str,
            dec_dms.d,
            dec_dms.m,
            dec_dms.s
        )
    }
}

impl CanTransformTo<FK5> for SkyCoordinate<ICRS> {
    type From = ICRS;

    fn transform(self) -> SkyCoordinate<FK5> {
        SkyCoordinate {
            coords: self.coords,
            _pm: self._pm,
            _epoch: self._epoch,
            _frame: FK5::new(),
        }
    }
    fn transform_to(&self, _target: FK5) -> CoordinateTransform<ICRS, FK5> {
        CoordinateTransform::<Self::From, FK5>::new(self.clone())
    }
}

impl CoordinateTransform<ICRS, FK5> {
    pub fn finish(&self) -> SkyCoordinate<FK5> {
        let co = self.coords.clone();
        let ret: SkyCoordinate<FK5> = co.transform();
        ret
    }
}
