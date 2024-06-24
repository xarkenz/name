// Utilities to assemble to ELF.
// This file contains relevant structs and methods to produce valid ET_REL and ET_EXEC files.

// Imports
use std::{fs, io::Write, vec::Vec};    // Used for ELF sections

// The following data definitions are required to construct ELF files.
// The meaning of each field is detailed in the appropriate struct.

// Consts

// Section setup for ET_REL files
// These are the sections which should be present in each ET_REL constructed by the functions in this file.
const NUM_OF_SECTIONS: usize = 6;       // This is e_shnum.
const SECTIONS: [&'static str; NUM_OF_SECTIONS] = [
    "",
    ".text",
    ".data",
    /*
    ".symtab",
    ".strtab",
    */
    ".debug",
    ".line",
    ".shstrtab",
];

// Constants pertaining to MIPS conventions
pub const MIPS_TEXT_START_ADDR: u32 = 0x00400000; // The address at which, by convention, MIPS begins the .text section
pub const MIPS_DATA_START_ADDR: u32 = 0x10000000; // The address at which, by convention, MIPS begins the .data section (I really typed this out again!)
const MIPS_ALIGNMENT: u32 = 0x1000;           // The appropriate alignment for MIPS executables (from all my research)
pub const MIPS_ADDRESS_ALIGNMENT: u32 = 4;        // MIPS is aligned by 4-byte word

// ELF File Header fields

// All of the following pertain to the e_ident field, and can be edited granularly below.
const EI_MAG: [u8; 4] = [0x7F, b'E', b'L', b'F'];  // ELF magic bytes to signal that an ELF file follows
const EI_CLASS: u8 = 1;         // Specify 32-bit format
const EI_DATA: u8 = 2;          // Specify BIG endian
const EI_VERSION: u8 = 1;       // Set original version for first iteration of constructing relocatable
const EI_OSABI: u8 = 0;         // Specify System V IBA (specified in original ELF TIS)
const EI_ABIVERSION: u8 = 0;    // Not needed
const EI_PAD: [u8; 7] = [0, 0, 0, 0, 0, 0, 0];  // Built-in padding
const EI_NIDENT: usize = 16;       // Size of this header

// this is the full e_ident field (a complete constant. should never need to be changed).
const E_IDENT_DEFAULT: [u8; EI_NIDENT] = [EI_MAG[0], EI_MAG[1], EI_MAG[2], EI_MAG[3], EI_CLASS, EI_DATA, EI_VERSION, EI_OSABI, EI_ABIVERSION,
    EI_PAD[0], EI_PAD[1], EI_PAD[2], EI_PAD[3], EI_PAD[4], EI_PAD[5], EI_PAD[6]];


// I am defining all feasible ET modes for later.
// const ET_NONE: u16 = 0;
const ET_REL: u16 = 1;
// const ET_EXEC: u16 = 2;
// const ET_DYN: u16 = 3;

// all ELFs will first be constructed with e_type set to ET_REL. The linker handles any changes.
const E_TYPE_DEFAULT: u16 = ET_REL;

// the e_machine field value 8 represents the MIPS instruction set.
const E_MACHINE_DEFAULT: u16 = 8;

// versioning begins at 1.
const E_VERSION_DEFAULT: u32 = 1;

// for relocatable object files, e_entry is handled by the linker. For now, it will be set to 0.
const E_ENTRY_DEFAULT: u32 = 0;

// the e_phoff field can be known ahead of time, since the program header follows the ELF header by convention.
// it is simply the size of the elf header in bytes, as is already specified to be 52 for a 32-bit executable.
const E_PHOFF_DEFAULT: u32 = 52;

// by convention, the section header follows all sections and any other headers. its value cannot be known ahead of time.
// thus, the e_shoff field must be filled in after the full module has been assembled.

// for MIPS, the MIPS ELF specification lays out the e_flags field as follows (irrelevant fields commented out but retained for completeness):
const EF_MIPS_NONREORDER: u32 = 0x00000001;         // No reordering of code to be done by assembler (better for education)
// const EF_MIPS_PIC: u32 = 0x00000002;             // Contains position-independent code
// const EF_MIPS_CPIC: u32 = 0x00000004;            // The PIC present follows standard conventions for calling PIC
// const EF_MIPS_UCODE: u32 = 0x00000010;           // This file contains UCODE (obsolete)
const EF_MIPS_ABI2: u32 = 0x00000020;               // This file has an EI_CLASS of ELFCLASS32 (ours does)
// const EF_MIPS_OPTIONS_FIRST: u32 = 0x00000080;   // The .MIPS.options section in this file contains descriptors for ld. (not us)
// const EF_MIPS_ARCH_ASE: u32 = 0x0f000000;        // Architectural extensions below are present:
// const EF_MIPS_ARCH_ASE_MDMX: u32 = 0x08000000;   // Uses MDMX multimedia extensions
// const EF_MIPS_ARCH_ASE_M16: u32 = 0x04000000;    // Uses MIPS-16 ISA extensions (we implement some of these as pseudo-instructions)
const EF_MIPS_ARCH: u32 = 0x30000000;               // Architecture of the code (mips version) version IV is implmented in NAME

// The bitwise-or combination of selected flags gives the proper e_flags.
const E_FLAGS_DEFAULT: u32 = EF_MIPS_ARCH | EF_MIPS_ABI2 | EF_MIPS_NONREORDER;

// As stated, the ELF header size is known to be 52 for 32-bit binaries.
const E_EHSIZE_DEFAULT: u16 = 52;

// Similarly, the program header entry size is known to be 32 for 32-bit binaries. It's from the way the struct is defined.
const E_PHENTSIZE_DEFAULT: u16 = 32;

// For our use case, the number of entries in the program header is known.
// Each object file we assemble prior to linking will have 1 entry for .text, and 1 entry for .data. (only loadable semgents)
const E_PHNUM_DEFAULT: u16 = 2;

// Just like the other sizes, e_shentsize is known because it's derived from the struct.
const E_SHENTSIZE_DEFAULT: u16 = 40;

// For our use case, e_shnum is known.
// Each object file we assemble needs all program header entries, along with 2 entries for .debug and .line (debug/lineinfo). (plus 1 for null)
const E_SHNUM_DEFAULT: u16 = NUM_OF_SECTIONS as u16;

// By convention, e_shstrndx is set to the last value in the section header. The first index is reserved
const E_SHSTRNDX_DEFAULT: u16 = E_SHNUM_DEFAULT - 1;

// Program header consts

// Program header types
// const PT_NULL: u32 = 0;
const PT_LOAD: u32 = 1;

// pt_flags
const PF_X: u32 = 0x1;  // Execute
const PF_W: u32 = 0x2;  // Write
const PF_R: u32 = 0x4;  // Read

// Section header consts

// section header type indicators
const SHT_NULL: u32 = 0;
const SHT_PROGBITS: u32 = 1;
const SHT_STRTAB: u32 = 3;

// sh_flags  (unused commented out):
const SHF_WRITE: u32 = 0x1;             // writable
const SHF_ALLOC: u32 = 0x2;             // occupies memory during construction
const SHF_EXECINSTR: u32 = 0x4;         // executable
// const SHF_MERGE: u32 = 0x10;         // might be merged
const SHF_STRINGS: u32 = 0x20;          // contains null-term strings
// const SHF_INFO_LINK: u32 = 0x40;     // sh_info contains SHT index
// const SHF_LINK_ORDER: u32 = 0x80;    // preserve order after combining
// const SHF_OS_NONCONFORMING: u32 = 0x100; // non-standard OS handling required
// const SHF_GROUP: u32 = 0x200;        // section is a member of a group
// const SHF_TLS: u32 = 0x400;          // section holds thread-local data

// Structs

// This struct for the ELF file header was derived from information found at https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
// as well as information from the original TIS ELF specification.
#[repr(C)]                  // Used to avoid aligment issues. Not sure it's necessary but honestly better safe than sorry in this case.
#[derive(Debug, Default, Clone, Copy)]
struct Elf32Header{
    e_ident: [u8; 16],      // Contains { {EI_MAG0 .. EIMAG3}, EI_CLASS, EI_DATA, EI_VERSION, EI_OSABI, EI_ABIVERSION, EI_PAD}
    e_type: u16,            // Identifies object file type (ET_REL before linking, ET_EXEC after)
    e_machine: u16,         // Target ISA
    e_version: u32,         // ELF version (will be 1 prior to linking, and incremented to 2 once linking is complete)
    e_entry: u32,           // Address of program entry point
    e_phoff: u32,           // Program header offset
    e_shoff: u32,           // Section header offset
    e_flags: u32,           // Flags are interpreted differently based on OS. For our use case, I don't yet know what they must be.
    e_ehsize: u16,          // Size of ELF Header (changes based on 32/64b arch)
    e_phentsize: u16,       // Size of program header entry
    e_phnum: u16,           // Number of entries in program header
    e_shentsize: u16,       // Size of section header entry
    e_shnum: u16,           // Number of entries in header entry
    e_shstrndx: u16,        // Index of section header containing section names (section header string index)
}

// This associated function serializes the struct to bytes. This is for writing to file.
impl Elf32Header {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec!();
        // Append each field to that byte vector
        bytes.extend_from_slice(&self.e_ident);
        bytes.extend_from_slice(&self.e_type.to_be_bytes());
        bytes.extend_from_slice(&self.e_machine.to_be_bytes());
        bytes.extend_from_slice(&self.e_version.to_be_bytes());
        bytes.extend_from_slice(&self.e_entry.to_be_bytes());
        bytes.extend_from_slice(&self.e_phoff.to_be_bytes());
        bytes.extend_from_slice(&self.e_shoff.to_be_bytes());
        bytes.extend_from_slice(&self.e_flags.to_be_bytes());
        bytes.extend_from_slice(&self.e_ehsize.to_be_bytes());
        bytes.extend_from_slice(&self.e_phentsize.to_be_bytes());
        bytes.extend_from_slice(&self.e_phnum.to_be_bytes());
        bytes.extend_from_slice(&self.e_shentsize.to_be_bytes());
        bytes.extend_from_slice(&self.e_shnum.to_be_bytes());
        bytes.extend_from_slice(&self.e_shstrndx.to_be_bytes());

        bytes
    }
}

// This struct was also derived from https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
// It will be found at e_phoff, and have e_phnum entries, each of size e_phentsize. That's why we specified it prior.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct Elf32ProgramHeader{
    p_type: u32,            // Indicates type. PT_LOAD signals a loadable segment (.text, .rodata, .data, etc.)
    p_offset: u32,          // Offset of segment in the file image
    p_vaddr: u32,           // Virtual address of segment in memory
    p_paddr: u32,           // Reserved for physical address of segment in memory (likely not used in our use case)
    p_filesz: u32,          // Size in bytes of the segment in the file image, can be 0 (but why?)
    p_memsz: u32,           // Size in bytes of the segment in memory, can be 0 for non-loaded (PT_NULL) segments
    p_flags: u32,           // Segment dependent flags: PF_R = read, PF_W = write, PF_X = execute. All three should NEVER be specified, though that's not enforced explicitly in my implmentation. WX is also generally not a good plan.
    p_align: u32,           // 0 and 1 specify no alignment, but positive powers of 2 specify p_vaddr = p_offset % p_align
}

// Similarly serialize Elf32ProgramHeader to bytes for writing to file
impl Elf32ProgramHeader{
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec!();
        // Append all fields to bytes vec
        bytes.extend_from_slice(&self.p_type.to_be_bytes());        // To Big-Endian bytes
        bytes.extend_from_slice(&self.p_offset.to_be_bytes());
        bytes.extend_from_slice(&self.p_vaddr.to_be_bytes());
        bytes.extend_from_slice(&self.p_paddr.to_be_bytes());
        bytes.extend_from_slice(&self.p_filesz.to_be_bytes());
        bytes.extend_from_slice(&self.p_memsz.to_be_bytes());
        bytes.extend_from_slice(&self.p_flags.to_be_bytes());
        bytes.extend_from_slice(&self.p_align.to_be_bytes());

        bytes
    }
}

// This struct was indeed also derived from https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
// It will be found at e_shoff, and have e_shnum entries, each of size e_shentsize. the string representation of the name of each entry is found at e_shstrndx.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct Elf32SectionHeader{
    sh_name: u32,           // Offset to .shstrtab section containing name of this section
    sh_type: u32,           // Type of header. SHT_PROGBITS signals program data, SHT_SYMTAB the symbol table, etc.
    sh_flags: u32,          // Identifies section attributes: see const definitions
    sh_addr: u32,           // Vaddr of section in memory (for loaded sections)
    sh_offset: u32,         // Offset of the section in file image
    sh_size: u32,           // Size in bytes of the section in the file image
    sh_link: u32,           // Section index of section
    sh_info: u32,           // Extra info on section
    sh_addralign: u32,      // Required alignment of section. Must be a power of two
    sh_entsize: u32,        // Size in bytes of each entry for sections that contain fixed-size entries (think tables)
}

impl Elf32SectionHeader {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec!();
        // Append all fields to bytes vec
        bytes.extend_from_slice(&self.sh_name.to_be_bytes());        // To Big-Endian bytes
        bytes.extend_from_slice(&self.sh_type.to_be_bytes());
        bytes.extend_from_slice(&self.sh_flags.to_be_bytes());
        bytes.extend_from_slice(&self.sh_addr.to_be_bytes());
        bytes.extend_from_slice(&self.sh_offset.to_be_bytes());
        bytes.extend_from_slice(&self.sh_size.to_be_bytes());
        bytes.extend_from_slice(&self.sh_link.to_be_bytes());
        bytes.extend_from_slice(&self.sh_info.to_be_bytes());
        bytes.extend_from_slice(&self.sh_addralign.to_be_bytes());
        bytes.extend_from_slice(&self.sh_entsize.to_be_bytes());

        bytes
    }
}

// To construct an ET_REL ELF file, we'll use the following struct:
#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RelocatableElf{
    file_header: Elf32Header,
    program_header_table: Vec<Elf32ProgramHeader>,
    sections: Vec<Vec<u8>>,
    section_header_table: Vec<Elf32SectionHeader>,
}

// Functions

// Create a new ET_REL ELF file header with default values
// takes parameters passed_e_entry, the entry point of the program,
// and passed_e_shoff, the section header offset calculated in a separate method.
fn create_new_et_rel_file_header(passed_e_shoff: u32) -> Elf32Header {
    Elf32Header{
        e_ident: E_IDENT_DEFAULT,
        e_type: E_TYPE_DEFAULT,
        e_machine: E_MACHINE_DEFAULT,
        e_version: E_VERSION_DEFAULT,
        e_entry: E_ENTRY_DEFAULT,
        e_phoff: E_PHOFF_DEFAULT,
        e_shoff: passed_e_shoff,
        e_flags: E_FLAGS_DEFAULT,
        e_ehsize: E_EHSIZE_DEFAULT,
        e_phentsize: E_PHENTSIZE_DEFAULT,
        e_phnum: E_PHNUM_DEFAULT,
        e_shentsize: E_SHENTSIZE_DEFAULT,
        e_shnum: E_SHNUM_DEFAULT,
        e_shstrndx: E_SHSTRNDX_DEFAULT,
    }
}

// This function combines all the previous to actually create a new object file.
pub fn create_new_et_rel(text_section: Vec<u8>, data_section: Vec<u8>, debug_section: Vec<u8>, line_section: Vec<u8>) -> RelocatableElf {
    // The section header string table entry requires some calculations.
    // Here we get the shstrtab as bytes from the constant defined at the top of the file.
    // We also get the size of the shstrtab.
    let mut shstrtab_section: Vec<u8> = vec!();
    for item in SECTIONS {
        shstrtab_section.extend_from_slice(item.as_bytes());
        shstrtab_section.extend_from_slice(&[b'\0']);
    }
    let shstrtab_size: u32 = shstrtab_section.len() as u32;

    // Get size of each section to properly calculate offsets in result file
    let text_size: u32 = text_section.len() as u32;
    let data_size: u32 = data_section.len() as u32;
    let debug_size: u32 = debug_section.len() as u32;
    let line_size: u32 = line_section.len() as u32;

    // Calculate offsets using sizes
    let text_offset: u32 = E_PHOFF_DEFAULT + (E_PHNUM_DEFAULT * E_PHENTSIZE_DEFAULT) as u32;     // The program header entries are for the two loadable segments, .text and .data
    let data_offset: u32 = text_offset + text_size;
    let debug_offset: u32 = data_offset + data_size;
    let line_offset: u32 = debug_offset + debug_size;
    let shstrtab_offset: u32 = line_offset + line_size;
    let sh_offset = shstrtab_offset + shstrtab_size;

    // Construct the ELF file header
    let elf_file_header: Elf32Header = create_new_et_rel_file_header(sh_offset);

    // Populate the program headers - by MIPS convention, section .text should be at 0x00400000 and section .data at 0x10000000
    let text_ph: Elf32ProgramHeader = Elf32ProgramHeader {
        p_type: PT_LOAD,
        p_offset: text_offset,
        p_vaddr: MIPS_TEXT_START_ADDR,
        p_paddr: MIPS_TEXT_START_ADDR,
        p_filesz: text_size,
        p_memsz: text_size,
        p_flags: PF_R | PF_X,   // section .text should not be writable
        p_align: MIPS_ALIGNMENT,
    };

    let data_ph: Elf32ProgramHeader = Elf32ProgramHeader {
        p_type: PT_LOAD,
        p_offset: data_offset,
        p_vaddr: MIPS_DATA_START_ADDR,
        p_paddr: MIPS_DATA_START_ADDR,
        p_filesz: data_size,
        p_memsz: data_size,
        p_flags: PF_R | PF_W,   // section .data should not be executable
        p_align: MIPS_ALIGNMENT,
    };

    // Construct program header table
    let complete_program_header_table: Vec<Elf32ProgramHeader> = vec![text_ph, data_ph];

    // Populate the section headers - indexes are in the same order as the struct (.text, .data, .debug, .line)
    // First field is SHT_NULL and reserved, but must be included.
    let null_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: 0,     // This is a byte index
        sh_type: SHT_NULL,
        sh_flags: 0,
        sh_addr: 0,
        sh_offset: 0,
        sh_size: 0,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 0,
        sh_entsize: 0,
    };

    let text_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: 1,
        sh_type: SHT_PROGBITS,
        sh_flags: SHF_ALLOC | SHF_EXECINSTR,    // Allocated and executable
        sh_addr: MIPS_TEXT_START_ADDR,          // Implicit virtual address
        sh_offset: text_offset,
        sh_size: text_size,
        sh_link: 0, // Unused
        sh_info: 0, // Unused
        sh_addralign: MIPS_ADDRESS_ALIGNMENT,
        sh_entsize: 0 // Unused in this section
    };

    let data_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: text_sh.sh_name + SECTIONS[1].len() as u32 + 1,
        sh_type: SHT_PROGBITS,
        sh_flags: SHF_ALLOC | SHF_WRITE,    // Allocated and writeable
        sh_addr: MIPS_DATA_START_ADDR,
        sh_offset: data_offset,
        sh_size: data_size,
        sh_link: 0, // Unused
        sh_info: 0, // Unused
        sh_addralign: MIPS_ADDRESS_ALIGNMENT,
        sh_entsize: 0, // Unused in this section
    };

    let debug_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: data_sh.sh_name + SECTIONS[2].len() as u32 + 1,
        sh_type: SHT_PROGBITS,
        sh_flags: SHF_STRINGS,  // The debug section for NAME contains null-term strings
        sh_addr: 0,             // Not loaded
        sh_offset: debug_offset,
        sh_size: debug_size,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 0,
        sh_entsize: 0,
    };

    let line_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: debug_sh.sh_name + SECTIONS[3].len() as u32 + 1,
        sh_type: SHT_PROGBITS,
        sh_flags: SHF_STRINGS,  // The line info for NAME is also null-term strings
        sh_addr: 0,
        sh_offset: line_offset,
        sh_size: line_size,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 0,
        sh_entsize: 0,
    };

    let shstrtab_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: line_sh.sh_name + SECTIONS[4].len() as u32 + 1,
        sh_type: SHT_STRTAB,
        sh_flags: SHF_STRINGS,
        sh_addr: 0,
        sh_offset: shstrtab_offset,
        sh_size: shstrtab_size,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 0,
        sh_entsize: 0,
    };

    // Collect all previously defined section headers into the section header table
    let complete_section_header_table: Vec<Elf32SectionHeader> = vec![null_sh, text_sh, data_sh, debug_sh, line_sh, shstrtab_sh];

    // Collect all sections into the sections vector
    let complete_sections: Vec<Vec<u8>> = vec![text_section, data_section, debug_section, line_section, shstrtab_section];

    // Final step is to create the final ElfRelocatable struct
    return RelocatableElf{
        file_header: elf_file_header,
        program_header_table: complete_program_header_table,
        sections: complete_sections,
        section_header_table: complete_section_header_table,
    }
}

// This function creates a new file with the passed name and writes all bytes in a RelocatableElf object
pub fn write_et_rel_to_file(file_name: &str, et_rel: &RelocatableElf) -> Result<(), String> {
    // Declare file_bytes vector to push all these file bytes onto

    // Concatenate all bytes in file header
    let mut file_bytes: Vec<u8> = et_rel.file_header.to_bytes().to_vec();

    // Get all bytes in program header table
    for entry in &et_rel.program_header_table {
        file_bytes.extend(&entry.to_bytes());
    }

    // Add all sections
    for section in &et_rel.sections {
        file_bytes.extend(section);
    }

    // Section header table
    for entry in &et_rel.section_header_table {
        file_bytes.extend_from_slice(&entry.to_bytes());
    }

    // Write file bytes to output file
    let mut f: fs::File = fs::File::create(file_name).expect("Unable to write file");       // This is really bad and insecure for right now - path MUST be checked before this gets out of alpha
    f.write_all(&file_bytes).expect("Unable to write data.");

    Ok(())
}