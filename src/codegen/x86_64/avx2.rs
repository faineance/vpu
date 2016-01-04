
/*CPUs with AVX2

    Intel
        Haswell processor, Q2 2013
        Haswell E processor, Q3 2014
        Broadwell processor, Q4 2014
        Broadwell E processor, expected in 2016
        Skylake processor, Q3 2015
        Kaby Lake processor, expected in 2016
        Cannonlake processor, expected in 2017
    AMD
        Excavator-based processor, expected in 2015
        Zen-based processor, expected in 2016
*/
pub enum AVX2Instr {
    VBROADCASTSS, VBROADCASTSD, // Copy a 32-bit or 64-bit register operand to all elements of a XMM or YMM vector register. These are register versions of the same instructions in AVX1. There is no 128-bit version however, but the same effect can be simply achieved using VINSERTF128.
    VPBROADCASTB, VPBROADCASTW, VPBROADCASTD, VPBROADCASTQ, // Copy an 8, 16, 32 or 64-bit integer register or memory operand to all elements of a XMM or YMM vector register.
    VBROADCASTI128, // Copy a 128-bit memory operand to all elements of a YMM vector register.
    VINSERTI128, // Replaces either the lower half or the upper half of a 256-bit YMM register with the value of a 128-bit source operand. The other half of the destination is unchanged.
    VEXTRACTI128, // Extracts either the lower half or the upper half of a 256-bit YMM register and copies the value to a 128-bit destination operand.
    VGATHERDPD, VGATHERQPD, VGATHERDPS, VGATHERQPS, // Gathers single or double precision floating point values using either 32 or 64-bit indices and scale.
    VPGATHERDD, VPGATHERDQ, VPGATHERQD, VPGATHERQQ, // Gathers 32 or 64-bit integer values using either 32 or 64-bit indices and scale.
    VPMASKMOVD, VPMASKMOVQ, // Conditionally reads any number of elements from a SIMD vector memory operand into a destination register, leaving the remaining vector elements unread and setting the corresponding elements in the destination register to zero. Alternatively, conditionally writes any number of elements from a SIMD vector register operand to a vector memory operand, leaving the remaining elements of the memory operand unchanged.
    VPERMPS, VPERMD, // Shuffle the eight 32-bit vector elements of one 256-bit source operand into a 256-bit destination operand, with a register or memory operand as selector.
    VPERMPD, VPERMQ, // Shuffle the four 64-bit vector elements of one 256-bit source operand into a 256-bit destination operand, with a register or memory operand as selector.
    VPERM2I128, // Shuffle the four 128-bit vector elements of two 256-bit source operands into a 256-bit destination operand, with an immediate constant as selector.
    VPBLENDD, // Doubleword immediate version of the PBLEND instructions from SSE4.
    VPSLLVD, VPSLLVQ, // Shift left logical. Allows variable shifts where each element is shifted according to the packed input.
    VPSRLVD, VPSRLVQ, // Shift right logical. Same as above
    VPSRAVD,  // Shift right arithmetically. Same as above
}
