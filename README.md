# NAME

![logo](logo/logo.png)

NAME ("Not Another MIPS Emulator") is a MIPS assembly code emulation pipeline designed for educational use.

While this implementation focuses on MIPS, a fork of the project could feasibly produce an implementation for any other asm.

## Goals

NAME accomplishes a modular approach to assembly code emulation by dividing and conquering three crucial elements:

1. **Assembling** - accomplished by [name-as](name-as), a configurable assembler framework 
2. **Emulation** - accomplished by [name-emu](name-emu), an extensible framework for developing CPU emulators
3. **Development** - accomplished by 
  - [name-ext](name-ext), a VSCode integration for assembly development complete with a [DAP](https://microsoft.github.io/debug-adapter-protocol//) and [IntelliSense](https://learn.microsoft.com/en-us/visualstudio/ide/using-intellisense) for insight into emulated CPU cores
  - [name-fmt](name-fmt) a VSCode extension for canonical assembly formatting

## Building From Source

NAME is a vscode extension, which means it is built with typescript. Additionally, NAME uses Rust binaries to function. To ensure you can build from source, confirm you have installed the following software:
 - nodejs
 - rust

#### Common Problems Building From Source

In its current state, when building from source, NAME will not function unless `npm run build` has been executed in the `name-ext` directory.

## Test Files

Some test files have been included. You can find them in [test files](test_files/test_files.md).

## Assembly
The NAME MIPS assembler uses a few key steps:

#### Parsing
Parsing, in its current state, is implmented in [parser.rs](name-as/src/parser.rs). It uses [pest](https://pest.rs/) grammar rules to extract:
 - MIPS Vernacular (anything non-comment nor whitespace)
  - Instructions (mnemonic + operands)
  - Labels (identifiers + "\:")
  - Directives ("." + identifiers)
 - Identifiers (alphanumeric sequences not separated by space)

These extracted patterns are handled by the assembler.

This parser is changing very soon.

#### Assembling
The assembler takes each instruction, matches its mnemonic against the implemented set, and uses the operands to construct the appropriate MIPS instruction in binary form. This 4-byte word is then split into big-endian bytes and pushed to a vector. That vector is collected to create the .text segment for the assembled executable.

NAME operates using the ELF file format and associated conventions. If unfamiliar, read more [here](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format). NAME assembles each module in the user's directory into an ET_REL relocatable object file for the linker to handle, using code found in [elf_utils.rs](name-as/src/elf_utils.rs) and [nma.rs](name-as/src/nma.rs). Each ET_REL has the same following sections, present in this order:
 - Reserved null section
 - `.text`
 - `.data`
 - `.symtab`
 - `.strtab`
 - `.debug`
 - `.line`
 - `.shstrtab`

The rationale behind using ELF files is to provide students with observation opportunity less abstract than a typical classroom approach. Since NAME's primary purpose is to educate, a black box approach will not be as helpful at a collegiate level. ELF files are not only standardized, but also found in the real world - learning how to work with them can help subdue the apprehension most students feel in their first UNIX classes.

For a `.asm` file to assemble, it must contain a section `.text`. The assembler does not assume that a section `.text` is present. To be as clear as possible: 

**The `.text` directive is *required* for files to assemble.**

#### Linking
The linker ([linker.rs](name-as/src/linker.rs)) reads relocatable object files from disk and constructs them into RelocatableElf objects. This choice was made for two intertwined reasons - modularity and educational value. Keeping the linker separate allows for better classroom demonstrations regarding how object files are linked into a single executable, and since ELF is a standard format, there is no magical black box for the sake of the classroom; rather, the linking process is entirely accurate to other programs, allowing students to make better global connections.

The convention when linking into an executable is to search the symbol table for the global symbol `main`; if found, that is the entry point, but if `main` is not present, the entry point defaults to the MIPS .text start address: `0x4000000`. Conflicting global symbols raise an error, but duplicate local symbols are of course allowed. All linking is performed to the ELF TIS (Tool Interface Standard).

The linker outputs a single file with no extension - an ET_EXEC ELF executable that [name-emu](name-emu/) can interpret.

## Emulation
NAME's emulator, in its current state, is largely integrated with vscode, though the eventual goal is some degree of separation.

[mips.rs](name-emu/src/mips.rs) contains the MIPS object. The MIPS object contains the registers, data, and associated program information for an assembled executable.