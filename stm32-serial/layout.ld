MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 64K
  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 20K
}

SECTIONS
{
  /* Set stack top to end of RAM */
  __StackTop = ORIGIN(RAM) + LENGTH(RAM);

  .text :
  {
    LONG(__StackTop);
    KEEP(*(.ivtable));

	. = 0x144;

    *(.text*)
  } > FLASH
}
