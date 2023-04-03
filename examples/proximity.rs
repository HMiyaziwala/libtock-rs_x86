//! A simple libtock-rs example. Checks for proximity driver
//! and samples the sensor every 2 seconds.

#![no_main]
#![no_std]

use core::fmt::Write;
use libtock::console::Console;

use libtock::alarm::{Alarm, Milliseconds};
use libtock::proximity::Proximity;
use libtock::runtime::{set_main, stack_size};

set_main! {main}
stack_size! {0x200}

fn main() {
    match Proximity::exists() {
        Ok(()) => writeln!(Console::writer(), "proximity driver available").unwrap(),
        Err(_) => {
            writeln!(Console::writer(), "proximity driver unavailable").unwrap();
            return;
        }
    }

    loop {
        match Proximity::read_proximity_sync() {
            Ok(prox_val) => writeln!(Console::writer(), "Proximity: {}\n", prox_val).unwrap(),
            Err(_) => writeln!(Console::writer(), "error while reading proximity",).unwrap(),
        }

        Alarm::sleep_for(Milliseconds(2000)).unwrap();
    }
}
