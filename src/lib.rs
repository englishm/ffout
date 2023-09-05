extern crate ffmpeg_sys_next;

use std::error::Error;
use std::io::{Cursor, Read};

//use ffmpeg_sys_next::{AVFormatContext, AVPacket, URLContext};
use ffmpeg_sys_next::{AVClass, AVFormatContext, AVPacket};
use std::os::raw::{c_char, c_int, c_uchar};

use mp4::{self, ReadBox};

#[repr(C)]
pub struct MyURLProtocol {
    // TODO
    pub name: *const c_char,
    pub url_open: *const fn(*mut MyURLContext, *const c_char, c_int) -> c_int,
}

#[repr(C)]
pub struct MoQContext {
    // TODO
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MyURLContext {
    pub av_class: *const AVClass,
    pub prot: *const MyURLProtocol,
    pub priv_data: *mut MoQContext,
    pub filename: *mut c_char,
    pub flags: c_int,
    pub max_packet_size: c_int,
    pub is_streamed: c_int,
    pub is_connected: c_int,
    // ...
}

#[no_mangle]
pub extern "C" fn ffout_open(
    url_ctx_ptr: *mut MyURLContext,
    _url: *const c_char,
    _flags: c_int,
) -> c_int {
    println!("hi from ffout_open");
    let mut url_ctx = unsafe { *url_ctx_ptr };
    dbg!(url_ctx);
    let filename = unsafe { *url_ctx.filename }.to_string();
    let proto = url_ctx.prot;
    dbg!(filename);
    0 // Return value, replace with actual implementation
}

#[no_mangle]
pub extern "C" fn ffout_write(
    url_ctx_ptr: *mut MyURLContext,
    buf: *const c_uchar,
    size: c_int,
) -> c_int {
    let mut slice_of_bytes = unsafe { std::slice::from_raw_parts(buf, size.try_into().unwrap()) };
    let atom = read_atom(&mut slice_of_bytes).unwrap();
    //dbg!(&atom);
    let mut atom_reader = Cursor::new(&atom);
    let atom_header = mp4::BoxHeader::read(&mut atom_reader).unwrap();
    let name = atom_header.name.to_string();
    if !vec!["moof", "mdat"].contains(&name.as_str()) {
        println!("hi from ffout_write");
        println!("received {} bytes", &size);
        println!("buf (ptr) {:?}", &buf);
        dbg!(url_ctx_ptr);
        dbg!(&atom_header);
        dbg!(atom.len());
    }
    atom.len() as i32 // Return value, replace with actual implementation
}

#[no_mangle]
pub extern "C" fn ffout_close(_url_ctx_ptr: *mut MyURLContext) -> c_int {
    println!("ffout_close called");
    0
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

// Read a full MP4 atom into a vector.
fn read_atom<R: Read>(reader: &mut R) -> Result<Vec<u8>, Box<dyn Error>> {
    // Read the 8 bytes for the size + type
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;

    // Convert the first 4 bytes into the size.
    let size = u32::from_be_bytes(buf[0..4].try_into()?) as u64;

    let mut raw = buf.to_vec();

    //debug!("size: {}", &size);

    let mut limit = match size {
        // Runs until the end of the file.
        0 => reader.take(u64::MAX),

        // The next 8 bytes are the extended size to be used instead.
        1 => {
            reader.read_exact(&mut buf)?;
            let size_large = u64::from_be_bytes(buf);
            assert!(size_large >= 16);
            // anyhow::ensure!(
            //     size_large >= 16,
            //     "impossible extended box size: {}",
            //     size_large
            // );

            reader.take(size_large - 16)
        }

        2..=7 => {
            // anyhow::bail!("impossible box size: {}", size)
            panic!("impossible box size: {}", size)
        }

        // Otherwise read based on the size.
        size => reader.take(size - 8),
    };

    // Append to the vector and return it.
    let _read_bytes = limit.read_to_end(&mut raw)?;

    Ok(raw)
}
