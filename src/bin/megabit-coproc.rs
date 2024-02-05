#![no_std]
#![no_main]

use core::convert::Infallible;

use defmt_rtt as _;
use embassy_nrf::{
    bind_interrupts,
    gpio::{Level, Output, OutputDrive},
    peripherals, spim,
};
use embassy_time::Timer;
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

    let mut spim = spim::Spim::new_txonly(
        nrf_peripherals.SPI3,
        Irqs,
        nrf_peripherals.P0_13,
        nrf_peripherals.P1_01,
        config,
    );
    let mut ncs_0 = Output::new(nrf_peripherals.P0_27, Level::High, OutputDrive::Standard);
    let mut ncs_1 = Output::new(nrf_peripherals.P0_21, Level::High, OutputDrive::Standard);

    // ncs_0.set_low();
    // spim.transfer(&mut [], &[0xFF]).await.unwrap();
    // ncs_0.set_high();
    // ncs_1.set_low();
    // spim.transfer(&mut [], &[0x80]).await.unwrap();
    // ncs_1.set_high();

    let mut cmd_data = [0u8; 8];
    for ncs in [
        &mut ncs_0 as &mut dyn embedded_hal::digital::OutputPin<Error = Infallible>,
        &mut ncs_1,
    ] {
        // Disable display test
        ncs.set_low();
        spim.transfer(&mut [], &[0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00])
            .await
            .unwrap();
        ncs.set_high();
        // Set scan limit to max (7)
        ncs.set_low();
        spim.transfer(&mut [], &[0x0b, 0x07, 0x0b, 0x07, 0x0b, 0x07, 0x0b, 0x07])
            .await
            .unwrap();
        ncs.set_high();
        // Disable decode mode
        ncs.set_low();
        spim.transfer(&mut [], &[0x09, 0x00, 0x09, 0x00, 0x09, 0x00, 0x09, 0x00])
            .await
            .unwrap();
        ncs.set_high();
        // Set the brightness to low
        ncs.set_low();
        spim.transfer(&mut [], &[0x0a, 0x03, 0x0a, 0x03, 0x0a, 0x03, 0x0a, 0x03])
            .await
            .unwrap();
        ncs.set_high();
        // Disable shutdown mode
        ncs.set_low();
        spim.transfer(&mut [], &[0x0c, 0x01, 0x0c, 0x01, 0x0c, 0x01, 0x0c, 0x01])
            .await
            .unwrap();
        ncs.set_high();

        for row in 0..8 {
            ncs.set_low();
            spim.transfer(
                &mut [],
                &[row + 1, 0x00, row + 1, 0x00, row + 1, 0x00, row + 1, 0x00],
            )
            .await
            .unwrap();
            ncs.set_high();
        }

        for row in 0..8 {
            for (a, b) in [(6, 7), (4, 5), (2, 3), (0, 1)] {
                for col in 0..8 {
                    cmd_data[a] = (7 - row as u8) + 1;
                    cmd_data[b] = 1 << col;

                    ncs.set_low();
                    spim.transfer(&mut [], &cmd_data).await.unwrap();
                    ncs.set_high();
                    Timer::after_millis(50).await;
                }

                led.set_level(match led.get_output_level() {
                    Level::High => Level::Low,
                    Level::Low => Level::High,
                });

                cmd_data[b] = 0x00;
                ncs.set_low();
                spim.transfer(&mut [], &cmd_data).await.unwrap();
                ncs.set_high();
            }
        }
    }

    loop {
        led.set_high();
        Timer::after_millis(300).await;
        led.set_low();
        Timer::after_millis(300).await;
    }
}
