/* Linker script for the STMF030F4P6 */
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x08000000, LENGTH = 16K
  RAM : ORIGIN = 0x20000000, LENGTH = 4K
}

/* NOTE: Do *NOT* modify `_stack_start` unless you know what you are doing. */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
