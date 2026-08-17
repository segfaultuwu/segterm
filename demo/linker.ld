OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)

PHDRS
{
    text    PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* R+X */
    rodata  PT_LOAD    FLAGS((1 << 2)) ;             /* R   */
    data    PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ;  /* R+W */
}

SECTIONS
{
    . = 0xffffffff80000000;

    .text : {
        *(.text .text.*)
    } :text

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .rodata : {
        *(.rodata .rodata.*)
    } :rodata

    . = ALIGN(CONSTANT(MAXPAGESIZE));

    .data : {
        *(.data .data.*)
    } :data

    .requests_start_marker : {
        KEEP(*(.requests_start_marker))
    } :data

    .requests : {
        KEEP(*(.requests))
    } :data

    .requests_end_marker : {
        KEEP(*(.requests_end_marker))
    } :data

    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    } :data

    /DISCARD/ : {
        *(.eh_frame)
        *(.note .note.*)
    }
}
