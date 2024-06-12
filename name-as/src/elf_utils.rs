// Utilities to assemble to ELF.
// This file contains relevant structs and methods to produce valid ET_REL and ET_EXEC files.

// This struct was derived from information found at https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
#[repr(C)]                                  // Used to avoid aligment issues. Not sure it's necessary but honestly better safe than sorry in this case.
#[derive(Debug, Default, Clone, Copy)]
struct Elf32Header{
    e_ident: [u8; 10],      // Contains { {EI_MAG0 .. EIMAG3}, EI_CLASS, EI_DATA, EI_VERSION, EI_OSABI, EI_ABIVERSION, EI_PAD}
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

/*
sh_flags:
p
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