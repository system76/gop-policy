// SPDX-License-Identifier: GPL-3.0-only

#![no_std]
#![no_main]

mod gop_policy;

use gop_policy::{GOP_POLICY, GopPolicy};
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    unsafe {
        boot::install_protocol_interface(
            None,
            &GopPolicy::GUID,
            core::ptr::addr_of!(GOP_POLICY).cast(),
        )
        .status()
    }
}
