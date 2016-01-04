use std::slice;
use std::mem;
pub mod x86_64;
pub mod cpuid;

use byteorder::{LittleEndian, WriteBytesExt};

#[derive(PartialEq, Debug)]
pub enum Vendor {
	AuthenticAMD,
	GenuineIntel,
	KVM,
	VMware,
	XenHVM,
	Unknown
}

impl Vendor {
	fn detect(cpuid: &cpuid::CpuId) -> Vendor {
		let mut buf: Vec<u8> = vec![];
		// todo map or something, also remove unwrap
		buf.write_u32::<LittleEndian>(cpuid.vendor_info.rbx).unwrap();
		buf.write_u32::<LittleEndian>(cpuid.vendor_info.rdx).unwrap();
		buf.write_u32::<LittleEndian>(cpuid.vendor_info.rcx).unwrap();
		let vendor_string = String::from_utf8(buf).unwrap();

		match vendor_string.as_ref(){
			"AuthenticAMD" => Vendor::AuthenticAMD,
			"GenuineIntel"=> Vendor::GenuineIntel,
			"KVMKVMKVM" => Vendor::KVM,
			"VMwareVMware" => Vendor::VMware,
			"XenVMMXenVMM" => Vendor::XenHVM,
			_ => Vendor::Unknown
		}
	}
}

#[derive(Debug)]
pub struct OSFeatures {
	avx: bool,
	avx512: bool,
}

impl OSFeatures {
	pub fn detect(cpuid: &cpuid::CpuId) -> OSFeatures {
		OSFeatures { avx: OSFeatures::detect_avx(&cpuid), avx512: OSFeatures::detect_avx512(&cpuid) }
	}

	// todo (dodgy stuff with xgetbv required)
	fn detect_avx(cpuid: &cpuid::CpuId) -> bool {
		true
	}

	// same as above and doesn't matter anyway since avx512 is not availiable yet in consumer hardware
	fn detect_avx512(cpuid: &cpuid::CpuId) -> bool {
		false
	}
}

#[derive(Debug)]
struct HWFeatures {
	mmx: bool,          
	x64: bool,          
	abm: bool, // Advanced Bit Manipulation       
	rdrand: bool,       
	bmi1: bool,         
	bmi2: bool,         
	adx: bool,          
	mpx: bool,         
	prefetchwt1: bool, 
}


impl HWFeatures {
	fn detect(cpuid: &cpuid::CpuId) -> HWFeatures {
		HWFeatures {
			mmx: cpuid.feature_info.rdx             & (1 << 23) != 0,
			x64: cpuid.ext_processor_info.rdx       & (1 << 29) != 0,
			abm: cpuid.ext_processor_info.rcx       & (1 << 5)  != 0,
			rdrand: cpuid.feature_info.rcx          & (1 << 30) != 0,
			bmi1: cpuid.ext_feature_info.rbx        & (1 << 3)  != 0,
			bmi2: cpuid.ext_feature_info.rbx        & (1 << 8)  != 0,
			adx: cpuid.ext_feature_info.rbx         & (1 << 19) != 0,
			mpx: cpuid.ext_feature_info.rbx         & (1 << 14) != 0,
			prefetchwt1: cpuid.ext_feature_info.rcx & (1 << 0)  != 0,
		}
	}
}

#[derive(Debug)]
struct SIMDFeatures {
	// 128-bit
	sse: bool,         
	sse2: bool,        
	sse3 : bool,       
	ssse3: bool,       
	sse4a: bool,       
	sse41: bool,      
	sse42: bool,      
	aes: bool,      
	sha: bool, 

	// 256-bit
	avx: bool,         
	xop: bool,         
	fma3: bool,        
	fma4: bool,        
	avx2: bool,     

	// 512-bit
	avx512_f: bool,    //  AVX512 Foundation
	avx512_cd: bool,   //  AVX512 Conflict Detection
	avx512_pf: bool,   //  AVX512 Prefetch
	avx512_er: bool,   //  AVX512 Exponential + Reciprocal
	avx512_vl: bool,   //  AVX512 Vector Length Extensions
	avx512_bw: bool,   //  AVX512 Byte + Word
	avx512_dq: bool,   //  AVX512 Doubleword + Quadword
	avx512_ifma: bool, //  AVX512 Integer 52-bit Fused Multiply-Add
	avx512_vbmi: bool, //  AVX512 Vector Byte Manipulation Instructions 
}

impl SIMDFeatures {
	fn detect(cpuid: &cpuid::CpuId) -> SIMDFeatures {
		SIMDFeatures {
			// This is got boring quickly
			sse: cpuid.feature_info.rdx             & (1 << 23) != 0,
			sse2: cpuid.feature_info.rdx            & (1 << 26) != 0,
			sse3: cpuid.feature_info.rcx            & (1 <<  0) != 0,
			ssse3: cpuid.feature_info.rcx           & (1 <<  9) != 0,
			sse4a: cpuid.ext_processor_info.rcx     & (1 <<  6) != 0,
			sse41: cpuid.ext_processor_info.rcx     & (1 << 19) != 0,
			sse42: cpuid.ext_processor_info.rcx     & (1 << 20) != 0,
			aes: cpuid.ext_processor_info.rcx       & (1 << 25) != 0,
			sha: cpuid.ext_feature_info.rbx         & (1 << 29) != 0,

			avx: cpuid.feature_info.rcx 		    & (1 << 28) != 0,
			xop: cpuid.ext_processor_info.rcx       & (1 << 11) != 0,
			fma3: cpuid.feature_info.rcx            & (1 << 12) != 0,
			fma4: cpuid.ext_processor_info.rcx      & (1 << 16) != 0,
			avx2: cpuid.ext_feature_info.rbx 	    & (1 <<  5) != 0,

			avx512_f: cpuid.ext_feature_info.rbx    & (1 << 16) != 0,
			avx512_cd: cpuid.ext_feature_info.rbx   & (1 << 28) != 0,
			avx512_pf: cpuid.ext_feature_info.rbx   & (1 << 26) != 0,
			avx512_er: cpuid.ext_feature_info.rbx   & (1 << 27) != 0,
			avx512_vl: cpuid.ext_feature_info.rbx   & (1 << 31) != 0,
			avx512_bw: cpuid.ext_feature_info.rbx   & (1 << 30) != 0,
			avx512_dq: cpuid.ext_feature_info.rbx   & (1 << 17) != 0,
			avx512_ifma: cpuid.ext_feature_info.rbx & (1 << 21) != 0,
			avx512_vbmi: cpuid.ext_feature_info.rcx & (1 <<  1) != 0,
		}
	}

}

#[derive(Debug)]
pub struct ProcessorInfo {
	vendor: Vendor,
	os_features: OSFeatures,
	hw_features: HWFeatures,
	simd_features: SIMDFeatures
}


impl ProcessorInfo {
	fn detect(cpuid: &cpuid::CpuId) -> ProcessorInfo {
		ProcessorInfo {
			vendor: Vendor::detect(cpuid),
			os_features: OSFeatures::detect(cpuid),
			hw_features: HWFeatures::detect(cpuid),
			simd_features: SIMDFeatures::detect(cpuid)
		}
	}
}

#[test]
fn test() {
	let info = ProcessorInfo::detect(&cpuid::CpuId::detect());
}