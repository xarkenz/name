use crate::structs::Coprocessor0;
// TODO: Possibly a very good place for a macro. Lots of tiny fields to account for...
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