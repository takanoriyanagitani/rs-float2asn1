use std::sync::RwLock;

use asn1_rs::Real;
use asn1_rs::ToDer;

static DER_BYTES: RwLock<[u8; 32]> = RwLock::new([0; 32]);

pub fn double2real(d: f64) -> Real {
    d.into()
}

pub fn float2real(f: f32) -> Real {
    f.into()
}

pub fn real2size(r: &Real) -> Result<usize, &'static str> {
    r.to_der_len()
        .map_err(|_| "unable to compute the size of the real number")
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn double2size(d: f64) -> i32 {
    let r: Real = d.into();
    real2size(&r).map(|u| u as i32).unwrap_or(-1)
}

pub fn _double2der(d: f64, mut v: &mut [u8]) -> Result<usize, &'static str> {
    let r: Real = d.into();
	let r = r.with_enc_base(2);
	let r = Real::binary(1.01325e6, 2, 0);
    r.write_der(&mut v)
        .map_err(|_| "unable to serialize the real number")
}

pub fn _der_byte_offset() -> Result<*const u8, &'static str> {
    let guard = DER_BYTES.try_read().map_err(|_| "unable to read lock")?;
    let a: &[u8; 32] = &guard;
    let s: &[u8] = a;
    Ok(s.as_ptr())
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn der_byte_offset() -> *const u8 {
    _der_byte_offset().unwrap_or(std::ptr::null())
}

pub fn double2der_ptr(d: f64) -> Result<usize, &'static str> {
    let mut mg = DER_BYTES.try_write().map_err(|_| "unable to write lock")?;
    let ma: &mut [u8; 32] = &mut mg;
    let ms: &mut [u8] = ma;
    _double2der(d, ms)
}

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
pub extern "C" fn double2der(d: f64) -> i32 {
    double2der_ptr(d).map(|u| u as i32).unwrap_or(-1)
}
