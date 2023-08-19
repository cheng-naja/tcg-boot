use super::{get_protocol, Guid, Path, SimpleFileSystem, Status, SystemTable};
use core::mem::transmute;

/// Represents an `EFI_HANDLE` for a device.
pub struct Device(());

impl Device {
    pub fn locate<'a>(proto: &Guid, path: &'a Path) -> Result<(&'static Self, &'a Path), Status> {
        SystemTable::current()
            .boot_services()
            .locate_device_path(proto, path)
    }

    pub fn path(&self) -> Option<&Path> {
        static ID: Guid = Guid::new(
            0x09576e91,
            0x6d3f,
            0x11d2,
            [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
        );

        unsafe { get_protocol(transmute(self), &ID).map(|v| transmute(v)) }
    }

    pub fn file_system(&self) -> Option<&SimpleFileSystem> {
        unsafe { get_protocol(transmute(self), &SimpleFileSystem::ID).map(|v| transmute(v)) }
    }
}
