#![no_std]
#![allow(non_camel_case_types)]
#![cfg_attr(test, feature(test))]

#[cfg(test)]
extern crate test;

/*
#[repr(u8)]
enum c_void {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}
*/

#[cfg(any(target_arch = "x86",
          target_arch = "arm",
          target_arch = "mips",
          target_arch = "mipsel",
          target_arch = "powerpc",
          target_arch = "le32"))]
mod libc {
    pub type c_char = i8;
    pub type c_int = i32;
    pub type size_t = u32;
    pub type wchar_t = i32;
}

#[cfg(any(target_arch = "x86_64",
          target_arch = "aarch64"))]
mod libc {
    #[cfg(not(target_arch = "aarch64"))]
    pub type c_char = i8;
    #[cfg(target_arch = "aarch64")]
    pub type c_char = u8;
    pub type c_int = i32;
    pub type size_t = u64;
    #[cfg(not(target_arch = "aarch64"))]
    pub type wchar_t = i32;
    #[cfg(target_arch = "aarch64")]
    pub type wchar_t = u32;
}

pub mod ctype;
pub mod string;
