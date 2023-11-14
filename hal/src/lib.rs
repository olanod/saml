#![no_std]
use core::convert::Infallible;

use embedded_hal::digital;
use embedded_hal_async::{delay::DelayUs, digital::Wait, i2c};
// use saml11_pac::Peripherals;

pub fn init() {
    todo!()
}

pub struct Pin;

impl digital::InputPin for Pin {
    fn is_high(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        todo!()
    }
}
impl digital::OutputPin for Pin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}
impl digital::StatefulOutputPin for Pin {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        todo!()
    }
}
impl digital::ToggleableOutputPin for Pin {
    fn toggle(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl Wait for Pin {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl digital::ErrorType for Pin {
    type Error = Infallible;
}

pub struct Timer;

impl DelayUs for Timer {
    async fn delay_us(&mut self, _us: u32) {
        todo!()
    }

    async fn delay_ms(&mut self, _ms: u32) {
        todo!()
    }
}

pub struct I2c;

impl i2c::I2c for I2c {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                i2c::Operation::Read(buf) => self.do_read(address, buf).await,
                i2c::Operation::Write(buf) => self.do_write(address, buf).await,
            }
        }
        Ok(())
    }
}

impl i2c::ErrorType for I2c {
    type Error = Infallible;
}

impl I2c {
    async fn do_read(&mut self, _address: u8, _buf: &mut [u8]) {}

    async fn do_write(&self, _address: u8, _buf: &[u8]) {}
}
