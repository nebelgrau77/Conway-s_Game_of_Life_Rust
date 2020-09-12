# Conway-s_Game_Of_Life

My original MicroPython code ported to Rust on Arduino Nano 33 IoT (Microchip/Atmel SAMD21).

It follows the rules of the game as described here: https://en.wikipedia.org/wiki/Conway's_Game_of_Life
and here: http://rosettacode.org/wiki/Conway's_Game_of_Life

It starts with a randomly generated white noise pattern, which slowly dissolves.

Will restart after a 1000 generations.

![Game of Life](conway_L0.gif)

TO DO: 

* uses a custom fork of the arduino_nano33iot crate with the i2c_master function added: contribute to the main repo
* add some generation of the initial seed (e.g. from ADC)


