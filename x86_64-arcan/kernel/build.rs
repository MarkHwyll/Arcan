use std::path::Path;
use std::process::Command;

fn main() {
    let path = Path::new("./asm/bootloader.asm");
    let mut _new = Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg(path)
        .arg("-o")
        .arg("boot.o")
        .output()
        // .wait()
        // .spawn()
        .expect("Failed to execute bootloader");
    println!("cargo::rerun-if-changed=build.rs");
}
