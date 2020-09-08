# Conway-s_Game_Of_Life

My original MicroPython code ported to Rust on STM32L031.

It follows the rules of the game as described here: https://en.wikipedia.org/wiki/Conway's_Game_of_Life
and here: http://rosettacode.org/wiki/Conway's_Game_of_Life

It starts with a randomly generated white noise pattern, which slowly dissolves.

Will restart after a 1000 generations.

![Game of Life](conway_L0.gif)

TO DO: 

* move functions to a separate .rs file
* get the initial seed for random number generation from ADC (temperature, voltage)

