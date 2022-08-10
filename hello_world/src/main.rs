#![feature(start)]
#![no_std]

use mstd::println;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    0
}

fn main() {
    println!("Hello, world!");
}
