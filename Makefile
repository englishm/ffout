install:
	cargo build --release
	mkdir -p /usr/local/include/libffout
	cp target/release/libffout.pc /usr/local/lib/pkgconfig/
	cp target/release/libffout.dylib /usr/local/lib/
	cp target/release/libffout.a /usr/local/lib/
	cp target/release/ffout.h /usr/local/include/libffout/

uninstall:
	rm -f /usr/local/lib/pkgconfig/libffout.pc
	rm -f /usr/local/lib/libffout.dylib
	rm -f /usr/local/lib/libffout.a
	rm -f /usr/local/include/libffout/ffout.h
	rmdir /usr/local/include/libffout
