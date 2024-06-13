// Utilities to assemble to ELF.
// This file contains relevant structs and methods to produce valid ET_REL and ET_EXEC files.

use std::collections::HashMap;              // Used to define modes by name

// The following data definitions are required to construct ELF files.
// The meaning of each field is detailed in the appropriate struct.

// ELF File Header fields

// All of the following pertain to the e_ident field, and can be edited granularly below.
const EI_MAG: [u8; 4] = [0x7F, b'E', b'L', b'F'];  // ELF magic bytes to signal that an ELF file follows
const EI_CLASS: u8 = 1;         // Specify 32-bit format
const EI_DATA: u8 = 2;          // Specify BIG endian
const EI_VERSION: u8 = 1;       // Set original version for first iteration of constructing relocatable
const EI_OSABI: u8 = 0;         // Specify System V IBA (specified in original ELF TIS)
const EI_ABIVERSION: u8 = 0;    // Not needed
const EI_PAD: [u8; 7] = [0, 0, 0, 0, 0, 0, 0];  // Built-in padding

// This is the full e_ident field (a complete constant. should never need to be changed).
const e_ident_default: [u8; 16] = [EI_MAG[0], EI_MAG[1], EI_MAG[2], EI_MAG[3], EI_CLASS, EI_DATA, EI_VERSION, EI_OSABI, EI_ABIVERSION, EI_PAD];


// I am defining all feasible ET modes for later.
const ET_NONE: u16 = 0;
const ET_REL: u16 = 1;
const ET_EXEC: u16 = 2;
const ET_DYN: u16 = 3;
// All ELFs will first be constructed with e_type set to ET_REL. The linker handles any changes.
const e_type_default: u16 = ET_REL;

// The e_machine field value 8 represents the MIPS instruction set.
const e_machine_default: u16 = 8;

// the e_entry address cannot be known, as it corresponds to the global main function.
// the linker will need to handle filling in this field.
const e_entry_default: u32 = 0;

// the e_phoff field can be known ahead of time, since the program header follows the ELF header by convention.
// it is simply the size of the elf header in bytes, as is already specified to be 52 for a 32-bit executable.
const e_phoff_default: u32 = 52;

// by convention, the section header follows all sections and any other headers. its value cannot be known ahead of time.
// thus, the e_shoff field must be filled in after the full module has been assembled.
const e_shoff_default: u32 = 0;

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
// Each object file we assemble prior to linking will have 1 entry for .text, 1 entry for .data, and 1 entry for .bss (heap).
const e_phnum_default: u16 = 3;

// Just like the other sizes, e_shentsize is known because it's derived from the struct.
const e_shentsize_default: u16 = 40;

// For our use case, e_shnum is known.
// Each object file we assemble needs all program header entries, along with 1 entry for .note (debug/lineinfo).
const e_shnum_default:u16 = 4;

// By convention, e_shstrndx is set to the last value in the section header. Therefore,
const e_shstrndx_default:u16 = e_shnum_default;

// This struct for the ELF file header was derived from information found at https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
// as well as information from the original TIS ELF specification.
#[repr(C)]                                  // Used to avoid aligment issues. Not sure it's necessary but honestly better safe than sorry in this case.
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
    sh_flags: u32,          // Identifies section attributes: see below comment
    sh_addr: u32,           // Vaddr of section in memory (for loaded sections)
    sh_offset: u32,         // Offset of the section in file image
    sh_size: u32,           // Size in bytes of the section in the file image
    sh_link: u32,           // Section index of section
    sh_info: u32,           // Extra info on section
    sh_addralign: u32,      // Required alignment of section. Must be a power of two
    sh_entsize: u32,        // Size in bytes of each entry for sections that contain fixed-size entries (think tables)
}


fn create_new_elf_file_header() -> Result<Elf32Header, &'static str> {
    
}

/*
sh_flags:

0x1 = SHF_WRITE (writable)
0x2 = SHF_ALLOC (occupies memory during construction)
0x4 = SHF_EXECINSTR (executable)
0x10 = SHF_MERGE (might be merged)
0x20 = SHF_STRINGS (contains null-term strings)
0x40 = SHF_INFO_LINK (sh_info contains SHT index)
0x80 = SHF_LINK_ORDER (preserve order after combining)
0x100 = SHF_OS_NONCONFORMING (non-standard OS handling required)
0x200 = SHF_GROUP (section is a member of a group)
0x400 = SHF_TLS (section holds thread-local data)
*/