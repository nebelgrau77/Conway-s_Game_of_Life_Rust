//! conway's game of life DRAFT
//! 
//! very slow
//! 
//! can fit 64x64 grid using single bits to represent pixels
//! 
//! next steps: use ADC to get a different RNG seed each time
//! 

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f0xx_hal as hal;

use cortex_m_rt::entry;
use ssd1306::{prelude::*, Builder as SSD1306Builder};
use rand::prelude::*;

use crate::hal::{
    prelude::*,
    stm32,
    i2c::I2c,
};

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
    };

use core::fmt;
//use core::fmt::Write;
use arrayvec::ArrayString;

struct Pixel {
    
    byteidx: u16,
    bitidx: u8,
    value: u8,
}


#[entry]
fn main() -> ! {

    if let Some(mut p) = stm32::Peripherals::take() {
        
        cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);
        
        let gpioa = p.GPIOA.split(&mut rcc);
        let scl = gpioa.pa9.into_alternate_af4(cs);
        let sda = gpioa.pa10.into_alternate_af4(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), &mut rcc);

        let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x64).connect_i2c(i2c).into();
        
        disp.init().unwrap();
        
        let w: u8 = 64;
        let h: u8 = 64;   

        

        loop {

            let mut rng = SmallRng::seed_from_u64(0x0101_0808_0303_0909);

            let mut matrix = [0u8; 512]; 

            let text_style = TextStyleBuilder::new(Font6x8).text_color(BinaryColor::On).build();

            // generate first random matrix

            rng.fill_bytes(&mut matrix);

            // display it
        
            for x in 0..w {
                for y in 0..h {
                    let pixel = pixelgetter(x,y,w,matrix);
                    disp.set_pixel(x as u32,y as u32,pixel.value);
                }
            }

            let mut format_buf = ArrayString::<[u8; 8]>::new(); // create a buffer for the counter text

            let mut gen: u16 = 0; // generation counter
            
            format_gen(&mut format_buf, gen);

            Text::new(format_buf.as_str(), Point::new(80, 0)).into_styled(text_style).draw(&mut disp);

            disp.flush().unwrap();

            while gen < 1000 {

                let w: u8 = 64;
                let h: u8 = 64;
        
                // clean up the number area of the counter
        
                for m in 110..128 {
                    for n in 0..8 {
                        disp.set_pixel(m, n, 0);
                    }
                }
        
        
                let mut format_buf = ArrayString::<[u8; 8]>::new();
        
                gen += 1;
        
                format_gen(&mut format_buf, gen);
        
                Text::new(format_buf.as_str(), Point::new(80, 0)).into_styled(text_style).draw(&mut disp);
        
                
                matrix = matrix_evo(matrix, 64, w);
        
                for x in 0..w {
                    for y in 0..h {
                        let pixel = pixelgetter(x,y,w,matrix);
                        disp.set_pixel(x as u32,y as u32,pixel.value);
                    }
                }
                
        
        
                disp.flush().unwrap();
        
        
        
        
            }



        }

       
    });
    
}

    loop {continue;}
    
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
        } 
        else 
        { 
            if neighbors == 3 
            {
            new_cell = 1 
            }
        }
    
        return new_cell;
        
    }
 
    
// generate a new matrix made of bytes, based on the cells evolution

fn matrix_evo(matrix: [u8; 512], size: i8, w: u8) -> [u8; 512] {
    
    //must use signed integers otherwise can't do the -1/+1 operation 
    
    let mut new_matrix = [0u8;512];

    let mut new_cell: u8 = 0;
    let mut cell: u8 = 0;

    for x in 0..size {
        for y in 0..size {
            
            let pixel = pixelgetter(x as u8,y as u8,w,matrix);

            let mut neighbors: u8 = 0;
            
            for n in -1..2 {
                for m in -1..2 {
                    if x + n < 0 || y + m < 0 || (x + n) > (size - 1) || (y + m) > (size - 1) || ((m == 0) && (n == 0)) {
                        continue // do nothing, that is neighbors stays as it is
                    }
                    else {
                        let new_neighbor = pixelgetter((x+n) as u8, (y+m) as u8, w, matrix);
                        neighbors += new_neighbor.value;
                    }
                    
                    cell = pixel.value;
                    new_cell = evo(cell, neighbors);
                }
            }
        

        if new_cell == 0 {
            new_matrix[pixel.byteidx as usize] &= !(1 << (7-pixel.bitidx));
        } 
        else {
            new_matrix[pixel.byteidx as usize] |= 1 << (7-pixel.bitidx);
        }


        }
    }
    return new_matrix;

}

fn format_gen(buf: &mut ArrayString<[u8; 8]>, gen: u16) {   
    
    let singles = gen%10;
    let tens = (gen/10)%10;
    let hundreds = (gen/100)%10;
            
    fmt::write(buf, format_args!("Gen: {}{}{}", hundreds as u8, tens as u8, singles as u8)).unwrap();

}


// helper function to get the binary value of (x,y) pixel in a 1-D array of bytes
// needs the screen width as a parameter to correctly identify the bytes of each line

fn pixelgetter(x: u8, y: u8, w: u8, array: [u8; 512]) -> Pixel {
    let byteidx: u16 = y as u16 *(w as u16 /8) + x as u16/8;
    let bitidx: u8 = x%8;
    let byte: u8 = array[byteidx as usize];
    let pixelval: u8 = (byte/(2_u8.pow((7-bitidx) as u32)))%2;
    return Pixel {
        byteidx: byteidx,
        bitidx: bitidx,
        value: pixelval,
    };
}

