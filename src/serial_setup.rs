// use core::fmt;
// use core::ptr::addr_of_mut;
// use embedded_hal_nb::nb;
// use embedded_hal_nb::serial::{Error as SerialError, ErrorType, Read, Write};
// use embedded_io::{Read as EmbeddedIoRead, Write as EmbeddedIoWrite};
// use microbit::hal::uarte::{Instance, Uarte, UarteRx, UarteTx};

// static mut TX_BUF: [u8; 1] = [0; 1];
// static mut RX_BUF: [u8; 1] = [0; 1];

// pub struct UartePort<T: Instance>(UarteTx<T>, UarteRx<T>);

// impl<T: Instance> UartePort<T> {
//     pub fn new(serial: Uarte<T>) -> UartePort<T> {
//         let (tx, rx) = serial
//             .split(unsafe { &mut *addr_of_mut!(TX_BUF) }, unsafe {
//                 &mut *addr_of_mut!(RX_BUF)
//             })
//             .unwrap();
//         UartePort(tx, rx)
//     }
// }

// #[derive(Debug)]
// pub enum Error {
//     Other,
// }

// impl SerialError for Error {
//     fn kind(&self) -> embedded_hal_nb::serial::ErrorKind {
//         embedded_hal_nb::serial::ErrorKind::Other
//     }
// }

// impl<T: Instance> ErrorType for UartePort<T> {
//     type Error = Error;
// }

// impl<T: Instance> fmt::Write for UartePort<T> {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         for byte in s.bytes() {
//             nb::block!(self.write(byte)).map_err(|_| fmt::Error)?;
//         }
//         Ok(())
//     }
// }

// impl<T: Instance> Write<u8> for UartePort<T> {
//     fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
//         self.0
//             .write(&[word])
//             .map_err(|_| nb::Error::Other(Error::Other))?;
//         Ok(())
//     }

//     fn flush(&mut self) -> nb::Result<(), Self::Error> {
//         self.0.flush().map_err(|_| nb::Error::Other(Error::Other))
//     }
// }

// impl<T: Instance> Read<u8> for UartePort<T> {
//     fn read(&mut self) -> nb::Result<u8, Self::Error> {
//         let mut buffer = [0u8; 1];
//         match self.1.read(&mut buffer) {
//             Ok(1) => Ok(buffer[0]),
//             Ok(_) => Err(nb::Error::WouldBlock),
//             Err(_) => Err(nb::Error::Other(Error::Other)),
//         }
//     }
// }
use core::{fmt, ptr::addr_of_mut};
use microbit::hal::uarte::{Error, Instance, Uarte, UarteRx, UarteTx};

static mut TX_BUF: [u8; 1] = [0; 1];
static mut RX_BUF: [u8; 1] = [0; 1];

pub struct UartePort<T: Instance>(UarteTx<T>, UarteRx<T>);

impl<T: Instance> UartePort<T> {
    pub fn new(serial: Uarte<T>) -> UartePort<T> {
        let (tx, rx) = serial
            .split(unsafe { addr_of_mut!(TX_BUF).as_mut().unwrap() }, unsafe {
                addr_of_mut!(RX_BUF).as_mut().unwrap()
            })
            .unwrap();
        UartePort(tx, rx)
    }
}

impl<T: Instance> fmt::Write for UartePort<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write_str(s)
    }
}

impl<T: Instance> embedded_io::ErrorType for UartePort<T> {
    type Error = Error;
}

impl<T: Instance> embedded_io::Write for UartePort<T> {
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> {
        self.0.write(buffer)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.0.flush()
    }
}

impl<T: Instance> embedded_io::Read for UartePort<T> {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        self.1.read(buffer)
    }
}