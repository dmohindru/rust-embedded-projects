/* Linker script for nRF52833 (microbit v2) */
/* https://infocenter.nordicsemi.com/pdf/nRF52833_PS_v2.1.pdf */

MEMORY
{
  /* Flash memory: 512 KB total
     Reserve first 4 KB for bootloader if needed */
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  
  /* RAM memory: 128 KB total */
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full size RAM and grows downward. */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* You can use this symbol to customize the location of the .data section */
/* If omitted the .data section will be placed right after the .text section */
/* _sdata = ORIGIN(RAM); */

/* You can use this symbol to customize the location of the .bss section */
/* If omitted the .bss section will be placed right after the .data section */
/* _sbss = ORIGIN(RAM); */
