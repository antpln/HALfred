MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

SECTIONS
{
  .vector_table : {
    KEEP(*(.vector_table))
  } > FLASH

  .text : {
    *(.text*)
    *(.rodata*)
    KEEP(*(.init))
    KEEP(*(.fini))
  } > FLASH

  .data : {
    *(.data*)
  } > RAM AT > FLASH

  .bss : {
    *(.bss*)
    *(COMMON)
  } > RAM

  _stack_start = ORIGIN(RAM) + LENGTH(RAM); /* Top of the stack */
}
