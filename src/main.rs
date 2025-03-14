#![no_main]
#![no_std]

use cortex_m::interrupt;
use cortex_m_rt::entry;
use heapless::Vec;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
mod serial_setup;
use core::fmt::Write;
use embedded_io::Read;
use serial_setup::UartePort;

use microbit::{
    hal::uarte::{self, Baudrate, Parity},
    pac::UARTE0,
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
};
use embedded_hal::i2c::I2c;
use lsm303agr::{AccelOutputDataRate, AccelMode, Lsm303agr};

#[entry]
fn main() -> ! {
    // Main for getting i2c targets' id.
    // rtt_init_print!();
    // let board = microbit::Board::take().unwrap();
    // let mut i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    // let mut acc = [0];
    // let mut mag = [0];
    // // First write the address + register onto the bus, then read the chip's responses
    // i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_ID_REG], &mut acc).unwrap();
    // i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_ID_REG], &mut mag).unwrap();
    // rprintln!("The accelerometer chip's id is: {:#b}", acc[0]);
    // rprintln!("The magnetometer chip's id is: {:#b}", mag[0]);
    // loop {}

    // Main for using the lsm303agr driver
    // rtt_init_print!();
    // let board = microbit::Board::take().unwrap();

    // let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };

    // let mut sensor = Lsm303agr::new_with_i2c(i2c);
    // let mut delay = microbit::hal::Delay::new(board.SYST);
    // sensor.init().unwrap();
    // sensor
    //     .set_accel_mode_and_odr(&mut delay, AccelMode::Normal, AccelOutputDataRate::Hz50)
    //     .unwrap();
    // loop {
    //     if sensor.accel_status().unwrap().xyz_new_data() {
    //         let data = sensor.acceleration().unwrap();
    //         rprintln!(
    //             "Acceleration: x {} y {} z {}",
    //             data.x_mg(),
    //             data.y_mg(),
    //             data.z_mg()
    //         );
    //     }
    // }

    // My solution to the challenge
    rtt_init_print!();
    let take_board = microbit::Board::take();
    let board: microbit::Board;
    match take_board {
        Some(b) => {
            board = b;
        }
        None => {
            panic!("Error taking the board");
        }
    }

    // uart init
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // i2c init
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    let mut delay = microbit::hal::Delay::new(board.SYST);
    sensor.init().unwrap();

    // main loop
    let mut buffer: Vec<u8, 32> = Vec::new();
    loop {
        let mut buf: [u8; 1] = [0];
        serial.read_exact(&mut buf).unwrap();
        if buf[0] as char == '\r' || buffer.is_full() {
            let input_string = core::str::from_utf8(&buffer).unwrap();
            match input_string {
                "accelerometer" => {
                    write!(serial, "Accelerometer's data: \r\n");
                    let acceleration = sensor.acceleration().unwrap();
                    write!(serial, "x {}, y {}, z{}\n\r", acceleration.x_unscaled(), acceleration.y_unscaled(), acceleration.z_unscaled());
                    let temperature = sensor.temperature().unwrap();
                    write!(serial, "Temperature C: {}\n\r", temperature.degrees_celsius());

                }
                "magnetometer" => {
                    write!(serial, "Magnetometer's data: \r\n");
                    let mag_field = sensor.magnetic_field().unwrap();
                    write!(serial, "x {}, y {}, z{}\n\r", mag_field.x_raw(), mag_field.y_raw(), mag_field.y_raw());
                }
                _ => {
                    write!(serial, "Unkknown command. Please type either accelerometer or magnetometer");
                }
            }
            write!(serial, "\r\n");
            buffer.clear();
        } else {
            buffer.push(buf[0]);
        }
    }
}
