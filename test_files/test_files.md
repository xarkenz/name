# Test Files

Included in this directory are a couple of test files to get you started with NAME and get familiar with the workflow you'll want to use with it.

## Included tests

- ~~Hello, World!~~ *(Not yet included)*
    - Just a simple test of very basic instructions.
- ~~Fibonacci sequence (fib.asm)~~ *(Not yet included)*
    - A more sophisticated test.
- Palindrome checker (palindromes.asm)
    - A test for assembly across multiple files.
- Instruction demonstration (mips_test.asm)
    - A demo of various instructions present in NAME's simulation.

## Getting started

Simply open your .asm file in vscode with NAME installed, and choose either "Run Without Debugging" ~~or "Run and Debug"~~ *(Not yet implemented)*. You should have everything you need to play around and see how MIPS works!

## Recommended workflow

Unlike other MIPS emulators, NAME chooses to leave behind the assembled executable file after assembly, so anyone could take a peek inside if they feel so inclined (hex editor recommended). Because of this, it's recommended that each project have its own folder (good practice anyways!) so that the *.o files do not get mixed up.