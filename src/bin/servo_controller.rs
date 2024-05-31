#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::rcc::SupplyConfig;
use embassy_stm32::time::Hertz;
use embassy_stm32::Config;
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.supply_config = SupplyConfig::DirectSMPS;
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(25_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV5,
            mul: PllMul::MUL48,
            divp: Some(PllDiv::DIV2),
            divq: None,
            divr: None,
        });
        config.rcc.sys = Sysclk::PLL1_P; // 600 Mhz
        config.rcc.ahb_pre = AHBPrescaler::DIV1; // 300 Mhz
        config.rcc.apb1_pre = APBPrescaler::DIV1; // 150 Mhz
        config.rcc.apb2_pre = APBPrescaler::DIV1; // 150 Mhz
        config.rcc.apb4_pre = APBPrescaler::DIV1; // 150 Mhz
                                                  // config.rcc.apb5_pre = APBPrescaler::DIV2; // 150 Mhz
        config.rcc.voltage_scale = VoltageScale::Scale0;
    }
    let p = embassy_stm32::init(config);
    info!("Hello World!");

    let mut led = Output::new(p.PI12, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(500).await;

        info!("low");
        led.set_low();
        Timer::after_millis(500).await;
    }
}


