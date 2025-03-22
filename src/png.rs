use core::slice;
use std::{ffi::CStr, mem::forget};

use image::{open, ImageBuffer, Rgba, RgbaImage};

use crate::fail;

#[repr(C)]
pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[repr(C)]
pub struct Img {
    pub d0: u64,
    pub d1: u64,
    pub data: *mut Pixel,
}

#[no_mangle]
pub extern "C" fn read_image(filename: *const i8) -> Img {
    let filename = unsafe { CStr::from_ptr(filename) }
        .to_str()
        .expect("strings in jpl should be valid utf8");

    let img = open(filename)
        .unwrap_or_else(|err| fail!("read_image", "could not open image: {}", err))
        .to_rgba8();

    let (cols, rows) = img.dimensions();

    let mut pixels: Vec<_> = img
        .enumerate_pixels()
        .map(|(_, _, Rgba([r, g, b, a]))| Pixel {
            r: (*r as f64 / 255.0),
            g: (*g as f64 / 255.0),
            b: (*b as f64 / 255.0),
            a: (*a as f64 / 255.0),
        })
        .collect();
    let data = pixels.as_mut_ptr();
    forget(pixels);
    Img {
        d0: rows as u64,
        d1: cols as u64,
        data,
    }
}

#[no_mangle]
pub extern "C" fn write_image(img: Img, filename: *const i8) {
    let filename = unsafe { CStr::from_ptr(filename) }
        .to_str()
        .expect("strings in jpl should be valid utf8");

    let pixels = unsafe { slice::from_raw_parts(img.data, (img.d0 * img.d1) as usize) };

    let mut buffer: RgbaImage = ImageBuffer::new(img.d1 as u32, img.d0 as u32);

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let p = &pixels[(y * img.d1 as u32 + x) as usize];
        *pixel = Rgba([
            (p.r * 255.0) as u8,
            (p.g * 255.0) as u8,
            (p.b * 255.0) as u8,
            (p.a * 255.0) as u8,
        ]);
    }

    buffer
        .save(filename)
        .unwrap_or_else(|err| fail!("write_image", "could not open image: {}", err))
}
