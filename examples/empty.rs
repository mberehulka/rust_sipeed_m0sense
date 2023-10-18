#![no_std]
#![no_main]

use bl702_hal::{
    clock::{board_clock_init, system_init, ClockConfig},
    delay::McycleDelay,
    pac,
    prelude::*,
};
use embedded_hal_alpha::delay::blocking::DelayMs;

#[riscv_rt::entry]
fn main() -> ! {
    system_init();
    board_clock_init();
    let dp = pac::Peripherals::take().unwrap();
    let mut parts = dp.GLB.split();
    let clocks = ClockConfig::new().freeze(&mut parts.clk_cfg);

    let mut d = McycleDelay::new(clocks.sysclk().0);

    loop {
        d.delay_ms(1000).unwrap()
    }
}
