// SPDX-License-Identifier: GPL-3.0-only

#![no_std]
#![no_main]

extern crate uefi_std as std;

mod gop_policy;

use gop_policy::{GopPolicy, GOP_POLICY};
use std::prelude::*;
use std::uefi::boot::InterfaceType;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> Status {
    let uefi = unsafe { std::system_table_mut() };
    let mut handle = Handle(0);

    (uefi.BootServices.InstallProtocolInterface)(
        &mut handle,
        &GopPolicy::GUID,
        InterfaceType::Native,
        core::ptr::addr_of!(GOP_POLICY) as usize,
    )
}
