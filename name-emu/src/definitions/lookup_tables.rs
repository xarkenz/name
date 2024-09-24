use crate::decode::InstructionFn;
use crate::definitions::instruction_implementations::*;
use crate::definitions::syscalls::*;

/*

   ____  _____   _____ ____  _____  ______ 
  / __ \|  __ \ / ____/ __ \|  __ \|  ____|
 | |  | | |__) | |   | |  | | |  | | |__   
 | |  | |  ___/| |   | |  | | |  | |  __|  
 | |__| | |    | |___| |__| | |__| | |____ 
  \____/|_|     \_____\____/|_____/|______|
                                           
                                           

*/

pub const OPCODE_TABLE: [Option<InstructionFn>; 64] = [
    None,  // 0x00
    None,  // 0x01
    Some(j),  // 0x02
    Some(jal),  // 0x03
    Some(beq),  // 0x04
    Some(bne),  // 0x05
    Some(blez),  // 0x06
    Some(bgtz),  // 0x07
    Some(addi),  // 0x08
    Some(addiu),  // 0x09
    None /*Some(slti)*/,  // 0x0A
    None /*Some(sltiu)*/,  // 0x0B
    Some(andi),  // 0x0C
    Some(ori),  // 0x0D
    None,  // 0x0E
    Some(lui),  // 0x0F
    None,  // 0x10
    None,  // 0x11
    None,  // 0x12
    None,  // 0x13
    None,  // 0x14
    None,  // 0x15
    None,  // 0x16
    None,  // 0x17
    None,  // 0x18
    None,  // 0x19
    None,  // 0x1A
    None,  // 0x1B
    None,  // 0x1C
    None,  // 0x1D
    None,  // 0x1E
    None,  // 0x1F
    Some(lb),  // 0x20
    None,  // 0x21
    None,  // 0x22
    Some(lw),  // 0x23
    None,  // 0x24
    None,  // 0x25
    None,  // 0x26
    None,  // 0x27
    None /*Some(sb)*/,  // 0x28
    None,  // 0x29
    None,  // 0x2A
    Some(sw),  // 0x2B
    None,  // 0x2C
    None,  // 0x2D
    None,  // 0x2E
    None,  // 0x2F
    None,  // 0x30
    None,  // 0x31
    None,  // 0x32
    None,  // 0x33
    None,  // 0x34
    None,  // 0x35
    None,  // 0x36
    None,  // 0x37
    None,  // 0x38
    None,  // 0x39
    None,  // 0x3A
    None,  // 0x3B
    None,  // 0x3C
    None,  // 0x3D
    None,  // 0x3E
    None,  // 0x3F
];

/*

  ______ _    _ _   _  _____ _______ 
 |  ____| |  | | \ | |/ ____|__   __|
 | |__  | |  | |  \| | |       | |   
 |  __| | |  | | . ` | |       | |   
 | |    | |__| | |\  | |____   | |   
 |_|     \____/|_| \_|\_____|  |_|   
                                     
                                     

*/

pub const FUNCT_TABLE: [Option<InstructionFn>; 64] = [
    Some(sll), // 0x00
    None, // 0x01
    Some(srl), // 0x02
    None, // 0x03
    None, // 0x04
    None, // 0x05
    None, // 0x06 ok buddy -> oiuougou :3
    None, // 0x07
    Some(jr), // 0x08
    Some(jalr), // 0x09
    None, // 0x0A
    None, // 0x0B
    Some(syscall), // 0x0C
    None, // 0x0D
    None, // 0x0E
    None, // 0x0F
    None, // 0x10
    None, // 0x11
    None, // 0x12
    None, // 0x13
    None, // 0x14
    None, // 0x15
    None, // 0x16
    None, // 0x17
    None, // 0x18
    None, // 0x19
    None, // 0x1A
    None, // 0x1B
    None, // 0x1C
    None, // 0x1D
    None, // 0x1E
    None, // 0x1F
    Some(add), // 0x20
    Some(addu), // 0x21
    Some(sub), // 0x22
    None /*Some(subu)*/, // 0x23
    Some(and), // 0x24
    Some(or), // 0x25
    Some(xor), // 0x26
    Some(nor), // 0x27
    None, // 0x28
    None, // 0x29
    Some(slt), // 0x2A
    None, // 0x2B
    None, // 0x2C
    None, // 0x2D
    None, // 0x2E
    None, // 0x2F
    None, // 0x30
    None, // 0x31
    None, // 0x32
    None, // 0x33
    None, // 0x34
    None, // 0x35
    None, // 0x36
    None, // 0x37
    None, // 0x38
    None, // 0x39
    None, // 0x3A
    None, // 0x3B
    None, // 0x3C
    None, // 0x3D
    None, // 0x3E
    None, // 0x3F
];

/*

   _______     _______  _____          _      _      
  / ____\ \   / / ____|/ ____|   /\   | |    | |     
 | (___  \ \_/ / (___ | |       /  \  | |    | |     
  \___ \  \   / \___ \| |      / /\ \ | |    | |     
  ____) |  | |  ____) | |____ / ____ \| |____| |____ 
 |_____/   |_| |_____/ \_____/_/    \_\______|______|
                                                     
                                                     

*/

pub const SYSCALL_TABLE: [Option<SyscallFn>; 64] = [
    None, // 0x00
    Some(sys_print_int), // 0x01
    None, // 0x02
    None, // 0x03
    Some(sys_print_string), // 0x04
    None, // 0x05
    None, // 0x06
    None, // 0x07
    None, // 0x08
    None, // 0x09
    Some(sys_exit), // 0x0A
    Some(sys_print_char), // 0x0B
    None, // 0x0C
    None, // 0x0D
    None, // 0x0E
    None, // 0x0F
    None, // 0x10
    None, // 0x11
    None, // 0x12
    None, // 0x13
    None, // 0x14
    None, // 0x15
    None, // 0x16
    None, // 0x17
    None, // 0x18
    None, // 0x19
    None, // 0x1A
    None, // 0x1B
    None, // 0x1C
    None, // 0x1D
    None, // 0x1E
    None, // 0x1F
    None, // 0x20
    None, // 0x21
    None, // 0x22
    None, // 0x23
    None, // 0x24
    None, // 0x25
    None, // 0x26
    None, // 0x27
    None, // 0x28
    None, // 0x29
    None, // 0x2A
    None, // 0x2B
    None, // 0x2C
    None, // 0x2D
    None, // 0x2E
    None, // 0x2F
    None, // 0x30
    None, // 0x31
    None, // 0x32
    None, // 0x33
    None, // 0x34
    None, // 0x35
    None, // 0x36
    None, // 0x37
    None, // 0x38
    None, // 0x39
    None, // 0x3A
    None, // 0x3B
    None, // 0x3C
    None, // 0x3D
    None, // 0x3E
    None, // 0x3F
];