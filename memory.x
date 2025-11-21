/* RA6M5
 * Code Flash : 0x0000_0000 - 0x001F_FFFF (2 MiB)
 * SRAM0      : 0x2000_0000 - 0x2007_FFFF (512 KiB)
 * Data Flash : 0x0800_0000 - 0x0800_1FFF (8 KiB)  // 今回は未使用
 */

MEMORY
{
  FLASH (rx)  : ORIGIN = 0x00000000, LENGTH = 2048K
  RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 512K
  /* DATAFLASH (rx) : ORIGIN = 0x08000000, LENGTH = 8K */ /* 使うなら uncomment */
}

/* 任意：スタック/ヒープサイズ（必要に応じて値調整） */
_stack_size = 32K;
_heap_size  = 0K;

/* 任意：cortex-m-rt が参照できるように stack start を置く */
PROVIDE(_ram_end     = ORIGIN(RAM) + LENGTH(RAM));
PROVIDE(_stack_start = _ram_end);