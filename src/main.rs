#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::Drawable;
use embedded_graphics::{image::{Image, ImageRaw}, pixelcolor::BinaryColor, prelude::Point};
use panic_halt as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};

// struct I2CInterfaceWrapper(OLED);
// type OLED = BlockingI2c<
//     I2C1,
//     (
//         Pin<'B', 8, Alternate<OpenDrain>>,
//         Pin<'B', 9, Alternate<OpenDrain>>,
//     ),
// >;
// #[derive(PartialEq, Eq, Clone, Debug, Copy)]
// struct I2CError {}
// impl i2c::Error for I2CError {
//     fn kind(&self) -> i2c::ErrorKind {
//         i2c::ErrorKind::Other
//     }
// }
// impl ErrorType for I2CInterfaceWrapper {
//     type Error = I2CError;
// }
// impl I2c for I2CInterfaceWrapper {
//     fn transaction(
//         &mut self,
//         _address: u8,
//         _operations: &mut [Operation<'_>],
//     ) -> Result<(), Self::Error> {
//         Ok(())
//     }
// }

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();

    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.Hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

    // use ssd1306::test_helpers::I2cStub;
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
    let im = Image::new(&raw, Point::new(32, 0));
    im.draw(&mut display).unwrap();
    display.flush().unwrap();

    loop {}
}
