#![feature(start)]
#![no_std]

use mstd::entry_point;
use mstd::syscall::sys_write;

entry_point!(main);

fn main() {
    let data = b"Hello, world!\n";
    sys_write(1, data.as_ptr(), data.len());
}
