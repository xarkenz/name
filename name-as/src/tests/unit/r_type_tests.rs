use crate::assembler::assembly_utils::assemble_r_type;

#[test]
fn assemble_r_type_test() {
    let rd = Some("$t0".to_string());
    let rs = Some("$t1".to_string());
    let rt = Some("$t2".to_string());
    let shamt = Some(0);
    let assembled_output = assemble_r_type(rd, rs, rt, shamt, 32);
    assert_eq!(assembled_output, Ok(0x012A4020));

    let assembled_err = assemble_r_type(Some("bad register".to_string()), None, None, None, 32);
    assert!(assembled_err.is_err());

    let rd = Some("$t0".to_string());
    let rs = Some("$t1".to_string());
    let shamt = Some (32);
    let assembled_shamt_err = assemble_r_type(rd, rs, None, shamt, 32);
    assert!(assembled_shamt_err.is_err());
}