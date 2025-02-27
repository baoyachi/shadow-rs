#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::main;
use log::info;

use shadow_rs::shadow;

shadow!(build);
#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let delay = Delay::new();

    info!("{}", build::VERSION);

    loop {
        info!("Hello world!");
        delay.delay_millis(500);
    }
}
