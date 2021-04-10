MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
