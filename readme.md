# Conway-s_Game_Of_Life

My original MicroPython code ported to Rust on STM32F1 "blue pill".
It's visibly faster on 72MHz "blue pill" in Rust than on 240MHz ESP32 in MicroPython :)

It follows the rules of the game as described here: https://en.wikipedia.org/wiki/Conway's_Game_of_Life
and here: http://rosettacode.org/wiki/Conway's_Game_of_Life

It starts with a randomly generated white noise pattern, which slowly dissolves.
Problem: currently it's the same every time, as the RNG starts with the exact same seed. 
Need to implement a "seedless" solution, or use some random value, e.g. read with ADC, as a seed.

Will restart after a 1000 generations.

The code needs some refactoring. It's not as modular as the MicroPython code at the moment.