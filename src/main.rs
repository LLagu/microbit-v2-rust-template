#![no_std]
#![no_main]
#![deny(unsafe_code)]

extern crate microbit; // Remove once https://github.com/nrf-rs/microbit/issues/17 is solved
use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hoi!");
    loop {}
}