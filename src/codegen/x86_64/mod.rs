pub mod avx;
pub mod avx2;
pub mod avx512;

pub mod sse;
pub mod sse2;
pub mod sse3;
pub mod ssse3;
pub mod sse4a;
pub mod sse41;
pub mod sse42;

pub mod emit;

pub enum Flags {
    // Status flags
    CF, // Carry
    PF, // Parity
    AF, // Auxiliary Carry Flag
    ZF, // Zero flag
    SF, // Sign flag
    OF, // Overflow flag

    DF, // Direction flag

    //  System Flags and IOPL Field
    TF, // Trap flag
    IF, // Interrupt enable flag
    IOPL, // I/O privilege level field
    NT, // Nested task flag
    RF, // Resume flag
    VM, // Virtual-8086 mode flag
    AC, // Alignment check (or access control) flag
    VIF, // Virtual interrupt flag
    VIP, // Virtual interrupt pending flag
    ID, // Identification flag
}

// massive enum incoming (todo: break apart)
pub enum Register {
    RAX, EAX,  AX, AH, AL, // Accumulator
    RBX, EBX,  BX, BH, BL, // Base index (for use with arrays)
    RCX, ECX,  CX, CH, CL, // Counter (for use with loops and strings)
    RDX, EDX,  DX, DH, DL, // Extend the precision of the accumulator (e.g. combine 32-bit EAX and EDX for 64-bit integer operations in 32-bit code)


    RSI, ESI,  SI,   SIL, // Source index for string operations.
    RDI, EDI,  DI,   DIL, // Destination index for string operations.
    RSP, ESP,  SP,   SPL, // Stack pointer for top address of the stack.
    RBP, EBP,  BP,   BPL, // Stack base pointer for holding the address of the current stack frame.

    R8,  R8D,  R8W,  R8B,
    R9,  R9D,  R9W,  R9B,
    R10, R10D, R10W, R10B,
    R11, R11D, R11W, R11B,
    R12, R12D, R12W, R12B,
    R13, R13D, R13W, R13B,
    R14, R14D, R14W, R14B,
    R15, R15D, R15W, R15B,

    // Segment registers 
    CS, // Code
    DS, // Data
    SS, // Stack
    ES, // Extra data
    FS, // Extra data #2
    GS, // Extra data #3

    // FPU registers
    ST0, 
    ST1, 
    ST2, 
    ST3, 
    ST4, 
    ST5, 
    ST6, 
    ST7,

    // MMX registers 
    MM0, 
    MM1, 
    MM2, 
    MM3, 
    MM4, 
    MM5, 
    MM6, 
    MM7,

    // 128-bit SSE registers 
    XMM0, 
    XMM1, 
    XMM2, 
    XMM3, 
    XMM4, 
    XMM5, 
    XMM6, 
    XMM7,
    XMM8, 
    XMM9, 
    XMM10, 
    XMM11, 
    XMM12, 
    XMM13, 
    XMM14, 
    XMM15,

    // 256-bit SSE registers 
    YMM0, 
    YMM1, 
    YMM2, 
    YMM3, 
    YMM4, 
    YMM5, 
    YMM6, 
    YMM7,
    YMM8, 
    YMM9, 
    YMM10, 
    YMM11, 
    YMM12, 
    YMM13, 
    YMM14, 
    YMM15,


    RIP, EIP, IP, //  Instruction pointer
    RFLAGS, EFLAGS, FLAGS, // Flags register

    // Control registers
    CR0, CR2, CR3, CR4, CR8,
    
    // Debug registers
    DR0, DR1, DR2, DR3, DR6, DR7,
}

impl Register {
    pub fn volatile(&self) -> bool {
        match *self {
            Register::RAX | Register::RCX | Register::RDX | 
            Register::R8  | Register::R9  | Register::R10 | 
            Register::R11 => true,
            _ => false,
        }
    }
    // pub fn len(&self) -> RetType {
        //     // add code here
        // }
    // }
}