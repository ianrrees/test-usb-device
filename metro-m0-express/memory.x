MEMORY
{
  /* No bootloader */
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 256K

  /* Leave 8k for the default bootloader on the Metro M0 */
  /* FLASH (rx) : ORIGIN = 0x00000000 + 8K, LENGTH = 256K - 8K */

  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 32K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

