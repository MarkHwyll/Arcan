all: cr qemu
qemu:
	qemu-system-x86_64 -enable-kvm -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.4m.fd -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.4m.fd -drive format=raw,file=fat:rw:esp

cr:
	cp target/x86_64-unknown-uefi/debug/fern.efi esp/efi/boot/bootx64.efi

rs:
	cargo build --target x86_64-unknown-uefi

clean:
	cargo clean
	rm -rf esp/efi/boot/bootx64.efi

