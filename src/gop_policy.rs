// SPDX-License-Identifier: GPL-3.0-only

#![allow(non_snake_case)]
#![allow(unused)]

use std::prelude::*;
use std::uefi::memory::PhysicalAddress;

static VBT: &[u8] = include_bytes!(env!("FIRMWARE_OPEN_VBT"));

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

    unsafe { *VbtAddress = PhysicalAddress(VBT.as_ptr() as u64) };
    unsafe { *VbtSize = VBT.len() as u32 };

    Status::SUCCESS
}

extern "efiapi" fn GetPlatformDockStatus(_CurrentDockStatus: DockStatus) -> Status {
    Status::UNSUPPORTED
}

pub static GOP_POLICY: GopPolicy = GopPolicy {
    Revision: GopPolicy::REVISION_03,
    GetPlatformLidStatus,
    GetVbtData,
    GetPlatformDockStatus,
    GopOverrideGuid: Guid::NULL,
};
