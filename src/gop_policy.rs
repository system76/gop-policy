// SPDX-License-Identifier: GPL-3.0-only

#![allow(unused)]

use std::prelude::*;
use std::uefi::boot::LocateSearchType;
use std::uefi::firmware_volume::{FirmwareVolume2, SectionType};
use std::uefi::memory::PhysicalAddress;

// From edk2
const VBT_FILE_GUID: Guid = guid!("56752da9-de6b-4895-8819-1945b6b76c22");

// Protocol definition

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct LidStatus(u32);

impl LidStatus {
    pub const CLOSED: Self = Self(0);
    pub const OPEN: Self = Self(1);
    pub const MAX: Self = Self(2);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct DockStatus(u32);

impl DockStatus {
    pub const DOCKED: Self = Self(0);
    pub const UNDOCKED: Self = Self(1);
    pub const MAX: Self = Self(2);
}

#[rustfmt::skip]
#[derive(Debug)]
#[repr(C)]
pub struct GopPolicy {
    pub revision: u32,
    pub get_platform_lid_status: extern "efiapi" fn (current_lid_status: *mut LidStatus) -> Status,
    pub get_vbt_data: extern "efiapi" fn (vbt_address: *mut PhysicalAddress, vbt_size: *mut u32) -> Status,
    pub get_platform_dock_status: extern "efiapi" fn (current_dock_status: DockStatus) -> Status,
    pub gop_override_guid: Guid,
}

impl GopPolicy {
    pub const GUID: Guid = guid!("ec2e931b-3281-48a5-8107-df8a8bed3c5d");
    pub const REVISION_01: u32 = 0x01;
    pub const REVISION_03: u32 = 0x03;
}

// Protocol implementation

extern "efiapi" fn get_platform_lid_status(current_lid_status: *mut LidStatus) -> Status {
    if current_lid_status.is_null() {
        return Status::INVALID_PARAMETER;
    }

    // TODO: Get real lid status
    unsafe { *current_lid_status = LidStatus::OPEN };

    Status::SUCCESS
}

extern "efiapi" fn get_vbt_data(vbt_address: *mut PhysicalAddress, vbt_size: *mut u32) -> Status {
    if vbt_address.is_null() || vbt_size.is_null() {
        return Status::INVALID_PARAMETER;
    }

    let mut status = Status::SUCCESS;
    let st = unsafe { std::system_table_mut() };

    // Multiple FVs in the FD, so check all.
    let mut count = 0;
    let mut hbuffer = core::ptr::null_mut();
    status = (st.BootServices.LocateHandleBuffer)(
        LocateSearchType::ByProtocol,
        &FirmwareVolume2::GUID,
        core::ptr::null(),
        &mut count,
        &mut hbuffer,
    );
    if status.is_error() {
        return Status::NOT_FOUND;
    }

    let handles = unsafe { core::slice::from_raw_parts(hbuffer, count) };
    for handle in handles {
        let mut interface = 0;
        status = (st.BootServices.HandleProtocol)(*handle, &FirmwareVolume2::GUID, &mut interface);

        let mut vbt_ptr = core::ptr::null_mut();
        let mut vbt_len = 0;
        let mut auth_status = 0;

        let fv: &FirmwareVolume2 = unsafe { &*(interface as *const FirmwareVolume2) };
        status = (fv.ReadSection)(
            fv,
            &VBT_FILE_GUID,
            SectionType::RAW,
            0,
            &mut vbt_ptr,
            &mut vbt_len,
            &mut auth_status,
        );

        if status.is_success() {
            unsafe { *vbt_address = PhysicalAddress(vbt_ptr as u64) };
            unsafe { *vbt_size = vbt_len as u32 };
            break;
        }
    }

    (st.BootServices.FreePool)(hbuffer as usize);

    status
}

extern "efiapi" fn get_platform_dock_status(_current_dock_status: DockStatus) -> Status {
    Status::UNSUPPORTED
}

pub static GOP_POLICY: GopPolicy = GopPolicy {
    revision: GopPolicy::REVISION_03,
    get_platform_lid_status,
    get_vbt_data,
    get_platform_dock_status,
    gop_override_guid: Guid::NULL,
};
