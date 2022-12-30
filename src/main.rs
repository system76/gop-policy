// SPDX-License-Identifier: GPL-3.0-only

#![no_std]
#![no_main]
#![feature(prelude_import)]
#![feature(try_trait_v2)]
#![allow(non_snake_case)]

#[macro_use]
extern crate uefi_std as std;

#[allow(unused_imports)]
#[prelude_import]
use std::prelude::*;

use core::ops::FromResidual;
use std::uefi::status::Status;

mod gop_policy;

#[no_mangle]
pub extern "C" fn main() -> Status {
    let gop_policy = gop_policy::GopPolicy::new();
    if let Err(err) = gop_policy.install() {
        println!("GopPolicy error: {:?}", err);
        Status::from_residual(err)
    } else {
        Status(0)
    }
}
