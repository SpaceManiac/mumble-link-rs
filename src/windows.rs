extern crate kernel32;
extern crate winapi;

use std::{io, mem};
use LinkedMem;

pub type UiTick = winapi::DWORD;

pub struct Map {
    handle: winapi::HANDLE,
    ptr: *mut LinkedMem,
}

impl Map {
    pub fn new() -> io::Result<Map> {
        unsafe {
            let handle = kernel32::OpenFileMappingW(
                winapi::FILE_MAP_ALL_ACCESS,
                winapi::FALSE,
                wide!(M u m b l e L i n k).as_ptr(),
            );
            if handle.is_null() {
                return Err(io::Error::last_os_error());
            }
            let ptr = kernel32::MapViewOfFile(
                handle,
                winapi::FILE_MAP_ALL_ACCESS,
                0,
                0,
                mem::size_of::<LinkedMem>() as u64,
            );
            if ptr.is_null() {
                kernel32::CloseHandle(handle);
                return Err(io::Error::last_os_error());
            }
            Ok(Map {
                handle: handle,
                ptr: ptr as *mut LinkedMem,
            })
        }
    }

    // TODO: resolve "private type in public interface" warning, either with
    // `pub(crate)` syntax when it's stable or moving `LinkedMem` to this mod.
    pub fn as_mut(&mut self) -> &mut LinkedMem {
        unsafe { &mut *self.ptr }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            kernel32::CloseHandle(self.handle);
        }
    }
}
