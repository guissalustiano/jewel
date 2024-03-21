MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* These values correspond to the NRF5340-app */
  /*
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
  */

  /* These values correspond to the NRF5340-net */
  FLASH : ORIGIN = 0x01000000, LENGTH = 256K
  RAM : ORIGIN =   0x21000000, LENGTH = 64K
}
