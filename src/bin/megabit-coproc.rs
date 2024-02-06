#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Level, Output, OutputDrive},
    peripherals, spim,
};
use embassy_time::Timer;
use megabit_coproc_embassy::DotMatrix;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    SPIM3 => spim::InterruptHandler<peripherals::SPI3>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let nrf_peripherals = embassy_nrf::init(Default::default());
    let mut led = Output::new(nrf_peripherals.P0_06, Level::Low, OutputDrive::Standard);
    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M4;
    config.mode = spim::MODE_0;

    let spim = spim::Spim::new_txonly(
        nrf_peripherals.SPI3,
        Irqs,
        nrf_peripherals.P0_13,
        nrf_peripherals.P1_01,
        config,
    );
    let ncs_0 = Output::new(nrf_peripherals.P0_27, Level::High, OutputDrive::Standard);
    let ncs_1 = Output::new(nrf_peripherals.P0_21, Level::High, OutputDrive::Standard);

    let mut dot_matrix = DotMatrix::new(spim, ncs_0, ncs_1).await.unwrap();

    for row in 0..16 {
        for col in 0..32 {
            dot_matrix.set_pixel(row, col, true).await.unwrap();
            Timer::after_millis(100).await;
            dot_matrix.set_pixel(row, col, false).await.unwrap();
            Timer::after_millis(100).await;
        }
    }

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
