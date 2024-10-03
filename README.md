# NAME

![logo](logo/logo.png)

NAME ("Not Another MIPS Emulator") is a MIPS assembly code emulation pipeline designed for educational use. It contains a MIPS assembler, linker, emulator, and VSCode development extension. The first three tools can be used entirely from the command line with `cargo`.

**Note** that while this implementation focuses on MIPS, in particular [this](https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MD00086-2B-MIPS32BIS-AFP-6.06.pdf) TIS, a fork of this project could feasibly produce an implementation for any other asm.

NAME operates using the ELF file format and associated conventions. If unfamiliar, read more [here](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format).

The rationale behind using ELF files is to provide students with observation opportunity less abstract than a typical classroom approach. NAME's primary purpose is to educate, and since a black box approach will not be as helpful at a collegiate level, it follows that the de-abstraction should be accurate to real-world implmentations. ELF files are not only standardized, but also found in the real world - learning how to work with them earlier in the degree plan can help subdue the apprehension most students feel in their first UNIX classes. Students can even use tools such as `readelf -a` and `mips-linux-gnu-objdump -M reg_names=32 -d` to examine the files they themselves produce, stimulating exploration and experimentation.

**Note** that for a `.asm` file to assemble, it must contain a section `.text`. The assembler does not assume that a section `.text` is present. To be as clear as possible:

**The `.text` directive is *required* for your files to assemble.**

## Goals

NAME accomplishes a modular approach to assembly code emulation by dividing and conquering four crucial elements:

1. **Assembling** - accomplished by [name-as](name-as), a maintainable assembler that outputs ELF object files
2. **Linking** - accomplished by [name-ld](name-ln), a sophisticated linker which can manage many modules at once
3. **Emulation** - accomplished by [name-emu](name-emu), a performant CPU emulator
4. **Development** - accomplished by
  - [name-ext](name-ext), a VSCode integration for assembly development complete with a [DAP](https://microsoft.github.io/debug-adapter-protocol//) and [IntelliSense](https://learn.microsoft.com/en-us/visualstudio/ide/using-intellisense) for insight into emulated CPU cores
  - [name-fmt](name-fmt) a VSCode extension for canonical assembly formatting and syntax highlighting

## Building From Source

NAME is a vscode extension, which means it is built with typescript. Additionally, NAME uses Rust binaries to function. To ensure you can build from source, confirm you have installed the following software:
 - nodejs
 - rust

### Common Problems Building From Source

In its current state, when building from source, NAME will not function unless `npm run build` has been executed in the `name-ext` directory.

## Test Files

Some test files have been included. You can find them in [test files](test_files/test_files.md).

## Assembly
NAME assembles each module in the user's directory into an ET_REL relocatable object file for the linker to handle, using code found in [elf_utils.rs](name-as/src/elf_utils.rs) and [assembler.rs](name-as/src/nma.rs). Each ET_REL has the same following sections, present in this order:
 - Reserved null section
 - `.text`
 - `.data`
 - `.symtab`
 - `.strtab`
 - `.shstrtab`

The order in which sections appear in linked, multi-module programs is not specified. Single-module programs maintain the same section order once assembled.

The NAME MIPS assembler uses a few key steps:

#### Parsing
Parsing is implmented in [parser.rs](name-as/src/parser.rs). It uses [logos](https://github.com/maciejhirsz/logos) to create a lexer based on several regex patterns, which can be found in [tokens.rs](name-as/src/tokens.rs). It performs one pass through the file, breaking each line into its individual components - see **LineComponent** in [structs.rs](name-const/src/structs.rs) for the data structure and associated enum.

These extracted patterns are handled by the assembler.

#### Assembling
When the parsed line components contain an instruction mnemonic, the assembler first attempts to retrieve the associated [InstructionInformation](name-const/src/structs.rs). If it cannot be found, an error is returned. Once the InstructionInformation is retrieved, the assembler checks the associated operands for the correct argument configuration. If alternate configurations exist, they are also checked for. If no configurations match, an error is returned. Then, the assembler calls the appropriate helper function to actually pack the instruction. These helpers are defined in [assembly_utils.rs](name-as/src/assembly_utils.rs).

If an instruction which expects a branch label returns `Ok(None)`, this means the branch label was given but not yet defined, otherwise known as a forward reference. Take the following assembly code:
```mips
main:
  lui    $t0, 10
  j exit
skip:
  # This label is skipped over
exit:
  lui    $v0, 10
  syscall
```

It's clear that when the assembler encounters the `j exit` instruction, it does not yet know the corresponding address.

NAME tackles this problem using backpatching. The assembler keeps track of any detected forward references (labels referenced with no symbol table entry) and attempts to resolve them once labels are encountered. It does so by saving the byte offset, InstructionInformation, operands, and other associated information for a line once the forward reference is encountered, at which point placeholder bytes `0x00000000` are placed in the `.text` section. Once the correct label is encountered, instructions can be patched by invoking [assemble_instruction](name-as/src/assemble_instruction.rs) on the saved information and arguments, then slicing in the new bytes on the `.text` section. If labels are referenced but never defined in any module, NAME throws an error.

**Note** that when NAME encounters an error on a line, it does not panic. All errors are detected and detailed information is printed for each error in one pass. There are very few fatal errors for NAME.

Once the assembler has assembled the entire file in memory, it is written to disk as an ELF object file. See [elf_utils.rs](name-const/src/elf_utils.rs) for implementation details.

## Linking
The linker ([linker.rs](name-as/src/linker.rs)) reads relocatable object files from disk and constructs them into Elf objects. This choice was made for two intertwined reasons - modularity and educational value. Keeping the linker separate allows for better classroom demonstrations regarding how object files are linked into a single executable, and since ELF is a standard format, there is no magical black box for the sake of the classroom; rather, the linking process is entirely accurate to real-world programs, allowing students to make better global connections.

The convention when linking into an executable is to search the symbol table for the global symbol `main`; if found, that is the entry point, else since `main` is not present, the entry point defaults to the MIPS `.text` start address: `0x4000000`. Conflicting global symbols raise an error, but duplicate local symbols are of course allowed. All linking is performed to the ELF TIS (Tool Interface Standard).

The linker outputs a single file with no extension - an ET_EXEC ELF executable that [name-emu](name-emu/) can interpret.

## Emulation
Thanks for giving me a few development cycles.

The NAME emulator accepts ELF files of type `ET_EXEC`. Emulation is carried out using some structs: the `Processor` struct contains the registers as well as the program counter, and the `Memory` struct contains sections `.text` and `.data`. While I could have kept all these pieces separate, it simplifies some later portions of the approach tremendously. The logic for simulation is present in [simulator.rs](name-emu/src/simulator.rs).

### Fetch
Fetching is a simple access of `.text` from where `$pc`, the program counter, points to. If the address currently in `$pc` is not accessible by the emulator, an error is thrown. This is an intuitive and straightforward approach, so it's included in [simulator.rs](name-emu/src/simulator.rs).

### Decode
Decoding is the first novel approach in the emulation process. First, the instruction passed to the decode function has its opcode extracted. The opcode is put through a lookup table for a function pointer with the signature `Fn(&mut Processor, &mut Memory, u32)`. This function pointer represents the target instruction's implementation. If the opcode is `0`, a separate lookup is performed specific for R-type instructions. This allows instruction addition to be a simple edit of a data structure followed by implementing a microscopic function, improving NAME's extensibility. See [decode.rs](name-emu/src/decode.rs) for implementation details.

Each instruction performs its own unpacking (`unpack_r_type`). This reduces lookup operations.

### Execute
Execution is as simple as invoking the extracted function pointer from the previous step. There exists no separate `execute.rs` since it's a one-liner.
