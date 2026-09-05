#![no_std]
#![no_main]
#![allow(unused)]
mod kernell;

use core::time::Duration;
use log::{info, warn};
use uefi::boot::exit_boot_services;
use uefi::mem::memory_map::MemoryMapOwned;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("HELLO FROM FERN");
    let x = kernell::load_kernel_image(cstr16!("//sys/kernel"));
    info!("bytes is {:?}",x);
    let _memory_map: MemoryMapOwned = unsafe { exit_boot_services(None) };
    Status::SUCCESS
}
