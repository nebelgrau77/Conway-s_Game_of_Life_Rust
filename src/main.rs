// Will only work with Range5 or higher
// Needs Range6 or higher to work with 400kHz I2C frequency

//#![deny(warnings)]
//#![deny(unsafe_code)]

#![no_main]
#![no_std]

use panic_halt;
use cortex_m;
use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, 
                    prelude::*, 
                    rcc::{Config,
                          MSIRange,
                          PLLDiv,
                          PLLMul,
                          PLLSource}};

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
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the clock.    
    let mut rcc = dp.RCC.freeze(Config::pll(PLLSource::HSI16, PLLMul::Mul3, PLLDiv::Div4)); // 24 MHz
    //let mut rcc = dp.RCC.freeze(Config::msi(MSIRange::Range6)); //works only with Range5 or Range6

    let mut delay = cp.SYST.delay(rcc.clocks);
    
    //delay necessary for the I2C to initiate correctly and start on boot without having to reset the board
    delay.delay_ms(BOOT_DELAY_MS);

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);

    // set up ADC for "random" seed reading from internal temperature and voltage
    let mut adc = dp.ADC.constrain(&mut rcc);
    
    let mut temperature = stm32l0xx_hal::adc::VTemp::new(); 
    let mut voltage = stm32l0xx_hal::adc::VRef::new();

    let temp: u16 = adc.read(&mut temperature).unwrap();
    let volt: u16 = adc.read(&mut voltage).unwrap();

    let seed: u64 = temp as u64 * volt as u64;

    let scl = gpioa.pa9.into_open_drain_output();
    let sda = gpioa.pa10.into_open_drain_output();
    
    let i2c = dp.I2C1.i2c(sda, scl, 100.khz(), &mut rcc); 

    let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x32).connect_i2c(i2c).into();
        
    disp.init().unwrap();
    
    // STM32L031 does not have a hardware RNG, need to use software
   

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

