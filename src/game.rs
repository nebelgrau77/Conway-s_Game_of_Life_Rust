use core::fmt;
use arrayvec::ArrayString;

pub struct Pixel {
    byteidx: u16,
    bitidx: i16,
    pub value: u8,
    }

static WX: i16 = 32; // grid width
static HY: i16 = 32; // grid height

// helper function: handles the cell evolution based on the cell's initial state and the number of neighbors
// https://rosettacode.org/wiki/Conway's_Game_of_Life

pub fn evo(cell: u8, neighbors: u8) -> u8 {
    
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

pub fn matrix_evo(buffer: [u8; 512]) -> [u8; 512] {
    
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
                    if x + n < 0 || y + m < 0 || (x + n) > (WX - 1) || (y + m) > (HY - 1) || ((m == 0) && (n == 0)) {
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

pub fn pixelgetter(x: i16, y: i16, buffer: [u8; 512]) -> Pixel {
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

pub fn counter(buf: &mut ArrayString<[u8; 8]>, gen: u16) {   
    
    let singles = gen%10;
    let tens = (gen/10)%10;
    let hundreds = (gen/100)%10;
    let thousands = (gen/1000)%10;
            
    fmt::write(buf, format_args!("Gen:{}{}{}{}", thousands as u8, hundreds as u8, tens as u8, singles as u8)).unwrap();

}