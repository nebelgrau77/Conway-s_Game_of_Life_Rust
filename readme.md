# Conway-s_Game_Of_Life

My original MicroPython code ported to Rust on STM32F030.
The speed seems to be similar between the STM32 @ 48 MHz in Rust and the ESP32 @ 240MHz in MicroPython :)

It follows the rules of the game as described here: https://en.wikipedia.org/wiki/Conway's_Game_of_Life
and here: http://rosettacode.org/wiki/Conway's_Game_of_Life

It starts with a randomly generated white noise pattern, which slowly dissolves.
Need to use some random value, e.g. read with ADC, as a seed, otherwise it's the same sequence every time.

Will restart after a 1000 generations.

The code still needs some refactoring. It's not as modular as the MicroPython code at the moment.