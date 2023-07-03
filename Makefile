install:
	mkdir -p /usr/local/include/ffout
	cp target/release/ffout.pc /usr/local/lib/pkgconfig/
	cp target/release/libffout.dylib /usr/local/lib/
	cp target/release/libffout.a /usr/local/lib/
	cp target/release/libffout.h /usr/local/include/ffout/

uninstall:
	rm -f /usr/local/lib/pkgconfig/ffout.pc
	rm -f /usr/local/lib/libffout.dylib
	rm -f /usr/local/lib/libffout.a
	rm -f /usr/local/include/ffout/libffout.h
	rmdir /usr/local/include/ffout
