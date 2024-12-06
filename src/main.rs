// SPDX-License-Identifier: GPL-3.0-only

#![no_std]
#![no_main]

#[macro_use]
extern crate uefi_std as std;

use std::prelude::*;

mod gop_policy;

#[no_mangle]
pub extern "C" fn main() -> Status {
    let gop_policy = gop_policy::GopPolicy::new();
    if let Err(err) = gop_policy.install() {
        println!("GopPolicy error: {:?}", err);
        err
    } else {
        Status::SUCCESS
    }
}
