# Hello, world!
    .eqv    SysPrintString, 4
    .eqv    SysExit, 10
    
    .data

OurBelovedString:
    .asciiz     "Hello, World!"

    .text
    la  $a0, OurBelovedString
    li  $v0, SysPrintString
    syscall
    li  $v0, SysExit
    syscall