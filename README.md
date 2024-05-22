# NAME

![logo](logo/logo.png)

NAME ("Not Another MIPS Emulator") is a modular, language-agnostic assembly code emulation pipeline designed for educational use.

While this implementation focuses on MIPS, a fork of the project could easilty produce an implementation for any other asm.
Details on how that's all meant to be done is documented within [name-as](name-as), [name-emu](name-emu), and later in this README.

```rust
// TODO: Actually document that :)
```

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

#### Common Problems

In its current state, when building from source, NAME will not function unless `npm run build` has been executed in the `name-ext` directory.

## Test Files

Some test files have been included. You can find them in [test files](test_files.md).