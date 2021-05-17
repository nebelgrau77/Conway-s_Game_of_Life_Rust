// Will only work with Range5 or higher
// Needs Range6 or higher to work with 400kHz I2C frequency

//#![deny(warnings)]
//#![deny(unsafe_code)]

#![no_main]
#![no_std]

use panic_halt;
use cortex_m;
use cortex_m_rt::entry;
use stm32l4xx_hal::{
    delay::Delay,
    prelude::*,
    serial::{Config, Serial},
    i2c::I2c,
    hal::blocking::rng::Read,
    };
    
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

static WX: i16 = 64; // grid width
static HY: i16 = 64; // grid height

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    let clocks = rcc
        .cfgr
        .hsi48(true) // needed for RNG
        .sysclk(64.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);

    let mut delay = Delay::new(cp.SYST, clocks);

    //delay necessary for the I2C to initiate correctly and start on boot without having to reset the board
    delay.delay_ms(BOOT_DELAY_MS);

    let mut scl = gpioa.pa9.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut sda = gpioa.pa10.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1r1);
    let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x64).connect_i2c(i2c).into();
        
    disp.init().unwrap();
    

    // setup hardware rng
    let mut rng = dp.RNG.enable(&mut rcc.ahb2, clocks);

    //let seed: u32 = rng.get_random_data();

    //let mut soft_rng = SmallRng::seed_from_u64(0xdead_beef_cafe_d00d);



    loop {
                        
        let mut buffer = [0u8; 512];
                    
        let text_style = TextStyleBuilder::new(Font6x8).text_color(BinaryColor::On).build();

        let mut text_buf = ArrayString::<[u8; 8]>::new();

        let mut gen: u16 = 0; // generation counter

        // generate first random grid

        rng.read(&mut buffer).unwrap();

        //soft_rng.fill_bytes(&mut buffer);
        
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

