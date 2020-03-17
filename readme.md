# Conway-s_Game_Of_Life

![](conway.gif)

My original MicroPython code ported to Rust on STM32F4.

It follows the rules of the game as described here: https://en.wikipedia.org/wiki/Conway's_Game_of_Life
and here: http://rosettacode.org/wiki/Conway's_Game_of_Life

It starts with a randomly generated white noise pattern, which slowly dissolves. 
First pattern is generated using built-in hardware Random Numbers Generator.

Will restart after a 1000 generations.