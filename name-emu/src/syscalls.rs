use name_const::structs::{Processor, Memory};

pub type SyscallFn = fn(&mut Processor, &mut Memory) -> Result<bool, String>;

// Syscall 10 - SysExit
pub fn sys_exit(_cpu: &mut Processor, _memory: &mut Memory) -> Result<bool, String> {
    return Ok(true);
}