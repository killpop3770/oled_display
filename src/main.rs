#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::prelude::{Primitive, Size};
use embedded_graphics::primitives::PrimitiveStyleBuilder;
use embedded_graphics::primitives::{Circle, Rectangle, Triangle};
use embedded_graphics::text::{Baseline, Text};
use embedded_graphics::Drawable;
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::Point,
};
use panic_halt as _;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};

// STM32F1XX-HAL WORK ONLY WITH SSD1306 LIB VERSION 0.8.0 !!!
// LIB VERSION HIGHER THAN 0.8.0 DOES NOT WORK, BECAUSE
// TRAIT POLICY IN SSD1306 IS CHANGED FROM BLOCKING TO ASYNC VERSION OF I2C OBJECT
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

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // display.init().unwrap();
    // let raw: ImageRaw<BinaryColor> = ImageRaw::new(include_bytes!("./rust.raw"), 64);
    // let im = Image::new(&raw, Point::new(32, 0));
    // im.draw(&mut display).unwrap();
    // display.flush().unwrap();

    // display.init().unwrap();

    // let y_offset = 20;

    // let style = PrimitiveStyleBuilder::new()
    //     .stroke_width(1)
    //     .stroke_color(BinaryColor::On)
    //     .build();

    // // screen outline
    // // default display size is 128x64 if you don't pass a _DisplaySize_
    // // enum to the _Builder_ struct
    // Rectangle::new(Point::new(0, 0), Size::new(127, 63))
    //     .into_styled(style)
    //     .draw(&mut display)
    //     .unwrap();

    // // triangle
    // Triangle::new(
    //     Point::new(16, 16 + y_offset),
    //     Point::new(16 + 16, 16 + y_offset),
    //     Point::new(16 + 8, y_offset),
    // )
    // .into_styled(style)
    // .draw(&mut display)
    // .unwrap();

    // // square
    // Rectangle::new(Point::new(52, y_offset), Size::new_equal(16))
    //     .into_styled(style)
    //     .draw(&mut display)
    //     .unwrap();

    // // circle
    // Circle::new(Point::new(88, y_offset), 16)
    //     .into_styled(style)
    //     .draw(&mut display)
    //     .unwrap();

    // display.flush().unwrap();

    // display.init().unwrap();

    // let y_offset = 16;

    // let text_style = MonoTextStyleBuilder::new()
    //     .font(&FONT_6X10)
    //     .text_color(BinaryColor::On)
    //     .build();

    // Text::with_baseline("=============", Point::new(0, 0), text_style, Baseline::Top)
    //     .draw(&mut display)
    //     .unwrap();

    // Text::with_baseline(
    //     "Hello world!",
    //     Point::new(0, y_offset),
    //     text_style,
    //     Baseline::Top,
    // )
    // .draw(&mut display)
    // .unwrap();

    // Text::with_baseline(
    //     "=============",
    //     Point::new(0, y_offset * 2),
    //     text_style,
    //     Baseline::Top,
    // )
    // .draw(&mut display)
    // .unwrap();
    // display.flush().unwrap();
    // loop {}

    // using noise https://en.wikipedia.org/wiki/Hexspeak
    display.init().unwrap();
    let mut buf = [0x00u8; 1024];
    let mut rng = SmallRng::seed_from_u64(0xdead_beef_cafe_d00d);
    loop {
        rng.fill_bytes(&mut buf);
        
        display.draw(&buf).unwrap();
    }

}
