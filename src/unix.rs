use std::{io, mem, ptr};
use std::ffi::CString;
use libc;
use LinkedMem;

pub type UiTick = u32;

pub struct Map {
    fd: libc::c_int,
    ptr: *mut LinkedMem,
}

impl Map {
    pub fn new() -> io::Result<Map> {
        unsafe {
            let path = CString::from(format!("/MumbleLink.{}", libc::getuid()));
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
                mem::size_of::<LinkedMem>(),
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
                ptr: ptr as *mut LinkedMem,
            })
        }
    }

    pub fn as_mut(&mut self) -> &mut LinkedMem {
        unsafe { &mut *self.ptr }
    }
}

impl Drop for Map {
    fn drop(&mut self) {
        unsafe {
            libc::close(self.fd);
        }
    }
}
