// SPDX-License-Identifier: GPL-3.0-only

use core::ops::FromResidual;
use std::prelude::*;
use std::uefi::Handle;
use std::uefi::boot::InterfaceType;
use std::uefi::guid::{Guid, NULL_GUID};
use std::uefi::memory::PhysicalAddress;
use std::uefi::status::{Error, Result, Status};

static VBT: &[u8] = include_bytes!(env!("FIRMWARE_OPEN_VBT"));

pub static GOP_POLICY_GUID: Guid = Guid(0xec2e931b,0x3281,0x48a5,[0x81,0x7,0xdf,0x8a,0x8b,0xed,0x3c,0x5d]);
pub const GOP_POLICY_REVISION: u32 = 0x03;

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

extern "win64" fn GetPlatformLidStatus(CurrentLidStatus: *mut LidStatus) -> Status {
    if CurrentLidStatus.is_null() {
        return Status::from_residual(Error::InvalidParameter);
    }

    // TODO: Get real lid status
    unsafe { *CurrentLidStatus = LidStatus::LidOpen };

    Status(0)
}

extern "win64" fn GetVbtData(VbtAddress: *mut PhysicalAddress, VbtSize: *mut u32) -> Status {
    if VbtAddress.is_null() || VbtSize.is_null() {
        return Status::from_residual(Error::InvalidParameter);
    }

    unsafe { *VbtAddress = PhysicalAddress(VBT.as_ptr() as u64) };
    unsafe { *VbtSize = VBT.len() as u32 };

    Status(0)
}

extern "win64" fn GetPlatformDockStatus(_CurrentDockStatus: DockStatus) -> Status {
    Status::from_residual(Error::Unsupported)
}

#[repr(C)]
pub struct GopPolicy {
    pub Revision: u32,
    pub GetPlatformLidStatus: extern "win64" fn (CurrentLidStatus: *mut LidStatus) -> Status,
    pub GetVbtData: extern "win64" fn (VbtAddress: *mut PhysicalAddress, VbtSize: *mut u32) -> Status,
    pub GetPlatformDockStatus: extern "win64" fn (CurrentDockStatus: DockStatus) -> Status,
    pub GopOverrideGuid: Guid,
}

impl GopPolicy {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            Revision: GOP_POLICY_REVISION,
            GetPlatformLidStatus,
            GetVbtData,
            GetPlatformDockStatus,
            GopOverrideGuid: NULL_GUID,
        })
    }

    pub fn install(self: Box<Self>) -> Result<()> {
        let uefi = unsafe { std::system_table_mut() };

        let self_ptr = Box::into_raw(self);
        let mut handle = Handle(0);
        (uefi.BootServices.InstallProtocolInterface)(&mut handle, &GOP_POLICY_GUID, InterfaceType::Native, self_ptr as usize)?;

        //let _ = (uefi.BootServices.UninstallProtocolInterface)(handle, &GOP_POLICY_GUID, self_ptr as usize);

        Ok(())
    }
}
