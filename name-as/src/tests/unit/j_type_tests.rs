use crate::assembly_utils::assemble_j_type;

#[test]
fn assemble_j_type_test() {
    let opcode: u32 = 3;
    let target: u32 = 0x40BEE0;

    let assembled_output = assemble_j_type(opcode, Some(target));
    assert_eq!(assembled_output, Ok(Some(0x0c40BEE0)));
}