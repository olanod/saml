#![no_std]

use embedded_hal_async::{delay::DelayUs, digital::Wait, i2c};

struct I2c;

impl i2c::I2c for I2c {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                i2c::Operation::Read(buf) => self.read(address, buf).await?,
                i2c::Operation::Write(buf) => self.write(address, buf).await?,
            }
        }
    }
}

impl I2c {
    async fn read(address: u8, buf: &mut [u8]) {}

    async fn write(address: u8, buf: &[u8]) {}
}

struct Pin;

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

struct Timer;

impl DelayUs for Timer {
    async fn delay_us(&mut self, us: u32) {
        todo!()
    }

    async fn delay_ms(&mut self, ms: u32) {
        todo!()
    }
}
