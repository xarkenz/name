use crate::structs::Coprocessor0;
use crate::exception::register_set::to_register;
use crate::exception::registers::Register;

use paste::paste;

#[macro_export]
// The following macro allows us to enter just a couple values to generate an appropriately-named getter and setter for 
// each bit field.
macro_rules! getset {
    // This variant is for when the register is supplied.
    ($name:ident, $reg:expr, $start:expr, $end:expr) => {
        paste! {
            impl Coprocessor0 {
                pub fn [<get_ $name>](&self) -> u32 {
                    self.get_bit_field(to_register($reg), $start, $end)
                }

                pub fn [<set_ $name>](&mut self, value: u32) {
                    self.set_bit_field(to_register($reg), $start, $end, value)
                }
            }
        }
    };
    // And this variant is when the register shouldn't necessarily be supplied;
    ($name:ident, $start:expr, $end:expr) => {
        paste! {
            impl Coprocessor0 {
                pub fn [<get_ $name>](&self, register: usize) -> u32 {
                    self.get_bit_field(register, $start, $end)
                }

                pub fn [<set_ $name>](&mut self, register: usize, value: u32) {
                    self.set_bit_field(register, $start, $end, value)
                }
            }
        }
    };
}

/// All the implementations below are to perform bit-level accesses.
/// Coprocessor 0 is organized with multiple fields inside any one 4-byte word. It would get old to remember where they all are.
/// For instance, bit 1 of register 12, select 0 (Status) represents the Exception Level.
impl Coprocessor0 {
    fn get_bit_field(&self, register: usize, start: usize, end: usize) -> u32 {
        if start > end || end > 31 {
            panic!("Improper values provided to get_bit_field (Start: {}, End: {})", start, end);
        }

        let num_bits = end - start + 1;
        let mask = ((1 as u32) << num_bits) - 1;    // Neat hack I learned a while ago. Give it some thought!

        // And retrieve the field. Easy as that!
        return (self.registers[register] >> start) & mask;
    }

    fn set_bit_field(&mut self, register: usize, start: usize, end: usize, value: u32) -> () {
        if start > end || end > 31 {
            panic!("Improper values provided to set_bit_field (Start: {}, End: {})", start, end);
        }

        let num_bits = end - start + 1;
        let mask = ((1 as u32) << num_bits) - 1;

        // Clear the specified field
        self.registers[register] &= !(mask << start);
        // Set the specified field
        self.registers[register] |= (value & mask) << start;
    }
}

// Below are all the macro-defined accessors for the small bit fields.
getset!(exception_level, Register::Status, 1, 1);