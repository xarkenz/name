use super::registers::Register;
/// The table contained in this file defines the details for every register NAME needs to use in Coprocessor 0.
/// This simplifies the Register and Select field representation, 
/// as the MIPS standard has overloaded meanings for certain registers in certain contexts.
/// It is entirely based on information from this document: https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MD00090-2B-MIPS32PRA-AFP-06.02.pdf
pub struct Cp0RegisterInformation {
    name: Register,
    register: usize,
    select: usize,
}

pub const CP0_REGISTER_INFO: &[Cp0RegisterInformation] = &[
    Cp0RegisterInformation {
        name: Register::Status,
        register: 12,
        select: 0,
    },
    Cp0RegisterInformation {
        name: Register::Cause,
        register: 13,
        select: 0,
    }
];