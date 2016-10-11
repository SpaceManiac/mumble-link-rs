extern crate kernel32;
extern crate winapi;

use std::io;
use libc::c_void;

pub type UiTick = winapi::DWORD;

pub struct Map {
    handle: winapi::HANDLE,
    ptr: *mut c_void,
}

impl Map {
    pub fn new(size: usize) -> io::Result<Map> {
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
                size as u64,
            );
            if ptr.is_null() {
                kernel32::CloseHandle(handle);
                return Err(io::Error::last_os_error());
            }
            Ok(Map {
                handle: handle,
                ptr: ptr as *mut c_void,
            })
        }
    }

    pub fn ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            kernel32::CloseHandle(self.handle);
        }
    }
}
