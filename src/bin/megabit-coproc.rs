#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let peripherals = embassy_nrf::init(Default::default());
    let mut led = Output::new(peripherals.P0_13, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
