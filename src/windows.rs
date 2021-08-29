use std::io;
use libc::{c_void, wchar_t};
use winapi::um::{winnt, memoryapi, handleapi};

pub fn copy(dest: &mut [wchar_t], src: &str) {
    if dest.is_empty() { return }
    let mut index = 0;
    for ch in src.encode_utf16() {
        if index == dest.len() - 1 { break }
        dest[index] = ch;
        index += 1;
    }
    dest[index] = 0;
}

pub fn read(src: &[wchar_t]) -> String {
    let zero = src.iter().position(|&c| c == 0).unwrap_or(src.len());
    String::from_utf16_lossy(&src[..zero])
}

pub struct Map {
    handle: winnt::HANDLE,
    pub ptr: *mut c_void,
}

impl Map {
    pub fn new(size: usize) -> io::Result<Map> {
        unsafe {
            let handle = memoryapi::OpenFileMappingW(
                memoryapi::FILE_MAP_ALL_ACCESS,
                winapi::shared::minwindef::FALSE,
                wide!(M u m b l e L i n k).as_ptr(),
            );
            if handle.is_null() {
                return Err(io::Error::last_os_error());
            }
            let ptr = memoryapi::MapViewOfFile(
                handle,
                memoryapi::FILE_MAP_ALL_ACCESS,
                0,
                0,
                size,
            );
            if ptr.is_null() {
                handleapi::CloseHandle(handle);
                return Err(io::Error::last_os_error());
            }
            Ok(Map {
                handle: handle,
                ptr: ptr as *mut c_void,
            })
        }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            handleapi::CloseHandle(self.handle);
        }
    }
}
