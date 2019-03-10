extern crate chrono;
#[macro_use]
extern crate tesys;
pub use tesys::loggable;

use chrono::prelude::*;
use chrono::Local;
use std::env;
use tesys::astrometry::datetime; //::*;
use tesys::astrometry::frame::CanTransformTo;
use tesys::astrometry::frames::{FK5, ICRS};
use tesys::astrometry::{Epoch, Frame, Location, ProperMotion, SkyCoordinate};
use tesys::Peer;
use tesys::net::{Message,Route};

fn main() -> Result<(), i32> {
    // Let's first check and see if we have a config file as a command line argument
    if env::args().count() <2 {
        loggable::err("Usage: tesys <config_file>");
        return Err(-1);
    }

    // We have one, so load it and away we go.
    let conf_file = env::args().nth(1).unwrap();
    loggable::log(&format!("Loading configuration: {}", conf_file));

    tesys::loggable::log("Starting Tesys...");

    // Test sky coordinate calculations for Vega
    let coord_icrs = SkyCoordinate::<ICRS>::new(279.23473479, 38.78368896)
        .with_epoch(Epoch::j2000())
        .with_proper_motion(ProperMotion::new(200.94, 286.23));
    let coord_fk5: SkyCoordinate<FK5> = coord_icrs.transform_to(FK5::new()).finish();
    loggable::log(&format!("Vega: {}", coord_icrs));
    loggable::log(&format!("Vega: {}", coord_fk5));
    let ang = coord_icrs.ra();
    loggable::log(&format!("Vega RA: {}", ang));

    // Test time calculations
    let dt = Local::now();
    loggable::log(&format!("Local time: {}", dt));  // rhc
    let loc = Location::new(50.73778, -3.535278); // Exeter campus
    loggable::log(&format!(
        "Modified Julian Date: {}",
        datetime::datetime_to_modified_julian_date(dt.with_timezone(&Utc))
    ));
    loggable::log(&format!(
        "Sidereal time (Exeter): {}",
        datetime::get_sidereal_time(dt.with_timezone(&Utc), loc.clone()).to_hms()
    ));
    loggable::log(&format!("A: {}", coord_icrs.to_sky_position(dt, loc.clone())));

    // Testing message
    let m = Message::new().with_payload(coord_icrs.clone()).finish();
    dbg!(m.clone());
    let coord1_icrs: SkyCoordinate<ICRS> = match m.get_payload() {
        Ok(p) => p,
        Err(_) => SkyCoordinate::<ICRS>::new(0., 0.),
    };
    tesys_warn!("B: {:?}", coord1_icrs);


    loggable::log("Initialising Peer...");
    let mut _p = Peer::new();
    println!("_p (Peer) {:?}", _p); // rhc 20190118
    _p.load_plugins();
    _p.run()
}
