#![no_std]
#![no_main]

use bl702_hal as hal;
use hal::{
    clock::{board_clock_init, system_init, ClockConfig},
    delay::McycleDelay,
    pac,
    prelude::*,
};
use embedded_hal_alpha::{delay::blocking::DelayMs, digital::blocking::OutputPin};

#[riscv_rt::entry]
fn main() -> ! {
    system_init();
    board_clock_init();
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    let clocks = ClockConfig::new().freeze(&mut parts.clk_cfg);

    let mut led_r = parts.pin25.into_pull_up_output();
    let mut led_g = parts.pin24.into_pull_up_output();
    let mut led_b = parts.pin23.into_pull_up_output();

    let mut d = McycleDelay::new(clocks.sysclk().0);

    loop {
        led_r.set_high().unwrap();
        led_g.set_high().unwrap();
        led_b.set_high().unwrap();
        d.delay_ms(1000).unwrap();

        led_r.set_low().unwrap();
        led_g.set_low().unwrap();
        led_b.set_low().unwrap();
        d.delay_ms(1000).unwrap();
    }
}
