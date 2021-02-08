#![no_main]
#![no_std]

use panic_halt as _;

use nrf52840_hal as hal;

use hal::pac::{CorePeripherals, Peripherals};
use hal::{prelude::*,
          rng,
          delay::Delay,
          Twim,          
        };

use cortex_m_rt::entry;

use ssd1306::{prelude::*, Builder, I2CDIBuilder};

const BOOT_DELAY_MS: u16 = 100; //small delay for the I2C to initiate correctly and start on boot without having to reset the board

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
    };

use arrayvec::ArrayString;

mod game;
use game::*;

static WX: i16 = 64; // grid width
static HY: i16 = 64; // grid height

#[entry]
fn main() -> ! {
    
    let p = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();

    // set up GPIO ports
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    // define I2C pins
    let scl = port0.p0_02.into_floating_input().degrade(); // clock
    let sda = port0.p0_31.into_floating_input().degrade(); // data

    let pins = hal::twim::Pins{
        scl: scl,
        sda: sda
    };    

    // initialize a delay provider
    let mut delay = Delay::new(core.SYST);
    
    // wait for just a moment
    delay.delay_ms(BOOT_DELAY_MS);

    // set up I2C    
    let i2c = Twim::new(p.TWIM0, pins, hal::twim::Frequency::K400);

    // set up SSD1306 display
    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_> = Builder::new().connect(interface).into();          
    disp.init().unwrap();

    // initialize Random Numbers Generator
    let mut rng = rng::Rng::new(p.RNG);
    
    loop {
                        
        let mut buffer = [0u8; 512];
                    
        let text_style = TextStyleBuilder::new(Font6x8).text_color(BinaryColor::On).build();

        let mut text_buf = ArrayString::<[u8; 8]>::new();

        let mut gen: u16 = 0; // generation counter

        // generate first random grid

        rng.random(&mut buffer);
        
        // display it
        
        for x in 0..WX {
            for y in 0..HY {
                let pixel = pixelgetter(x,y,buffer);
                disp.set_pixel(x as u32,y as u32,pixel.value);
            }
        }

        counter(&mut text_buf, gen);

        Text::new(text_buf.as_str(), Point::new(80, 0)).into_styled(text_style).draw(&mut disp).unwrap();

        disp.flush().unwrap();
        
        while gen < 1000 {
    
            // clean up the number area of the counter
    
            for m in 100..128 {
                for n in 0..8 {
                    disp.set_pixel(m, n, 0);
                }
            }
    
            let mut text_buf = ArrayString::<[u8; 8]>::new();
            
            gen += 1;

            counter(&mut text_buf, gen);
            
            Text::new(text_buf.as_str(), Point::new(80, 0)).into_styled(text_style).draw(&mut disp).unwrap();
                    
            buffer = matrix_evo(buffer);
    
            for x in 0..WX {
                for y in 0..HY {
                    let pixel = pixelgetter(x,y,buffer);
                    disp.set_pixel(x as u32,y as u32,pixel.value);
                    }
                }
          
            disp.flush().unwrap();
    
            }

        }

    }

