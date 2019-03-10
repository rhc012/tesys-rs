use std::sync::Mutex;

extern crate colored;
pub use self::colored::*;

lazy_static! {
    //static ref OUTFILE: String = "index.csv".to_string();
    pub static ref LOGGABLE_MTX: Mutex<i8> = Mutex::new(0);
}

#[allow(dead_code)]

pub trait Loggable {
    fn _loggable_ident() -> &'static str;
}

pub fn log(str: &str) {
    let _mtx = LOGGABLE_MTX.lock().unwrap();
    println!(
        "{}",
        format!("[ {} ] {}", "Info".magenta().bold(), str.green())
    );
}

pub fn warn(str: &str) {
    let _mtx = LOGGABLE_MTX.lock().unwrap();
    println!(
        "{}",
        format!("[ {} ] {}", "Warning".magenta().bold(), str.yellow())
    );
}

pub fn err(str: &str) {
    let _mtx = LOGGABLE_MTX.lock().unwrap();
    println!(
        "{}",
        format!("[ {} ] {}", "Error".magenta().bold(), str.red())
    );
}

pub fn log_labelled(str: &str, com: &str) {
    let _mtx = LOGGABLE_MTX.lock().unwrap();
    println!(
        "{}",
        format!(
            "[ {} : {} ] {}",
            com.white().bold(),
            "Info".magenta().bold(),
            str.green()
        )
    );
}

pub fn warn_labelled(str: &str, com: &str) {
    let _mtx = LOGGABLE_MTX.lock().unwrap();
    println!(
        "{}",
        format!(
            "[ {} : {} ] {}",
            com.white().bold(),
            "Warning".magenta().bold(),
            str.yellow()
        )
    );
}

pub fn err_labelled(str: &str, com: &str) {
    let _mtx = LOGGABLE_MTX.lock().unwrap();
    println!(
        "{}",
        format!(
            "[ {} : {} ] {}",
            com.white().bold(),
            "Error".magenta().bold(),
            str.red()
        )
    );
}

#[macro_export]
macro_rules! tesys_log {
    ($self:ident, $ ( $ arg : tt ) *) => {
        loggable::log_labelled(&format!( $ ( $ arg ) * ), $self::_loggable_ident() );
    };
    ($ ( $ arg : tt ) *) => {
        loggable::log(&format!( $ ( $ arg ) * ));
    };
}

#[macro_export]
macro_rules! tesys_warn {
    ($self:ident, $ ( $ arg : tt ) *) => {
        loggable::warn_labelled(&format!( $ ( $ arg ) * ), $self::_loggable_ident() );
    };
    ($ ( $ arg : tt ) *) => {
        loggable::warn(&format!( $ ( $ arg ) * ));
    };
}

#[macro_export]
macro_rules! tesys_err {
    ($self:ident, $ ( $ arg : tt ) *) => {
        loggable::err_labelled(&format!( $ ( $ arg ) * ), $self::_loggable_ident() );
    };
    ($ ( $ arg : tt ) *) => {
        loggable::err(&format!( $ ( $ arg ) * ));
    };
}

// rhc 20190218
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_log() {
        log("log works"); // doesn't actually produce any output, but the test passes
    }
}
