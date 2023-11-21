// SPDX-License-Identifier: GPL-3.0-only

#![no_std]
#![no_main]
#![allow(non_snake_case)]

// XXX: ??? `st` *is* in an `unsafe` block
#![allow(clippy::not_unsafe_ptr_arg_deref)]

extern crate alloc;

mod gop_policy;

use alloc::boxed::Box;
use r_efi::efi;

#[export_name = "efi_main"]
pub extern "efiapi" fn main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let mut handle: efi::Handle = core::ptr::null_mut();
    let mut guid = gop_policy::GOP_POLICY_GUID;

    let policy = gop_policy::GopPolicy::new();
    let ptr = Box::into_raw(policy) as *mut core::ffi::c_void;

    unsafe {
        ((*(*st).boot_services).install_protocol_interface)(
            &mut handle,
            &mut guid,
            efi::NATIVE_INTERFACE,
            ptr,
        )
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static GLOBAL_ALLOCATOR: r_efi_alloc::global::Bridge = r_efi_alloc::global::Bridge::new();
