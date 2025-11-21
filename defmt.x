SECTIONS
{
  /* defmt v1/v0.3の小片(.defmt.*)をまとめて1つの .defmt にする */
  .defmt : ALIGN(4)
  {
    KEEP(*(.defmt.*));
  } > FLASH
}