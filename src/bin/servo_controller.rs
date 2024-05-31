#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::gpio::{AnyPin, Pin, Pull};
use embassy_stm32::gpio::{Input, Level, Output, Speed};
use embassy_stm32::rcc::SupplyConfig;
use embassy_stm32::time::Hertz;
use embassy_stm32::timer::simple_pwm::PwmPin;
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn led_1_task(led: AnyPin) {
    let mut led = Output::new(led, Level::High, Speed::Low);
    loop {
        Timer::after(Duration::from_millis(500)).await;
        led.toggle();
    }
}
#[embassy_executor::task]
async fn led_2_task(led: AnyPin) {
    let mut led = Output::new(led, Level::Low, Speed::Low);
    loop {
        Timer::after(Duration::from_millis(500)).await;
        led.toggle();
    }
}
#[embassy_executor::main]
async fn main(spawner: Spawner) {
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
    let pwm = PwmPin::new_ch3(p.PA2, OutputType::PushPull);
    spawner.spawn(led_1_task(p.PI12.degrade())).unwrap();
    spawner.spawn(led_2_task(p.PI13.degrade())).unwrap();
    info!("Hello World!");

    let mut led_3 = Output::new(p.PI14, Level::High, Speed::Low);

    let button_1 = Input::new(p.PK2, Pull::Up);
    let mut button_1 = ExtiInput::new(button_1, p.EXTI2);
    loop {
        button_1.wait_for_falling_edge().await;
        info!("Button fired");
        led_3.toggle();
    }
}
