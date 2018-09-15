extern crate env_logger;
extern crate chrono;
#[macro_use]
extern crate log;

use std::io::Write;

use env_logger::{Builder, Color};

pub fn init() {
    // This is env_logger::init() but taking logging values from  instead of RUST_LOG.
    // env_logger/RUST_LOG is used by cargo and other rust tools so console fills with garbage from
    // other processes  when we're only interested in our own garbage!
    let mut builder = Builder::from_env("RUST_OPCUA_LOG");
    builder.format(|buf, record| {
        use chrono;
        let now = chrono::Utc::now();
        let time_fmt = now.format("%Y-%m-%d %H:%M:%S%.3f");

        let mut style = buf.style();

        match record.metadata().level() {
            log::Level::Error => {
                // White on red
                style.set_color(Color::White);
                style.set_bg(Color::Red);
            }
            log::Level::Warn => {
                // Yellow on black
                style.set_color(Color::Yellow);
            }
            log::Level::Info => {
                // Blue on black
                style.set_color(Color::Cyan);
            }
            _ => {}
        }

        writeln!(buf, "{} - {} - {} - {}", time_fmt, style.value(record.level()), record.target(), record.args())
    });
    builder.init();

    info!("Logging is enabled, use RUST_OPCUA_LOG environment variable to control filtering, logging level");
}