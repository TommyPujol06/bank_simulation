#![no_std]
#![cfg_attr(
    any(target_arch = "nvptx", target_arch = "nvptx64"),
    feature(abi_ptx, stdsimd)
)]

#[cfg(any(target_arch = "nvptx", target_arch = "nvptx64"))]
mod kernel;
