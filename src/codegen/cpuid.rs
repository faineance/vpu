
use std::str;
use std::slice;
use std::mem;
use byteorder::{LittleEndian, WriteBytesExt};


const VENDOR_INFO: u32 = 0x0;
const FEATURE_INFO: u32 = 0x1;
const EXT_FEATURE_INFO: u32 = 0x7;
const EXT_PROCESSOR_INFO: u32 = 0x80000001;

#[cfg(target_arch = "x86_64")]
pub fn cpuid(func: u32) -> CpuIdInfo {
    let (rax, rbx, rcx, rdx);
    unsafe {
        asm!("cpuid"
            : // output operands
            "={rax}"(rax),
            "={rbx}"(rbx),
            "={rcx}"(rcx),
            "={rdx}"(rdx)
            : // input operands
            "{rax}"(func),
            "{rcx}"(0 as u32)
            : // clobbers
            : // options
            );
    }
    CpuIdInfo {
        rax: rax, 
        rbx: rbx, 
        rcx: rcx, 
        rdx: rdx
    }
}

// Rename to something better
pub struct CpuIdInfo {
    pub rax: u32,
    pub rbx: u32,
    pub rcx: u32,
    pub rdx: u32,
}

pub struct CpuId {
    pub highest_func_param: u32,
    pub vendor_info: CpuIdInfo, 
    pub feature_info: CpuIdInfo,
    pub ext_feature_info: CpuIdInfo,
    pub ext_processor_info: CpuIdInfo
}

impl CpuId {
    pub fn detect() -> CpuId {
        CpuId {
            highest_func_param: cpuid(VENDOR_INFO).rax,
            vendor_info: cpuid(VENDOR_INFO), 
            feature_info: cpuid(FEATURE_INFO),
            ext_feature_info: cpuid(EXT_FEATURE_INFO),
            ext_processor_info: cpuid(EXT_PROCESSOR_INFO)
        }
    }
}

#[test]
fn test_usage() {
    let v = cpuid(VENDOR_INFO);

    let mut wtr: Vec<u8> = vec![];
    wtr.write_u32::<LittleEndian>(v.rbx).unwrap();
    wtr.write_u32::<LittleEndian>(v.rdx).unwrap();
    wtr.write_u32::<LittleEndian>(v.rcx).unwrap();
    let string = String::from_utf8(wtr).unwrap();
    assert!(string == "AuthenticAMD" || string == "GenuineIntel")
}



