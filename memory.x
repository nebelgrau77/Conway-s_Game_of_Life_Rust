/*
https://learn.adafruit.com/introducing-the-adafruit-nrf52840-feather/hathach-memory-map
*/

MEMORY
{
  /*
   * S140 SoftDevice takes up space at the bottom of FLASH and RAM, so
   * offset our values from there.
   */
  FLASH (rx): ORIGIN = 0x26000,    LENGTH = 0xED000 - 0x26000
  RAM (rwx):  ORIGIN = 0x20003400, LENGTH = 0x20010000 - 0x20003400
}
