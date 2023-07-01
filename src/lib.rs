use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn rust_fn() -> Result<()> {
    Ok(())
}

#[cfg(cargo_c)]
pub mod capi {
    use ffmpeg_sys_next::{AVFormatContext, AVPacket};
    use std::os::raw::c_int;
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
}

#[cfg(cargo_c)]
pub use capi::*;
