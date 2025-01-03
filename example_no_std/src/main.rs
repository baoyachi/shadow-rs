#![no_std]
#![no_main]

use shadow_rs_consumer::shadow;

shadow!(build);

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[allow(clippy::all, clippy::pedantic, clippy::restriction, clippy::nursery)]
pub fn func() {
    log::info!("short_commit: {}", build::SHORT_COMMIT);
}
