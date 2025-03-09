#![no_std]
#![no_main]
#![deny(unsafe_code)]

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};
// use embedded_hal::{delay::DelayNs, digital::OutputPin};
// use microbit::{board::Board, hal::timer::Timer};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hoi!");
    loop {}
}