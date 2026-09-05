use uefi::CStr16;
use uefi::boot::{self, ScopedProtocol};
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;

pub fn load_kernel_image(path: &CStr16) -> Vec<u64> {
    let handle =
        boot::get_handle_for_protocol::<SimpleFileSystem>().expect("NO FILE PROTOCOL FOUND");
    let mut file_system = boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
        .expect("FAILED TO OPEN SIMPLEFILESYSTEM.");
    let mut root_dir = file_system
        .open_volume()
        .expect("ROOT DIRECTORY NOT FOUND./FAILED TO OPEN ROOT DIRECTORY");
    let mut image = root_dir
        .open(path, FileMode::Read, FileAttribute::READ_ONLY)
        .expect("COULD NOT RETRIVE FILE HANDLE");
    let bytes = image.read();

    bytes
}
