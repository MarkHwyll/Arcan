#![no_std]
//set the bitmap allocator first
// then implement the stack allocator next

//Use the bootloader crate v 0.9 to get the memory map at start time
//to get the memory map
pub mod init {
    use bootloader::BootInfo;
    pub fn init(_bootinfo: &'static BootInfo) {}
}
