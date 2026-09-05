all: kernel fern

kernel:
	cargo build
	cp target/x86_64-arcan/debug/kernel fern/
fern:
	cd fern
	make rs
	make
