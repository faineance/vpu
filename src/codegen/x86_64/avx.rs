
/*CPUs with AVX

    Intel
        Sandy Bridge processor, Q1 2011
        Sandy Bridge E processor, Q4 2011
        Ivy Bridge processor, Q1 2012
        Ivy Bridge E processor, Q3 2013
        Haswell processor, Q2 2013
        Haswell E processor, Q3 2014
        Broadwell processor, Q4 2014
        Broadwell E processor, expected in 2016
        Skylake processor, Q3 2015
        Kaby Lake processor, expected in 2016
        Cannonlake processor, expected in 2017
    AMD:
        Bulldozer-based processor, Q4 2011
        Piledriver-based processor, Q4 2012
        Steamroller-based processor, Q1 2014
        Excavator-based processor, expected in 2015
        Jaguar-based processor
        Puma-based processor
*/

pub enum AVXInstr {
	VBROADCASTSS, VBROADCASTSD, VBROADCASTF128, // Copy a 32-bit, 64-bit or 128-bit memory operand to all elements of a XMM or YMM vector register.
	VINSERTF128, 	// Replaces either the lower half or the upper half of a 256-bit YMM register with the value of a 128-bit source operand. The other half of the destination is unchanged.
	VEXTRACTF128, 	// Extracts either the lower half or the upper half of a 256-bit YMM register and copies the value to a 128-bit destination operand.
	VMASKMOVPS, VMASKMOVPD, // Conditionally reads any number of elements from a SIMD vector memory operand into a destination register, leaving the remaining vector elements unread and setting the corresponding elements in the destination register to zero. Alternatively, conditionally writes any number of elements from a SIMD vector register operand to a vector memory operand, leaving the remaining elements of the memory operand unchanged. On the AMD Jaguar processor architecture, this instruction with a memory source operand takes more than 300 clock cycles when the mask is zero, in which case the instruction should do nothing. This appears to be a design flaw.[6]
	VPERMILPS, VPERMILPD, // Permute In-Lane. Shuffle the 32-bit or 64-bit vector elements of one input operand. These are in-line 256-bit instructions, meaning that they operate on all 256 bits with two separate 128-bit shuffles, so they can not shuffle across the 128-bit lanes.[7]
	VPERM2F128, // Shuffle the four 128-bit vector elements of two 256-bit source operands into a 256-bit destination operand, with an immediate constant as selector.
	VZEROALL,// Set all YMM registers to zero and tag them as unused. Used when switching between 128-bit use and 256-bit use.
	VZEROUPPER,// Set the upper half of all YMM registers to zero. Used when switching between 128-bit use and 256-bit use.
}
