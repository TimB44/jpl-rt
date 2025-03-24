use std::f64;

#[no_mangle]
pub extern "C" fn sqrt(n: f64) -> f64 {
    n.sqrt()
}

#[no_mangle]
pub extern "C" fn _sqrt(n: f64) -> f64 {
    sqrt(n)
}

#[no_mangle]
pub extern "C" fn exp(n: f64) -> f64 {
    n.exp()
}

#[no_mangle]
pub extern "C" fn _exp(n: f64) -> f64 {
    exp(n)
}

#[no_mangle]
pub extern "C" fn sin(n: f64) -> f64 {
    n.sin()
}

#[no_mangle]
pub extern "C" fn _sin(n: f64) -> f64 {
    sin(n)
}

#[no_mangle]
pub extern "C" fn cos(n: f64) -> f64 {
    n.cos()
}

#[no_mangle]
pub extern "C" fn _cos(n: f64) -> f64 {
    cos(n)
}

#[no_mangle]
pub extern "C" fn tan(n: f64) -> f64 {
    n.tan()
}

#[no_mangle]
pub extern "C" fn _tan(n: f64) -> f64 {
    tan(n)
}

#[no_mangle]
pub extern "C" fn asin(n: f64) -> f64 {
    n.asin()
}

#[no_mangle]
pub extern "C" fn _asin(n: f64) -> f64 {
    asin(n)
}

#[no_mangle]
pub extern "C" fn acos(n: f64) -> f64 {
    n.acos()
}

#[no_mangle]
pub extern "C" fn _acos(n: f64) -> f64 {
    acos(n)
}

#[no_mangle]
pub extern "C" fn atan(n: f64) -> f64 {
    n.atan()
}

#[no_mangle]
pub extern "C" fn _atan(n: f64) -> f64 {
    atan(n)
}

#[no_mangle]
pub extern "C" fn log(n: f64) -> f64 {
    n.ln()
}

#[no_mangle]
pub extern "C" fn _log(n: f64) -> f64 {
    log(n)
}

#[no_mangle]
pub extern "C" fn pow(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

#[no_mangle]
pub extern "C" fn _pow(base: f64, exp: f64) -> f64 {
    pow(base, exp)
}

#[no_mangle]
pub extern "C" fn fmod(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

#[no_mangle]
pub extern "C" fn _fmod(base: f64, exp: f64) -> f64 {
    fmod(base, exp)
}
