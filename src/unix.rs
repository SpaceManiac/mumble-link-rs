use std::{io, ptr};
use std::ffi::CString;
use libc::{self, wchar_t};

pub fn copy(dest: &mut [wchar_t], src: &str) {
    if dest.is_empty() { return }
    let mut index = 0;
    for ch in src.chars() {
        if index == dest.len() - 1 { break }
        dest[index] = ch as wchar_t;
        index += 1;
    }
    dest[index] = 0;
}

pub fn read(src: &[wchar_t]) -> String {
    let zero = src.iter().position(|&c| c == 0).unwrap_or(src.len());
    src[..zero].iter()
        .map(|&c| ::std::char::from_u32(c as u32).unwrap_or('\u{FFFD}'))
        .collect()
}

pub struct Map {
    fd: libc::c_int,
    pub ptr: *mut libc::c_void,
}

impl Map {
    pub fn new(size: usize) -> io::Result<Map> {
        unsafe {
            let path = CString::new(format!("/MumbleLink.{}", libc::getuid())).unwrap();
            let fd = libc::shm_open(
                path.as_ptr(),
                libc::O_RDWR,
                libc::S_IRUSR | libc::S_IWUSR
            );
            if fd < 0 {
                return Err(io::Error::last_os_error());
            }
            let ptr = libc::mmap(
                ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                0
            );
            if ptr as isize == -1 {
                libc::close(fd);
                return Err(io::Error::last_os_error());
            }
            Ok(Map {
                fd: fd,
                ptr: ptr,
            })
        }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}
