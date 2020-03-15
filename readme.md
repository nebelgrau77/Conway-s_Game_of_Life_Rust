# Conway-s_Game_Of_Life

My original MicroPython code ported to Rust on STM32F1 "blue pill".
It's visibly faster on 72MHz "blue pill" in Rust than on 240MHz ESP32 in MicroPython :)

It follows the rules of the game as described here: https://en.wikipedia.org/wiki/Conway's_Game_of_Life
and here: http://rosettacode.org/wiki/Conway's_Game_of_Life

It starts with a randomly generated white noise pattern, which slowly dissolves. 
The initial seed for the RNG is taken from the internal temperature sensor

Will restart after a 1000 generations.