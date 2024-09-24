.include    "SysCalls.asm"
    
.data

OurBelovedString: .asciiz     "hello\nworlde\ti am swagalicious\\\'\""

.text
    li  $a0, '\n'
    li  $v0, SysPrintChar
    syscall
    li  $a0, '\t'
    li  $v0, SysPrintChar
    syscall
    li  $a0, '\\'
    li  $v0, SysPrintChar
    syscall

    la  $a0, OurBelovedString
    li  $v0, SysPrintString
    syscall
    li  $v0, SysExit
    syscall