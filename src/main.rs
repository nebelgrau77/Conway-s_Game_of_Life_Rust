#![no_std]
#![no_main]

use edgebadge as hal;
use panic_halt as _;

use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::prelude::*;
use embedded_graphics::{egrectangle, primitive_style};
use embedded_graphics::fonts::{Font6x8, Font12x16, Text};
use embedded_graphics::style::TextStyle;
use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::delay::Delay;
use hal::prelude::*;

use rand::prelude::*;

use arrayvec::ArrayString;

const BOOT_DELAY_MS: u16 = 100; 

mod game;

use game::*;


#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT).split();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    // short delay to allow the bus to start correctly
    delay.delay_ms(BOOT_DELAY_MS); 

    let (mut display, _backlight) = pins
        .display
        .init(
            &mut clocks,
            peripherals.SERCOM4,
            &mut peripherals.MCLK,
            peripherals.TC2,
            &mut delay,
            &mut pins.port,
        )
        .unwrap();

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (160, 128),
        style = primitive_style!(stroke_width = 0, fill_color = RgbColor::BLACK)
    )
    .draw(&mut display)
    .unwrap();

    let seed = 0xdead_beef_cafe_d00d;

    let mut rng = SmallRng::seed_from_u64(seed);

    loop {

        let mut buffer = [0u8; 512]; // 64/8 = 16 bytes * 64 lines 
        
        let text_style_counter = TextStyle::new(Font6x8, Rgb565::new(0,255,0));

        let text_style_title = TextStyle::new(Font12x16, Rgb565::new(0,255,0));
        
        let title = "Game of Life";

        let mut text_buf = ArrayString::<[u8; 8]>::new();

        let mut gen: u16 = 0; // generation counter
        
        Text::new(title, Point::new(8, 80)).into_styled(text_style_title).draw(&mut display).unwrap();

        Text::new(text_buf.as_str(), Point::new(80, 0)).into_styled(text_style_counter).draw(&mut display).unwrap();

        // generate first random grid

        rng.fill_bytes(&mut buffer);
        
        // display it
        
        for x in 0..game::WX {
            for y in 0..game::HY {
                let pixel = pixelgetter(x,y,buffer);
                
                // as the value is 0 or 1 it's enough to multiply by the "on" color value
                display.set_pixel(x as u16,y as u16,pixel.value as u16 * 2016_u16).unwrap(); 
            }
        }

        counter(&mut text_buf, gen);

        Text::new(text_buf.as_str(), Point::new(80, 0)).into_styled(text_style_counter).draw(&mut display).unwrap();



        while gen < 1000 {
    
            
            // clean up the number area of the counter
    
            for m in 100..128 {
                for n in 0..8 {
                    display.set_pixel(m, n, 0).unwrap();
                }
            }
    
            let mut text_buf = ArrayString::<[u8; 8]>::new();
            
            gen += 1;

            counter(&mut text_buf, gen);
            
            Text::new(text_buf.as_str(), Point::new(80, 0)).into_styled(text_style_counter).draw(&mut display).unwrap();

            buffer = matrix_evo(buffer);
    
            for x in 0..WX {
                for y in 0..HY {
                    let pixel = pixelgetter(x,y,buffer);
                    display.set_pixel(x as u16,y as u16,pixel.value as u16 * 2016_u16).unwrap(); 
                    }
                }
          
            }

        }

}