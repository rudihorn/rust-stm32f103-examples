MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 64K
  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 20K
}

SECTIONS
{
  /* Set stack top to end of RAM */
  __StackTop = ORIGIN(RAM) + LENGTH(RAM);

  /* first entry should be the stack pointer and then the interrupt table */
  .ivtable :
  {
    LONG(__StackTop);
    KEEP(*(.ivtable));
  } > FLASH

  /* after that the program data is copied */
  .text :
  {
    *(.text*)
	*(.rodata*)
  } > FLASH
  
  .etext : 
  {
	_etext = . ;
  } > FLASH

  /* The data has a virtual address in RAM, but is copied just after the program data */
  .data : AT ( ADDR(.etext) ) 
  {
    _data = . ;
    *(.data*)
	_edata = . ;
  } > RAM

  /* The data that receives a virtual address in RAM, but needs to be initialized to zero */
  .bss : { 
    _bstart = . ;
    *(.bss*) *(COMMON) 
	_bend = . ;
  } > RAM
}
