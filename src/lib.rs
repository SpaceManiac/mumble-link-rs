extern crate libc;

use std::io;
use libc::{c_float, wchar_t};

#[macro_use]
mod wide;

#[cfg_attr(windows, path="windows.rs")]
#[cfg_attr(not(windows), path="unix.rs")]
mod imp;

const CONTEXT_LEN: usize = 256;

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

struct LinkedMem {
    ui_version: u32,
    ui_tick: imp::UiTick,
    avatar: Position,
    name: [wchar_t; 256],
    camera: Position,
    identity: [wchar_t; 256],
    context_len: u32,
    context: [u8; CONTEXT_LEN],
    description: [wchar_t; 2048],
}

pub struct MumbleLink {
    map: imp::Map,
}

impl MumbleLink {
    pub fn new(name: &str, description: &str) -> io::Result<MumbleLink> {
        let mut map = try!(imp::Map::new());
        unsafe {
            let mem = map.as_mut();
            wide::copy(&mut mem.name, name);
            wide::copy(&mut mem.description, description);
            mem.ui_version = 2;
        }
        Ok(MumbleLink {
            map: map,
        })
    }

    pub fn tick(&mut self, update: Update) {
        unsafe {
            let mem = self.map.as_mut();
            mem.ui_tick += 1;
            mem.avatar = update.avatar;
            mem.camera = update.camera;
            wide::copy(&mut mem.identity, update.identity);
            let len = std::cmp::min(update.context.len(), CONTEXT_LEN);
            mem.context[..len].copy_from_slice(&update.context[..len]);
            mem.context_len = len as u32;
        }
    }
}

#[derive(Default)]
pub struct Update<'a> {
    pub avatar: Position,
    pub camera: Position,
    pub identity: &'a str,
    pub context: &'a [u8],
}
