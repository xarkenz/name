use crate::assembler::assembly_utils::assemble_i_type;

#[test]
fn assemble_i_type_test() {
    let opcode: u32 = 13;
    let rt: Option<String> = Some("$t0".to_string());
    let rs: Option<String> = Some("$t2".to_string());
    let immediate: Option<i32> = Some(0xBEEF);

    let assembled_output = assemble_i_type(opcode, rs, rt, immediate);
    assert_eq!(assembled_output, Ok(0x3548BEEF));
}