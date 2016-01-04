
// Added with Pentium III
pub enum SSEInstr {
    // ------------Floating point instructions------------
    //Memory-to-register/register-to-memory/register-to-register data movement
    //Scalar
    MOVSS,
    //Packed
    MOVAPS, MOVUPS, MOVLPS, MOVHPS, MOVLHPS, MOVHLPS, MOVMSKPS,

    // Arithmetic
    //Scalar
    ADDSS, SUBSS, MULSS, DIVSS, RCPSS, SQRTSS, MAXSS, MINSS, RSQRTSS,
    //Packed
    ADDPS, SUBPS, MULPS, DIVPS, RCPPS, SQRTPS, MAXPS, MINPS, RSQRTPS,
    // Compare
    //Scalar
    CMPSS, COMISS, UCOMISS,
    //Packed
    CMPPS,

    // Data shuffle and unpacking
    //Packed
    SHUFPS, UNPCKHPS, UNPCKLPS,

    // Data-type conversion
    //Scalar
    CVTSI2SS, CVTSS2SI, CVTTSS2SI,
    //Packed
    CVTPI2PS, CVTPS2PI, CVTTPS2PI,

    // Bitwise logical operations
    //Packed
    ANDPS, ORPS, XORPS, ANDNPS,

    //------------Integer instructions------------

    // Arithmetic
    PMULHUW, PSADBW, PAVGB, PAVGW, PMAXUB, PMINUB, PMAXSW, PMINSW,

    // Data movement
    PEXTRW, PINSRW,

    // Other
    PMOVMSKB, PSHUFW,

    // Other instructions
    // MXCSR management
    LDMXCSR, STMXCSR,

    // Cache and Memory management
    MOVNTQ, MOVNTPS, MASKMOVQ, PREFETCH0, PREFETCH1, PREFETCH2, PREFETCHNTA, SFENCE,
}

