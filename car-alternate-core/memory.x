/*
  Linker script for the STM32F103C8T6

  Taken from the `stm32-rs/stm32f1xx-hal` GitHub repository
  => https://github.com/stm32-rs/stm32f1xx-hal/blob/master/memory.x
*/

MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}