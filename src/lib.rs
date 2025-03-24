pub mod math;
pub mod png;
pub mod show;

use std::env::args;
use std::time::SystemTime;
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

#[no_mangle]
pub extern "C" fn _main() {
    main()
}

#[no_mangle]
pub extern "C" fn fail_assertion(msg: *const i8) {
    let msg = to_str(msg);

    println!("[abort] {}", msg);
    exit(1);
}

#[no_mangle]
pub extern "C" fn _fail_assertion(msg: *const i8) {
    fail_assertion(msg)
}

#[no_mangle]
pub extern "C" fn jpl_alloc(size: u64) -> *mut c_void {
    if size <= 0 {
        fail!(
            "jpl_alloc",
            "Could not allocate 0 or negative amount of memory",
        );
    } else {
        // TODO: not sure if i could implement a jpl_free with this current version
        let mem = vec![0u8; size as usize].into_boxed_slice();
        Box::into_raw(mem) as *mut c_void
    }
}

#[no_mangle]
pub extern "C" fn _jpl_alloc(size: u64) -> *mut c_void {
    jpl_alloc(size)
}

#[no_mangle]
pub extern "C" fn get_time() -> f64 {
    SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|_| fail!("get_time", "could not measure time"))
        .as_secs_f64()
}

#[no_mangle]
pub extern "C" fn _get_time() -> f64 {
    get_time()
}

#[no_mangle]
pub extern "C" fn print_time(time: f64) {
    println!("[time] {:.6}ms", time * 1000.0)
}

#[no_mangle]
pub extern "C" fn _print_time(time: f64) {
    print_time(time)
}

#[no_mangle]
pub extern "C" fn print(msg: *const i8) {
    let msg = to_str(msg);

    println!("{}", msg);
}

#[no_mangle]
pub extern "C" fn _print(msg: *const i8) {
    print(msg)
}

#[no_mangle]
pub extern "C" fn to_int(n: f64) -> i64 {
    if n.is_nan() {
        0
    } else if n == f64::INFINITY {
        i64::MAX
    } else if n == f64::NEG_INFINITY {
        i64::MIN
    } else {
        n as i64
    }
}

#[no_mangle]
pub extern "C" fn _to_int(n: f64) -> i64 {
    to_int(n)
}

#[no_mangle]
pub extern "C" fn to_double(n: u64) -> f64 {
    n as f64
}

#[no_mangle]
pub extern "C" fn _to_double(n: u64) -> f64 {
    to_double(n)
}
