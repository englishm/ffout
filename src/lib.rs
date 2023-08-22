extern crate ffmpeg_sys_next;

use ffmpeg_sys_next::{AVFormatContext, AVPacket, URLContext};
use std::os::raw::{c_char, c_int, c_uchar};

#[no_mangle]
pub extern "C" fn ffout_open(
    _url_ctx: *mut URLContext,
    _url: *const c_char,
    _flags: c_int,
) -> c_int {
    println!("hi from ffout_open");
    0 // Return value, replace with actual implementation
}

#[no_mangle]
pub extern "C" fn ffout_write(url_ctx: *mut URLContext, buf: *const c_uchar, size: c_int) -> c_int {
    println!("hi from ffout_write");
    println!("received {} bytes", &size);
    println!("buf (ptr) {:?}", &buf);
    let _slice_of_bytes = unsafe { std::slice::from_raw_parts(buf, size.try_into().unwrap()) };
    dbg!(url_ctx);
    //dbg!(slice_of_bytes);

    0 // Return value, replace with actual implementation
}

#[no_mangle]
pub extern "C" fn write_header(_ctx: *mut AVFormatContext) -> c_int {
    // Implementation of write_header
    0 // Return value, replace with actual implementation
}

#[no_mangle]
pub extern "C" fn write_packet(_ctx: *mut AVFormatContext, _pkt: *mut AVPacket) -> c_int {
    // Implementation of write_packet
    0 // Return value, replace with actual implementation
}

#[no_mangle]
pub extern "C" fn write_trailer(_ctx: *mut AVFormatContext) -> c_int {
    // Implementation of write_trailer
    0 // Return value, replace with actual implementation
}
