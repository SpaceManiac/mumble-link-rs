//! **mumble-link** provides an API for using the [Mumble Link][link] plugin
//! for position-aware VoIP communications.
//!
//! [link]: https://wiki.mumble.info/wiki/Link

extern crate libc;

use std::io;
use libc::{c_float, wchar_t};

#[macro_use]
mod wide;

#[cfg_attr(windows, path="windows.rs")]
#[cfg_attr(not(windows), path="unix.rs")]
mod imp;

struct LinkedMem {
    ui_version: u32,
    ui_tick: imp::UiTick,
    avatar: Position,
    name: [wchar_t; 256],
    camera: Position,
    identity: [wchar_t; 256],
    context_len: u32,
    context: [u8; 256],
    description: [wchar_t; 2048],
}

/// An active Mumble link connection.
pub struct MumbleLink {
    map: imp::Map,
}

impl MumbleLink {
    /// Open the Mumble link, providing the specified application name and
    /// description.
    ///
    /// Opening the link may fail if Mumble is not running or another
    /// application is utilizing the Mumble link.
    pub fn new(name: &str, description: &str) -> io::Result<MumbleLink> {
        let map = try!(imp::Map::new(std::mem::size_of::<LinkedMem>()));
        unsafe {
            let mem = as_mut(map.ptr());
            if mem.ui_version != 0 {
                let zero = mem.name.iter().position(|&c| c == 0).unwrap_or(mem.name.len());
                let name = String::from_utf16_lossy(&mem.name[..zero]);
                let zero = mem.description.iter().position(|&c| c == 0).unwrap_or(mem.description.len());
                let description = String::from_utf16_lossy(&mem.description[..zero]);
                return Err(io::Error::new(io::ErrorKind::Other,
                    format!("MumbleLink in use: {}: {}", name, description)))
            }
            wide::copy(&mut mem.name, name);
            wide::copy(&mut mem.description, description);
            mem.ui_version = 2;
        }
        Ok(MumbleLink {
            map: map,
        })
    }

    /// Update the link with the latest player information. Should be called
    /// once per frame to be kept up to date.
    pub fn tick(&mut self, update: Update) {
        unsafe {
            let mem = as_mut(self.map.ptr());
            mem.ui_tick += 1;
            mem.avatar = update.avatar;
            mem.camera = update.camera;
            wide::copy(&mut mem.identity, update.identity);
            let len = std::cmp::min(update.context.len(), mem.context.len());
            mem.context[..len].copy_from_slice(&update.context[..len]);
            mem.context_len = len as u32;
        }
    }
}

impl Drop for MumbleLink {
    fn drop(&mut self) {
        unsafe {
            as_mut(self.map.ptr()).ui_version = 0;
        }
    }
}

unsafe fn as_mut<'a>(ptr: *mut libc::c_void) -> &'a mut LinkedMem {
    // TODO: determine how safe this is; may cause problems if another
    // process writes to the MumbleLink file as well.
    &mut *(ptr as *mut LinkedMem)
}

/// The data needed each frame to update the link.
#[derive(Default)]
pub struct Update<'a> {
    /// The position of the player's avatar in the world.
    pub avatar: Position,
    /// The position of the player's camera in the world.
    pub camera: Position,
    /// The identity of the player, such as an in-game name.
    pub identity: &'a str,
    /// A context value. Only players with equal contexts will be able to hear
    /// one another.
    pub context: &'a [u8],
}

/// A position in three-dimensional space.
///
/// The vectors are in a left-handed coordinate system: X positive towards
/// "right", Y positive towards "up", and Z positive towards "front". One unit
/// is treated as one meter by the sound engine.
pub struct Position {
    pub position: [c_float; 3],
    pub front: [c_float; 3],
    pub top: [c_float; 3],
}

impl Default for Position {
    fn default() -> Self {
        Position {
            position: [0., 0., 0.],
            front: [0., 0., 1.],
            top: [0., 1., 0.],
        }
    }
}
