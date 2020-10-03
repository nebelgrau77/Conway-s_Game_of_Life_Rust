# Conway-s_Game_Of_Life

My original MicroPython code ported to Rust on Adafruit PyBadge (Microchip/Atmel SAMD51).

It follows the rules of the game as described here: https://en.wikipedia.org/wiki/Conway's_Game_of_Life
and here: http://rosettacode.org/wiki/Conway's_Game_of_Life

![game of life](conway_pybadge.gif)

It starts with a randomly generated white noise pattern, which slowly dissolves.

Will restart after a 1000 generations.

_NOTES:_ 

_The speed of the MCU allows relatively fast calculation of a bigger matrix, even 128x128. The resulting effect is not pretty, though:_
_the ST7735 refreshes in a "scanning" mode from left to right, it does not look as good as on the OLED displays._



