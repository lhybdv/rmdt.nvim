mac_build_so:
	cargo build --release
	cp target/release/librmdt.dylib lua/rmdt.so
