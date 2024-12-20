// SPDX-License-Identifier: GPL-3.0-only

#![allow(non_snake_case)]
#![allow(unused)]

use std::prelude::*;
use std::uefi::boot::{InterfaceType, LocateSearchType};
use std::uefi::firmware::{FirmwareVolume2, SectionType};
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
    pub Revision: u32,
    pub GetPlatformLidStatus: extern "efiapi" fn (CurrentLidStatus: *mut LidStatus) -> Status,
    pub GetVbtData: extern "efiapi" fn (VbtAddress: *mut PhysicalAddress, VbtSize: *mut u32) -> Status,
    pub GetPlatformDockStatus: extern "efiapi" fn (CurrentDockStatus: DockStatus) -> Status,
    pub GopOverrideGuid: Guid,
}

impl GopPolicy {
    pub const GUID: Guid = guid!("ec2e931b-3281-48a5-8107-df8a8bed3c5d");
    pub const REVISION_01: u32 = 0x01;
    pub const REVISION_03: u32 = 0x03;
}

// Protocol implementation

extern "efiapi" fn GetPlatformLidStatus(CurrentLidStatus: *mut LidStatus) -> Status {
    if CurrentLidStatus.is_null() {
        return Status::INVALID_PARAMETER;
    }

    // TODO: Get real lid status
    unsafe { *CurrentLidStatus = LidStatus::OPEN };

    Status::SUCCESS
}

extern "efiapi" fn GetVbtData(VbtAddress: *mut PhysicalAddress, VbtSize: *mut u32) -> Status {
    if VbtAddress.is_null() || VbtSize.is_null() {
        return Status::INVALID_PARAMETER;
    }

    let mut status = Status::SUCCESS;
    let uefi = unsafe { std::system_table_mut() };

    let mut count = 0;
    let mut hbuffer = core::ptr::null_mut();
    status = (uefi.BootServices.LocateHandleBuffer)(
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
        let _ = (uefi.BootServices.HandleProtocol)(
            *handle,
            &FirmwareVolume2::GUID,
            &mut interface,
        );

        let mut vbt_ptr = core::ptr::null_mut();
        let mut vbt_size = 0;
        let mut auth_status = 0;

        let fv: &FirmwareVolume2 = unsafe { &*(interface as *const FirmwareVolume2) };
        let status = (fv.ReadSection)(
            fv,
            &VBT_FILE_GUID,
            SectionType::RAW,
            0,
            &mut vbt_ptr,
            &mut vbt_size,
            &mut auth_status,
        );

        if status.is_success() {
            unsafe { *VbtAddress = PhysicalAddress(vbt_ptr as u64) };
            unsafe { *VbtSize = vbt_size as u32 };
            break;
        }
    }

    (uefi.BootServices.FreePool)(hbuffer as usize);

    status
}

extern "efiapi" fn GetPlatformDockStatus(_CurrentDockStatus: DockStatus) -> Status {
    Status::UNSUPPORTED
}

impl GopPolicy {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            Revision: Self::REVISION_03,
            GetPlatformLidStatus,
            GetVbtData,
            GetPlatformDockStatus,
            GopOverrideGuid: Guid::NULL,
        })
    }

    pub fn install(self: Box<Self>) -> Result<()> {
        let uefi = unsafe { std::system_table_mut() };

        // NOTE: This leaks the allocation so that it does not get dropped.
        let self_ptr = Box::into_raw(self);
        let mut handle = Handle(0);
        Result::from((uefi.BootServices.InstallProtocolInterface)(
            &mut handle,
            &Self::GUID,
            InterfaceType::Native,
            self_ptr as usize,
        ))?;

        Ok(())
    }
}
