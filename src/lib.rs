pub mod math;

use std::env::args;
use std::{
    ffi::{c_void, CStr},
    process::exit,
};

#[repr(C)]
pub struct Args {
    pub argc: u64,
    pub data: *mut u64,
    _pad: [u64; 3], // Padding for calling convention
}

macro_rules! fail {
    ($who:expr, $($msg:tt)*) => {{
        eprintln!("[abort] {}: {}", $who, format_args!($($msg)*));
        std::process::exit(127);
    }};
}
pub(crate) use fail;

fn to_str<'a>(c_str: *const i8) -> &'a str {
    unsafe { CStr::from_ptr(c_str) }
        .to_str()
        .unwrap_or_else(|_| fail!("to_str", "Could not parse c strings"))
}

extern "C" {
    pub fn jpl_main(args: Args);
}

#[no_mangle]
pub extern "C" fn main() {
    let parsed_args = args()
        .skip(1)
        .map(|s| u64::from_str_radix(&s, 10))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|err| fail!("main", "Could not parse command line arguments: {}", err))
        .into_boxed_slice();

    let argc = parsed_args.len() as u64;
    let data = Box::into_raw(parsed_args) as *mut u64;

    let args = Args {
        argc,
        data,
        _pad: [0; 3],
    };
    unsafe { jpl_main(args) };
    let slice_ptr = std::ptr::slice_from_raw_parts_mut(data, argc as usize);
    unsafe {
        drop(Box::from_raw(slice_ptr));
    }
}

