//! **mumble-link** provides an API for using the [Mumble Link][link] plugin
//! for position-aware VoIP communications.
//!
//! [link]: https://wiki.mumble.info/wiki/Link
//!
//! Connect to Mumble link with `MumbleLink::new()`, set the context or player
//! identity as needed, and call `update()` every frame with the position data.

extern crate libc;

use std::{io, ptr};
use libc::{c_float, wchar_t};

#[macro_use]
mod wide;

#[cfg_attr(windows, path="windows.rs")]
#[cfg_attr(not(windows), path="unix.rs")]
mod imp;

#[derive(Copy)]
struct LinkedMem {
    ui_version: u32,
    ui_tick: u32,
    avatar: Position,
    name: [wchar_t; 256],
    camera: Position,
    identity: [wchar_t; 256],
    context_len: u32,
    context: [u8; 256],
    description: [wchar_t; 2048],
}

impl Clone for LinkedMem {
    fn clone(&self) -> Self { *self }
}

/// An active Mumble link connection.
pub struct MumbleLink {
    map: imp::Map,
    local: LinkedMem,
}

impl MumbleLink {
    /// Open the Mumble link, providing the specified application name and
    /// description.
    ///
    /// Opening the link may fail if Mumble is not running or another
    /// application is utilizing the Mumble link.
    pub fn new(name: &str, description: &str) -> io::Result<MumbleLink> {
        let map = try!(imp::Map::new(std::mem::size_of::<LinkedMem>()));
        let mut local = LinkedMem {
            ui_version: 2,
            ui_tick: 0,
            avatar: Position::default(),
            name: [0; 256],
            camera: Position::default(),
            identity: [0; 256],
            context_len: 0,
            context: [0; 256],
            description: [0; 2048],
        };
        wide::copy(&mut local.name, name);
        wide::copy(&mut local.description, description);

        let previous = unsafe { ptr::read_volatile(map.ptr as *mut LinkedMem) };
        if previous.ui_version != 0 {
            let name = wide::read(&previous.name);
            let description = wide::read(&previous.description);
            return Err(io::Error::new(io::ErrorKind::Other,
                format!("MumbleLink in use: {}: {}", name, description)))
        }

        Ok(MumbleLink {
            map: map,
            local: local,
        })
    }

    /// Update the context string, used to determine which users on a Mumble
    /// server should hear each other positionally.
    ///
    /// If context between two Mumble users does not match, the positional audio
    /// data is stripped server-side and voice will be received as
    /// non-positional. Accordingly, the context should only match for players
    /// on the same game, server, and map, depending on the game itself. When
    /// in doubt, err on the side of including less; this allows for more
    /// flexibility in the future.
    ///
    /// The context should be changed infrequently, at most a few times per
    /// second.
    ///
    /// The context has a maximum length of 256 bytes.
    pub fn set_context(&mut self, context: &[u8]) {
        let len = std::cmp::min(context.len(), self.local.context.len());
        self.local.context[..len].copy_from_slice(&context[..len]);
        self.local.context_len = len as u32;
    }

    /// Update the identity, uniquely identifying the player in the given
    /// context. This is usually the in-game name or ID.
    ///
    /// The identity may also contain any additional information about the
    /// player which might be useful for the Mumble server, for example to move
    /// teammates to the same channel or give squad leaders additional powers.
    /// It is recommended that a parseable format like JSON or CSV is used for
    /// this.
    ///
    /// The identity should be changed infrequently, at most a few times per
    /// second.
    ///
    /// The identity has a maximum length of 255 UTF-16 code units.
    pub fn set_identity(&mut self, identity: &str) {
        wide::copy(&mut self.local.identity, identity);
    }

    /// Update the link with the latest position information. Should be called
    /// once per frame.
    ///
    /// `avatar` should be the position of the player. If it is all zero,
    /// positional audio will be disabled. `camera` should be the position of
    /// the camera, which may be the same as `avatar`.
    pub fn update(&mut self, avatar: Position, camera: Position) {
        self.local.ui_tick = self.local.ui_tick.wrapping_add(1);
        self.local.avatar = avatar;
        self.local.camera = camera;
        unsafe {
            ptr::write_volatile(self.map.ptr as *mut LinkedMem, self.local);
        }
    }
}

impl Drop for MumbleLink {
    fn drop(&mut self) {
        unsafe {
            // set ui_version to 0
            ptr::write_volatile(self.map.ptr as *mut u32, 0);
        }
    }
}

/// A position in three-dimensional space.
///
/// The vectors are in a left-handed coordinate system: X positive towards
/// "right", Y positive towards "up", and Z positive towards "front". One unit
/// is treated as one meter by the sound engine.
///
/// `front` and `top` should be unit vectors and perpendicular to each other.
#[derive(Copy, Clone)]
pub struct Position {
    /// The character's position in space.
    pub position: [f32; 3],
    /// A unit vector pointing out of the character's eyes.
    pub front: [f32; 3],
    /// A unit vector pointing out of the top of the character's head.
    pub top: [f32; 3],
}

// `f32` is used above for tidyness; assert that it matches c_float.
const _ASSERT_CFLOAT_IS_F32: c_float = 0f32;

impl Default for Position {
    fn default() -> Self {
        Position {
            position: [0., 0., 0.],
            front: [0., 0., 1.],
            top: [0., 1., 0.],
        }
    }
}
