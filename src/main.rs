//! Conway's Game of Life
//! 
//! //! original MicroPython code ported to STM32F4xx board (this code is for STM32F407VET6)
//! 
//! improved to store pixels as single bits, i.e. one byte is 8 pixels
//! 
//! first grid is generated using built-in hardware random number generator
//! 
//! the code currently resets after a 1000 generations
//! 
//! "seamless" version (toroidal array): 
//! left and right edges of the field are considered to be stitched together, 
//! as well as the top and bottom edges
//! 
//! the grid usually "dies" after a few hundred generations
//! leaving only static or oscillating debris
//!  

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f4xx_hal as hal;

use cortex_m_rt::entry;

use rand_core::RngCore;

use crate::hal::{
    i2c::I2c, 
    prelude::*, 
    stm32,
    delay::Delay,
    };

use ssd1306::{
    prelude::*, 
    Builder as SSD1306Builder
    };

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
    };

use core::fmt;
use arrayvec::ArrayString;

struct Pixel {
    byteidx: u16,
    bitidx: i16,
    value: u8,
}

const BOOT_DELAY_MS: u16 = 100; 

static WX: i16 = 64; // grid width
static HY: i16 = 64; // grid height


#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
) {
        // Set up the system clock. We want to run at max speed. 
        // High speed external clock from the external 8 MHz crystal
        // PCLK1 (internal APB1 clock frequency) set to the maximum
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(168.mhz()).pclk1(42.mhz()).freeze();
        
        let mut delay = Delay::new(cp.SYST, clocks);
        
        //delay necessary for the I2C to initiate correctly and start on boot without having to reset the board

        delay.delay_ms(BOOT_DELAY_MS);

        // Set up I2C - SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb9.into_alternate_af4().set_open_drain();
        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

        
        // Set up the display
        
        let mut disp: GraphicsMode<_> = SSD1306Builder::new().connect_i2c(i2c).into();
        
        disp.init().unwrap();

        // use hardware random numbers generator

        let mut rng = dp.RNG.constrain(clocks);

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

            Text::new(text_buf.as_str(), Point::new(80, 0)).into_styled(text_style).draw(&mut disp);

            disp.flush().unwrap();

            
            while gen < 1001 {
        
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

    loop {}
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
                    if (m == 0) && (n == 0) // the cell itself is not added to the neighbors
                    {
                        continue // do nothing, that is neighbors stays as it is
                    }
                    
                    else if x + n < 0 // cells on the left edge of the grid
                    { 
                        if y + m < 0 // cells on the upper edge of the grid
                        {
                            let new_neighbor = pixelgetter(WX-1, HY-1, buffer);
                            neighbors += new_neighbor.value;
                        }
                        else if (y + m) > (HY-1) // cells on the lower edge of the grid
                        {
                            let new_neighbor = pixelgetter(WX-1, 0, buffer);
                            neighbors += new_neighbor.value;
                        }
                        else 
                        {
                            let new_neighbor = pixelgetter(WX-1, y+m, buffer);
                            neighbors += new_neighbor.value;
                        }
                    }


                    else if (x + n) > (WX - 1) { // cells on the right edge of the grid
                        
                        if y + m < 0 { // cells on the upper edge of the grid
                            let new_neighbor = pixelgetter(0, HY-1, buffer); 
                            neighbors += new_neighbor.value;
                        }
                        else if (y + m) > (HY-1) { // cells on the lower edge of the grid
                            let new_neighbor = pixelgetter(0, 0, buffer);
                            neighbors += new_neighbor.value;                        
                        }
                        else {
                            let new_neighbor = pixelgetter(0, y+m, buffer);
                            neighbors += new_neighbor.value;   
                        }

                    }



                    else if y + m < 0 // cells on the upper edge of the grid
                    { 
                        if x + n < 0 // cells on the left edge of the grid
                        {
                            let new_neighbor = pixelgetter(WX-1, HY-1, buffer);
                            neighbors += new_neighbor.value;
                        }
                        else if (x + n) > (WX-1) // cells on the right edge of the grid
                        {
                            let new_neighbor = pixelgetter(0, HY-1, buffer);
                            neighbors += new_neighbor.value;
                       }
                        else 
                        {
                            let new_neighbor = pixelgetter(x+n, HY-1, buffer);
                            neighbors += new_neighbor.value;
                        }
                    }

                    

                    else if (y + m) > (HY - 1) { // cells on the lower edge of the grid

                        if x + n < 0 { // cells on the left edge of the grid
                            let new_neighbor = pixelgetter(WX-1, 0, buffer);
                            neighbors += new_neighbor.value;                        
                        }
                        else if (x + n) > (WX - 1) { // cells on the right edge of the grid
                            let new_neighbor = pixelgetter(0, 0, buffer);
                            neighbors += new_neighbor.value;    
                        }

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

fn pixelgetter(x: i16, y: i16, buffer: [u8; 512]) -> Pixel {
    let byteidx: u16 = y as u16 *(WX as u16 /8) + x as u16/8;
    let bitidx: i16 = x%8;
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