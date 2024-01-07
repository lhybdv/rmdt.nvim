mac_build_so:
	if [ ! -d "lua" ]; then mkdir "lua"; fi
	cargo build --release
	cp target/release/librmdt.dylib lua/rmdt.so

