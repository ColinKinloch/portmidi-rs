#[crate_id = "portmidiex1#0.0.1"];
#[license = "MIT"];

#[feature(globs)];

extern crate portmidi;

use portmidi::midi;
use std::io::stdio::println;

#[main]
fn main() {
    println("hello?");
    let error:midi::PmError = midi::initialize();
    println!("res :{:?}", error);
}
