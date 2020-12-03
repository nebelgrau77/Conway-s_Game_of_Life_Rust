# Conway-s_Game_Of_Life

My original MicroPython code ported to Rust on STM32F1 "blue pill", and then to other Cortex-M boards.
It's visibly faster on 72MHz "blue pill" in Rust than on 240MHz ESP32 in MicroPython :)

It follows the rules of the game as described here: https://en.wikipedia.org/wiki/Conway's_Game_of_Life
and here: http://rosettacode.org/wiki/Conway's_Game_of_Life

It starts with a randomly generated white noise pattern, which slowly dissolves. 
The initial seed for the RNG is taken from the internal temperature sensor

Will restart after a 1000 generations.

_Check branches for ports to other devboards:_
* _STM32F030_
* _STM32F051_
* _STM32F411_
* _STM32F407 (with hardware Random Number Generator)_
* _STM32F407 using a dual OLED setup_
* _STM32L0_
* _Microchip SAMD21 on Arduino Nano 33 IoT_
* _Microchip SAMD51 on Adafruit PyBadge_
* _Nordic nRF52840 on Adafruit ItsyBitsy nRF52840 Express_* 



