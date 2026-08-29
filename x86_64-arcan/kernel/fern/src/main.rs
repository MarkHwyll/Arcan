#![no_std]
#![no_main]

use log::info;
use uefi::prelude::*;
use core::time::Duration;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Hello from Fern, not Cern");
    boot::stall(Duration::from_secs(10));
    Status::SUCCESS
}
