// Utilities to assemble to ELF.
// This file contains relevant structs and methods to produce valid ET_REL and ET_EXEC files.
pub mod elf_utils;

// Imports
use std::vec::Vec;                          // Used for ELF sections

// The following data definitions are required to construct ELF files.
// The meaning of each field is detailed in the appropriate struct.

// Consts

// Constants pertaining to MIPS conventions
const MIPS_TEXT_START_ADDR: usize = 0x00400000; // The address at which, by convention, MIPS begins the .text section
const MIPS_DATA_START_ADDR: usize = 0x10000000; // The address at which, by convention, MIPS begins the .data section (I really typed this out again!)
const MIPS_ALIGNMENT: usize = 0x1000;           // The appropriate alignment for MIPS executables (from all my research)
const MIPS_ADDRESS_ALIGNMENT: usize = 4;        // MIPS is aligned by 4-byte word

// ELF File Header fields

// All of the following pertain to the e_ident field, and can be edited granularly below.
const EI_MAG: [u8; 4] = [0x7F, b'E', b'L', b'F'];  // ELF magic bytes to signal that an ELF file follows
const EI_CLASS: u8 = 1;         // Specify 32-bit format
const EI_DATA: u8 = 2;          // Specify BIG endian
const EI_VERSION: u8 = 1;       // Set original version for first iteration of constructing relocatable
const EI_OSABI: u8 = 0;         // Specify System V IBA (specified in original ELF TIS)
const EI_ABIVERSION: u8 = 0;    // Not needed
const EI_PAD: [u8; 7] = [0, 0, 0, 0, 0, 0, 0];  // Built-in padding
const EI_NIDENT: u8 = 16;       // Size of this header

// this is the full e_ident field (a complete constant. should never need to be changed).
const e_ident_default: [u8; EI_NIDENT] = [EI_MAG[0], EI_MAG[1], EI_MAG[2], EI_MAG[3], EI_CLASS, EI_DATA, EI_VERSION, EI_OSABI, EI_ABIVERSION, EI_PAD];


// I am defining all feasible ET modes for later.
const ET_NONE: u16 = 0;
const ET_REL: u16 = 1;
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
// all ELFs will first be constructed with e_type set to ET_REL. The linker handles any changes.
const e_type_default: u16 = ET_REL;

// the e_machine field value 8 represents the MIPS instruction set.
const e_machine_default: u16 = 8;

// versioning begins at 1.
const e_version_default: u32 = 1;

// for relocatable object files, e_entry is handled by the linker. For now, it will be set to 0.
const e_entry_default: u32 = 0;

// the e_phoff field can be known ahead of time, since the program header follows the ELF header by convention.
// it is simply the size of the elf header in bytes, as is already specified to be 52 for a 32-bit executable.
const e_phoff_default: u32 = 52;

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
const e_flags_default: u32 = EF_MIPS_ARCH | EF_MIPS_ABI2 | EF_MIPS_NONREORDER;

// As stated, the ELF header size is known to be 52 for 32-bit binaries.
const e_ehsize_default: u16 = 52;

// Similarly, the program header entry size is known to be 32 for 32-bit binaries. It's from the way the struct is defined.
const e_phentsize_default: u16 = 32;

// For our use case, the number of entries in the program header is known.
// Each object file we assemble prior to linking will have 1 entry for .text, and 1 entry for .data. (only loadable semgents)
const e_phnum_default: u16 = 2;

// Just like the other sizes, e_shentsize is known because it's derived from the struct.
const e_shentsize_default: u16 = 40;

// For our use case, e_shnum is known.
// Each object file we assemble needs all program header entries, along with 2 entries for .debug and .line (debug/lineinfo).
const e_shnum_default:u16 = 4;

// By convention, e_shstrndx is set to the last value in the section header. The section header is effectively 1-indexed since the first index is reserved
const e_shstrndx_default:u16 = e_shnum_default + 1;

// Section header consts

// section header type indicators
const SHT_NULL: usize = 0;
const SHT_PROGBITS: usize = 1;
const SHT_STRTAB: usize = 3;

// sh_flags  (unused commented out):
const SHF_WRITE: u32 = 0x1;             // writable
const SHF_ALLOC: u32 = 0x2;             // occupies memory during construction
const SHF_EXECINSTR: u32 = 0x4;         // executable
const SHF_MERGE: u32 = 0x10;            // might be merged
const SHF_STRINGS: u32 = 0x20;          // contains null-term strings
const SHF_INFO_LINK: u32 = 0x40;        // sh_info contains SHT index
const SHF_LINK_ORDER: u32 = 0x80;       // preserve order after combining
const SHF_OS_NONCONFORMING: u32 = 0x100; // non-standard OS handling required
const SHF_GROUP: u32 = 0x200;           // section is a member of a group
const SHF_TLS: u32 = 0x400;             // section holds thread-local data

// Section header string table consts
const SECTION_HEADER_STRING_TABLE_BYTES: [u8; 24] = [
    b'.', b't', b'e', b'x', b't', 0x00, 
    b'.', b'd', b'a', b't', b'a', 0x00, 
    b'.', b'd', b'e', b'b', b'u', b'g', 0x00, 
    b'.', b'l', b'i', b'n', b'e', 0x00,
    ];

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
    p_flags: u32,           // Segment dependent flags: PF_R = read, PF_W = write, PF_X = execute. All three should NEVER be specified, though that's not enforced in my implmentation. WX is also generally not a good plan.
    p_align: u32,           // 0 and 1 specify no alignment, but positive powers of 2 specify p_vaddr = p_offset % p_align
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

// To construct an ET_REL ELF file, we'll use the following struct:
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct RelocatableElf{
    file_header: Elf32Header,
    program_header_table: Vec<Elf32ProgramHeader>,
    section_dot_text: Vec<u32>,
    section_dot_data: Vec<u8>,
    section_dot_debug: Vec<u8>,
    section_dot_line: Vec<u8>,
    section_dot_shstrtab: Vec<u8>,
    section_header_table: Vec<Elf32SectionHeader>,
}

// Create a new ET_REL ELF file header with default values
// takes parameters passed_e_entry, the entry point of the program,
// and passed_e_shoff, the section header offset calculated in a separate method.
fn create_new_et_rel_file_header(passed_e_shoff: u32) -> Elf32Header {
    Elf32Header{
        e_ident: e_ident_default,
        e_type: e_type_default,
        e_machine: e_machine_default,
        e_version: e_version_default,
        e_entry: _e_entry_default,
        e_phoff: e_phoff_default,
        e_shoff: passed_e_shoff,
        e_flags: e_flags_default,
        e_ehsize: e_ehsize_default,
        e_phentsize: e_phentsize_default,
        e_phnum: e_phnum_default,
        e_shentsize: e_shentsize_default,
        e_shnum: e_shnum_default,
        e_shstrndx: e_shstrndx_default,
    }
}

// This function combines all the previous to actually create a new object file.
fn create_new_et_rel(text_section: Vec<u32>, data_section: Vec<u8>, debug_section: Vec<u8>, line_section: Vec<u8>) -> RelocatableElf {
    // Get size of each section to properly calculate offsets in result file
    let text_size: u32 = (text_section.len() * 4);  // The 4 gives us the size of the text section in bytes since it's a u32 (word) vector
    let data_size: u32 = (data_section.len() + 3) >> 2 << 2;    // Rouding up to nearest multiple of 4 with bitwise operations so that we avoid aligment issues later
    let debug_size: u32 = debug_section.len();
    let line_size: u32 = line_section.len();

    // Calculate offsets using sizes
    let text_offset: u32 = e_phoff_default + (2 * e_phentsize_default);     // The two program header entries are for the two loadable segments, .text and .data
    let data_offset: u32 = text_offset + text_size;
    let debug_offset: u32 = data_offset + data_size;
    let line_offset: u32 = debug_offset + debug_size;
    let sh_offset = line_offset + line_size;

    // Construct the ELF file header (1/8)
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

    // Construct program header table (2/8)
    let program_header_table_complete: Vec<Elf32ProgramHeader> = vec![text_ph, data_ph];

    // (3, 4, 5, 6, 7) / 8 are passed

    // Populate the section headers - indexes are in the same order as the struct (.text, .data, .debug, .line)
    // First field is SHT_NULL and reserved, but must be included.
    let null_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: 0,
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
        sh_name: 2,
        sh_type: SHT_PROGBITS,
        sh_flags: SHF_ALLOC | SHF_WRITE,    // Allocated and writeable
        sh_addr: MIPS_DATA_START_ADDR,
        sh_offset: data_offset,
        sh_size: data_size,
        sh_link: 0, // Unused
        sh_info: 0, // Unused
        sh_addralin: MIPS_ADDRESS_ALIGNMENT,
        sh_entsize: 0, // Unused in this section
    };

    let debug_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: 3,
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
        sh_name: 4,
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
        sh_name: 5,
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

    // Collect all previously defined section headers into the section header table (8/8)
    let complete_section_header_table: Vec<Elf32SectionHeader> = vec![text_sh, data_sh, debug_sh, line_sh, shstrtab_sh];

    // Final step is to create the final ElfRelocatable struct
    return RelocatableElf{
        file_header: elf_file_header,
        program_header_table: complete_program_header_table,
        section_dot_text: text_section,
        section_dot_data: data_section,
        section_dot_debug: debug_section,
        section_dot_line: line_section,
        section_dot_shstrtab: SECTION_HEADER_STRING_TABLE_BYTES,
        section_header_table: complete_section_header_table,
    }
}