#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::prelude::*;
use log::info;


shadow!(build);
#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();

    info!("{}", build::VERSION);

    loop {
        info!("Hello world!");
        delay.delay(500.millis());
    }
}
