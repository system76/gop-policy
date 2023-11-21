// SPDX-License-Identifier: GPL-3.0-only

use alloc::boxed::Box;
use r_efi::efi;

pub const GOP_POLICY_GUID: efi::Guid = efi::Guid::from_fields(
    0xec2e931b,
    0x3281,
    0x48a5,
    0x81,
    0x7,
    &[0xdf, 0x8a, 0x8b, 0xed, 0x3c, 0x5d],
);
pub const GOP_POLICY_REVISION: u32 = 0x03;

static VBT: &[u8] = include_bytes!(env!("FIRMWARE_OPEN_VBT"));

#[allow(unused)]
#[repr(C)]
pub enum LidStatus {
    LidClosed,
    LidOpen,
    LidStatusMax,
}

#[allow(unused)]
#[repr(C)]
pub enum DockStatus {
    Docked,
    UnDocked,
    DockStatusMax,
}

extern "efiapi" fn GetPlatformLidStatus(CurrentLidStatus: *mut LidStatus) -> efi::Status {
    if CurrentLidStatus.is_null() {
        return efi::Status::INVALID_PARAMETER;
    }

    // TODO: Get real lid status
    unsafe { *CurrentLidStatus = LidStatus::LidOpen };

    efi::Status::SUCCESS
}

extern "efiapi" fn GetVbtData(
    VbtAddress: *mut efi::PhysicalAddress,
    VbtSize: *mut u32,
) -> efi::Status {
    if VbtAddress.is_null() || VbtSize.is_null() {
        return efi::Status::INVALID_PARAMETER;
    }

    unsafe { *VbtAddress = VBT.as_ptr() as efi::PhysicalAddress };
    unsafe { *VbtSize = VBT.len() as u32 };

    efi::Status::SUCCESS
}

extern "efiapi" fn GetPlatformDockStatus(_CurrentDockStatus: DockStatus) -> efi::Status {
    efi::Status::UNSUPPORTED
}

#[repr(C)]
pub struct GopPolicy {
    pub Revision: u32,
    pub GetPlatformLidStatus: extern "efiapi" fn(CurrentLidStatus: *mut LidStatus) -> efi::Status,
    pub GetVbtData: extern "efiapi" fn(VbtAddress: *mut efi::PhysicalAddress, VbtSize: *mut u32) -> efi::Status,
    pub GetPlatformDockStatus: extern "efiapi" fn(CurrentDockStatus: DockStatus) -> efi::Status,
    pub GopOverrideGuid: efi::Guid,
}

impl GopPolicy {
    pub fn new() -> Box<Self> {
        let null_guid = efi::Guid::from_fields(0, 0, 0, 0, 0, &[0, 0, 0, 0, 0, 0]);

        Box::new(Self {
            Revision: GOP_POLICY_REVISION,
            GetPlatformLidStatus,
            GetVbtData,
            GetPlatformDockStatus,
            GopOverrideGuid: null_guid,
        })
    }
}
