//! Conway's Game of Life
//! 
//! original MicroPython code ported to STM32F1xx board (this code is for STM32F103C8T6)
//! 
//! improved to store pixels as single bits, i.e. one byte is 8 pixels
//! 
//! the code currently resets after a 1000 generations
//! 
//! the grid "dies" after less than 600 generations
//! leaving only static or oscillating debris
//! 
//! next steps: use ADC to get a different RNG seed each time
//! 

#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f1xx_hal as hal;
extern crate cortex_m;

use cortex_m_rt::entry;

use rand::prelude::*;

use hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
    delay::Delay,
    
};

use ssd1306::{prelude::*, Builder as SSD1306Builder};

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
    };

use core::fmt;
use core::fmt::Write;
use arrayvec::ArrayString;

struct Pixel {
    
    byteidx: u16,
    bitidx: i8,
    value: u8,
}

static WX: i8 = 64; // grid width
static HY: i8 = 64; // grid height


#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(24.mhz()).freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );
    
    // display initiated in GraphicsMode
    
    let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x64).connect_i2c(i2c).into();
        
    disp.init().unwrap();
  
    loop {

        let mut rng = SmallRng::seed_from_u64(0x0101_0808_0303_0909);

        let mut buffer = [0u8; 512];
        
        let text_style = TextStyleBuilder::new(Font6x8).text_color(BinaryColor::On).build();

        // generate first random grid

        rng.fill_bytes(&mut buffer);
        
        // display it
        
        for x in 0..WX {
            for y in 0..HY {
                let pixel = pixelgetter(x,y,buffer);
                disp.set_pixel(x as u32,y as u32,pixel.value);
            }
        }

        let mut text_buf = ArrayString::<[u8; 8]>::new(); // create a buffer for the counter text

        let mut gen: u16 = 0; // generation counter
        
        counter(&mut text_buf, gen);

        Text::new(text_buf.as_str(), Point::new(80, 0)).into_styled(text_style).draw(&mut disp);

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

        Text::new(text_buf.as_str(), Point::new(80, 0)).into_styled(text_style).draw(&mut disp);

        
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


// helper function: handles the cell evolution based on the cell's initial state and the number of neighbors
// https://rosettacode.org/wiki/Conway's_Game_of_Life

fn evo(cell: u8, neighbors: u8) -> u8 {
    
    let mut new_cell = 0;
        
    if cell == 1 {
        if neighbors == 2 || neighbors == 3
            {
            new_cell = 1 
            }
        } else {
            if neighbors == 3 {
            new_cell = 1 
            }
        }
    
        return new_cell;
        
        }
    

// generate a new matrix made of bytes, based on the cells evolution

fn matrix_evo(buffer: [u8; 512]) -> [u8; 512] {
    
    //must use signed integers otherwise can't do the -1/+1 operation 
    
    let mut new_buffer = [0u8;512];

    let mut new_cell: u8 = 0;
    let mut cell: u8 = 0;

    for x in 0..WX {
        for y in 0..HY {
            
            let pixel = pixelgetter(x,y, buffer);

            let mut neighbors: u8 = 0;
            
            for n in -1..2 {
                for m in -1..2 {
                    if x + n < 0 || y + m < 0 || (x + n) > (WX - 1) || (y + m) > (WX - 1) || ((m == 0) && (n == 0)) {
                        continue // do nothing, that is neighbors stays as it is
                    }
                    else {
                        let new_neighbor = pixelgetter(x+n, y+m, buffer);
                        neighbors += new_neighbor.value;
                    }
                    
                    cell = pixel.value;
                    new_cell = evo(cell, neighbors);
                }
            }

            if new_cell == 0 {
                new_buffer[pixel.byteidx as usize] &= !(1 << (7-pixel.bitidx)); // clear bit to 0
            } 
            else {
                new_buffer[pixel.byteidx as usize] |= 1 << (7-pixel.bitidx); // set bit to 1
            }

        }
    }
    return new_buffer;

}


// helper function to get the binary value of (x,y) pixel in a 1-D array of bytes
// as well as the index of the byte and the bit corresponding to that pixel
// note: in each byte the pixel with the lowest x coordinate will correspond to the most significant bit

fn pixelgetter(x: i8, y: i8, buffer: [u8; 512]) -> Pixel {
    let byteidx: u16 = y as u16 *(WX as u16 /8) + x as u16/8;
    let bitidx: i8 = x%8;
    let byte: u8 = buffer[byteidx as usize];
    let pixelval: u8 = (byte/(2_u8.pow((7-bitidx) as u32)))%2;
    return Pixel {
        byteidx: byteidx,
        bitidx: bitidx,
        value: pixelval,
    };
}


// helper function for the generation counter

fn counter(buf: &mut ArrayString<[u8; 8]>, gen: u16) {   
    
    let singles = gen%10;
    let tens = (gen/10)%10;
    let hundreds = (gen/100)%10;
    let thousands = (gen/1000)%10;
            
    fmt::write(buf, format_args!("Gen:{}{}{}{}", thousands as u8, hundreds as u8, tens as u8, singles as u8)).unwrap();

}