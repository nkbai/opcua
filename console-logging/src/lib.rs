#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};

use env_logger::{Builder, Color};
use log::{Level, LevelFilter};

pub fn init() {
    lazy_static! {
        static ref INITIALISED: AtomicBool = AtomicBool::new(false);
    }

    // Only need to call this once
    if !INITIALISED.swap(true, Ordering::Relaxed) {
        // This is env_logger::init() but taking logging values from  instead of RUST_LOG.
        // env_logger/RUST_LOG is used by cargo and other rust tools so console fills with garbage from
        // other processes  when we're only interested in our own garbage!
        let mut builder = Builder::from_env("RUST_OPCUA_LOG");
        builder.filter_level(LevelFilter::Debug);
        builder.format(|buf, record| {
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

            writeln!(buf, "{} - {} - {} - {}-{} - {}", time_fmt, style.value(record.level()), record.target(),record.file().unwrap() ,record.line().unwrap(), record.args())
        });
//        builder.filter(None,LevelFilter::Debug); 强制log Leve,替换环境变量控制的
        builder.init();
        info!("Logging is enabled, use RUST_OPCUA_LOG environment variable to control filtering, logging level");
    }
}