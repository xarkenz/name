// This file is meant for coprocessor 0's exception handler.

// The implementation here was derived entirely from this document: https://s3-eu-west-1.amazonaws.com/downloads-mips/documents/MD00090-2B-MIPS32PRA-AFP-06.02.pdf

// This enum contains all the exceptions we could generate.
#[derive(Debug)]
pub enum ExceptionType {
    Interrupt,
    // TlbMod,
    // TlbLoad,
    // TlbStore,
    AddressExceptionLoad,
    AddressExceptionStore,
    // BusFetch,
    // BusLoadStore,
    Syscall,
    Breakpoint,
    ReservedInstruction,
    // CoprocessorUnusable,
    ArithmeticOverflow,
    Trap,
    // MsaFloatingPoint,
    FloatingPoint,
    // Implementation Dependent,
    // Implementation Dependent,
    // TlbReadInhibit,
    // TlbExecutionInhibit,
    // MsaDisabled,
    // Reserved,
    // Watch,
    // MCheck,
    // Thread,
    // DspDisabled,
    // VirtualizedGuestException,
    // Reserved,
    // Reserved,
    // CacheErr,
    // Reserved,
}