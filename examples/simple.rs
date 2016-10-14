extern crate mumble_link;
extern crate time;

use std::io;
use std::sync::mpsc;
use mumble_link::*;

fn read_line() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf
}

fn main() {
    println!("Attempting to open Link...");
    let mut link = SharedLink::new("Test", "test.");
    println!("Enter an identity:");
    link.set_identity(&read_line());

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let mut timer = Timer::new(1000 / 50);
        let mut position = Position::default();
        let mut i = 0;
        position.position[0] = 0.005;
        loop {
            timer.sleep_until_tick();
            link.update(position, position);
            i += 1;
            if i == 200 {
                i = 0;
                println!("Status: {:?}", link.status());
            }
            loop {
                match rx.try_recv() {
                    Ok(Command::Left) => position.position = [-2., 0., 0.],
                    Ok(Command::Right) => position.position = [2., 0., 0.],
                    Ok(Command::Middle) => position.position = [0.005, 0., 0.],
                    Ok(Command::Distant) => position.position = [1000., 0., 0.],
                    Ok(Command::Red) => link.set_context(b"red"),
                    Ok(Command::Blue) => link.set_context(b"blue"),
                    Ok(Command::Free) => link.deactivate(),
                    Err(mpsc::TryRecvError::Disconnected) => return,
                    Err(mpsc::TryRecvError::Empty) => break,
                }
            }
        }
    });

    let help = "Commands are: left, right, middle, distant, red, blue, free, exit";
    println!("{}", help);
    loop {
        let m = match read_line().trim() {
            "left" => Command::Left,
            "right" => Command::Right,
            "middle" => Command::Middle,
            "distant" => Command::Distant,
            "red" => Command::Red,
            "blue" => Command::Blue,
            "free" => Command::Free,
            "exit" => { drop(tx); break }
            _ => { println!("{}", help); continue }
        };
        tx.send(m).unwrap();
    }
    println!("Exiting");
}

enum Command {
    Left,
    Right,
    Middle,
    Distant,
    Red,
    Blue,
    Free,
}

// Timer that remembers when it is supposed to go off
struct Timer {
    next_tick_at: time::Timespec,
    tick_len: time::Duration,
}

impl Timer {
    fn new(tick_len_ms: u64) -> Timer {
        let tick_len = time::Duration::milliseconds(tick_len_ms as i64);
        Timer {
            next_tick_at: time::get_time() + tick_len,
            tick_len: tick_len,
        }
    }

    fn sleep_until_tick(&mut self) {
        let difference = self.next_tick_at - time::get_time();
        if difference > time::Duration::zero() {
            std::thread::sleep(std::time::Duration::from_millis(difference.num_milliseconds() as u64))
        }
        self.next_tick_at = self.next_tick_at + self.tick_len;
    }
}
