use crate::definitions::syscalls::*;

/*

   _______     _______  _____          _      _
  / ____\ \   / / ____|/ ____|   /\   | |    | |
 | (___  \ \_/ / (___ | |       /  \  | |    | |
  \___ \  \   / \___ \| |      / /\ \ | |    | |
  ____) |  | |  ____) | |____ / ____ \| |____| |____
 |_____/   |_| |_____/ \_____/_/    \_\______|______|



*/

pub const SYSCALL_TABLE: [Option<SyscallFn>; 64] = [
    None,                   // 0x00
    Some(sys_print_int),    // 0x01
    None,                   // 0x02
    None,                   // 0x03
    Some(sys_print_string), // 0x04
    Some(sys_read_int),     // 0x05
    None,                   // 0x06
    None,                   // 0x07
    None,                   // 0x08
    None,                   // 0x09
    Some(sys_exit),         // 0x0A
    Some(sys_print_char),   // 0x0B
    Some(sys_read_char),    // 0x0C
    None,                   // 0x0D
    None,                   // 0x0E
    None,                   // 0x0F
    None,                   // 0x10
    None,                   // 0x11
    None,                   // 0x12
    None,                   // 0x13
    None,                   // 0x14
    None,                   // 0x15
    None,                   // 0x16
    None,                   // 0x17
    None,                   // 0x18
    None,                   // 0x19
    None,                   // 0x1A
    None,                   // 0x1B
    None,                   // 0x1C
    None,                   // 0x1D
    None,                   // 0x1E
    None,                   // 0x1F
    None,                   // 0x20
    None,                   // 0x21
    None,                   // 0x22
    None,                   // 0x23
    None,                   // 0x24
    None,                   // 0x25
    None,                   // 0x26
    None,                   // 0x27
    None,                   // 0x28
    None,                   // 0x29
    None,                   // 0x2A
    None,                   // 0x2B
    None,                   // 0x2C
    None,                   // 0x2D
    None,                   // 0x2E
    None,                   // 0x2F
    None,                   // 0x30
    None,                   // 0x31
    None,                   // 0x32
    None,                   // 0x33
    None,                   // 0x34
    None,                   // 0x35
    None,                   // 0x36
    None,                   // 0x37
    None,                   // 0x38
    None,                   // 0x39
    None,                   // 0x3A
    None,                   // 0x3B
    None,                   // 0x3C
    None,                   // 0x3D
    None,                   // 0x3E
    None,                   // 0x3F
];
