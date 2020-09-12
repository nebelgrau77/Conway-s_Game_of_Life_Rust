#![no_std]
#![no_main]

extern crate arduino_nano33iot as hal;
extern crate ssd1306;
extern crate arrayvec;
extern crate rand;
extern crate embedded_graphics;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::time::KiloHertz;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

use ssd1306::{prelude::*, Builder as SSD1306Builder};

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
    };

use rand::prelude::*;

use arrayvec::ArrayString;

const BOOT_DELAY_MS: u16 = 100; 

mod game;

use game::*;

static WX: i16 = 32; // grid width
static HY: i16 = 32; // grid height

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);    
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let gclk0 = clocks.gclk0();

    //delay necessary for the I2C to initiate correctly and start on boot without having to reset the board
    delay.delay_ms(BOOT_DELAY_MS);


    let i2c = hal::i2c_master(
        &mut clocks,
        KiloHertz(100),
        peripherals.SERCOM4, 
        &mut peripherals.PM, 
        pins.sda,
        pins.scl,
        &mut pins.port,
    );  



    let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x32).connect_i2c(i2c).into();
       

    disp.init().unwrap();

    let seed = 0xdead_beef_cafe_d00d;

    let mut rng = SmallRng::seed_from_u64(seed);

    loop {
                        
        let mut buffer = [0u8; 512];
                    
        let text_style = TextStyleBuilder::new(Font6x8).text_color(BinaryColor::On).build();

        let mut text_buf = ArrayString::<[u8; 8]>::new();

        let mut gen: u16 = 0; // generation counter

        // generate first random grid

        rng.fill_bytes(&mut buffer);
        
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


