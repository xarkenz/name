// Utilities to assemble to ELF.

// Imports
use std::{fs, io::Write, path::PathBuf, vec::Vec};

use crate::elf_def::*;
use crate::structs::{Section, Symbol, Visibility};    // Used for ELF sections

// Macros - had to learn somehow!

macro_rules! ELF32_ST_INFO {
    ($bind:expr, $type:expr) => {
        (($bind << 4) + ($type & 0xf)) as u8
    };
}

/*

 __          _______  _____ _______ ______ 
 \ \        / /  __ \|_   _|__   __|  ____|
  \ \  /\  / /| |__) | | |    | |  | |__   
   \ \/  \/ / |  _  /  | |    | |  |  __|  
    \  /\  /  | | \ \ _| |_   | |  | |____ 
     \/  \/   |_|  \_\_____|  |_|  |______|
                                           
                                           

*/

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
pub fn create_new_et_rel(text_section: Vec<u8>, data_section: Vec<u8>, symtab_section: Vec<u8>, strtab_section: Vec<u8>) -> RelocatableElf {
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
    let symtab_size: u32 = symtab_section.len() as u32;
    let strtab_size: u32 = strtab_section.len() as u32;

    // Calculate offsets using sizes
    let text_offset: u32 = E_PHOFF_DEFAULT + (E_PHNUM_DEFAULT * E_PHENTSIZE_DEFAULT) as u32;     // The program header entries are for the two loadable segments, .text and .data
    let data_offset: u32 = text_offset + text_size;
    let symtab_offset: u32 = data_offset + data_size;
    let strtab_offset: u32 = symtab_offset + symtab_size;
    let shstrtab_offset: u32 = strtab_offset + strtab_size;
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

    let symtab_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: data_sh.sh_name + SECTIONS[2].len() as u32 + 1,
        sh_type: SHT_SYMTAB,
        sh_flags: 0,  // The symtab does not have any flags associated
        sh_addr: 0,
        sh_offset: symtab_offset,
        sh_size: symtab_size,
        sh_link: 4,             // Link to appropriate string table
        sh_info: 0,
        sh_addralign: 0,
        sh_entsize: SH_ENTSIZE_SYMTAB,
    };

    let strtab_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: symtab_sh.sh_name + SECTIONS[3].len() as u32 + 1,
        sh_type: SHT_STRTAB,
        sh_flags: SHF_STRINGS,
        sh_addr: 0,
        sh_offset: strtab_offset,
        sh_size: strtab_size,
        sh_link: 0,
        sh_info: 0,
        sh_addralign: 0,
        sh_entsize: 0,
    };

    let shstrtab_sh: Elf32SectionHeader = Elf32SectionHeader {
        sh_name: strtab_sh.sh_name + SECTIONS[4].len() as u32 + 1,
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
    let complete_section_header_table: Vec<Elf32SectionHeader> = vec![null_sh, text_sh, data_sh, symtab_sh, strtab_sh, shstrtab_sh];

    // Final step is to create the final ElfRelocatable struct
    return RelocatableElf{
        file_header: elf_file_header,
        program_header_table: complete_program_header_table,
        section_header_table: complete_section_header_table,
        section_dot_text: text_section,
        section_dot_data: data_section,
        section_dot_symtab: symtab_section,
        section_dot_strtab: strtab_section,
        section_dot_shstrtab: shstrtab_section,
    }
}

// This function creates a new file with the passed name and writes all bytes in a RelocatableElf object
pub fn write_et_rel_to_file(file_name: &PathBuf, et_rel: &RelocatableElf) -> Result<(), String> {
    // Declare file_bytes vector to push all these file bytes onto
    // Concatenate all bytes in file header
    let mut file_bytes: Vec<u8> = et_rel.file_header.to_bytes().to_vec();

    // Get all bytes in program header table
    for entry in &et_rel.program_header_table {
        file_bytes.extend(&entry.to_bytes());
    }

    // Add all sections
    file_bytes.extend(&et_rel.section_dot_text);
    file_bytes.extend(&et_rel.section_dot_data);
    file_bytes.extend(&et_rel.section_dot_symtab);
    file_bytes.extend(&et_rel.section_dot_strtab);
    file_bytes.extend(&et_rel.section_dot_shstrtab);

    // Section header table
    for entry in &et_rel.section_header_table {
        file_bytes.extend_from_slice(&entry.to_bytes());
    }

    // Write file bytes to output file
    let mut f: fs::File = fs::File::create(file_name).expect("Unable to write file");       // This is really bad and insecure for right now - path MUST be checked before this gets out of alpha
    f.write_all(&file_bytes).expect("Unable to write data.");

    Ok(())
}

pub fn convert_symbol_to_elf32sym(symbol: &Symbol, strtab_index: u32) -> Elf32Sym {
    Elf32Sym {
        st_name: strtab_index,
        st_value: symbol.value,
        st_size: symbol.size,
        st_info: match symbol.visibility {
            Visibility::Local => ELF32_ST_INFO!(0, symbol.symbol_type),
            Visibility::Global => ELF32_ST_INFO!(1, symbol.symbol_type),
            Visibility::Weak => ELF32_ST_INFO!(2, symbol.symbol_type),
        },
        st_other: match symbol.visibility {
            Visibility::Local => 2,
            Visibility::Global => 0,
            Visibility::Weak => 0,
        },
        st_shndx: match symbol.section {
            Section::Text => 1,
            Section::Data => 2,
            _ => 0,
        },
    }
}

/*

  _____  ______          _____  
 |  __ \|  ____|   /\   |  __ \ 
 | |__) | |__     /  \  | |  | |
 |  _  /|  __|   / /\ \ | |  | |
 | | \ \| |____ / ____ \| |__| |
 |_|  \_\______/_/    \_\_____/ 
                                
                                

*/