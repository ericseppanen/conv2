#![allow(unused_imports)]
#![warn(clippy::cast_sign_loss, clippy::cast_possible_truncation)]

use std::arch::x86_64::{_mm_cvttss_si32, _mm_loadu_ps};

use conv2::{
    ConvAsUtil as _, ConvUtil as _, DefaultApprox, RoundToNearest, Saturate as _, ValueFrom,
    ValueInto as _,
};
use num_traits::NumCast;

pub fn old_main() {
    let x: f32 = 1e20;

    //let xi: i32 = x.approx_by::<RoundToNearest>().unwrap();
    //let xi = x as i32;

    let xi: u32 = NumCast::from(x).unwrap();

    //let xi: u32 = x.value_into().unwrap();

    println!("{xi:?}");
}

pub fn num_f32_u32(input: Vec<f32>) -> Vec<u32> {
    input
        .into_iter()
        .map(|input| {
            let output: u32 = NumCast::from(input).unwrap_or(0);
            output
        })
        .collect()
}

pub fn conv2_f32_u32(input: Vec<f32>) -> Vec<u32> {
    input
        .into_iter()
        .map(|input| {
            let output: u32 = input.approx().unwrap_or(0);
            output
        })
        .collect()
}

/// Convert f32 to i64 using the CVTTSS2SI instruction. If the input f32 is out of range of the output i64, then the result is i64::MIN.
#[inline(always)]
fn f32_to_i32(float: f32) -> i32 {
    // The compiler optimizes this function into a single instruction without the need for inline assembly.

    let floats = [float, 0., 0., 0.];
    let floats_pointer = floats.as_ptr();
    let floats_register = unsafe { _mm_loadu_ps(floats_pointer) };
    unsafe { _mm_cvttss_si32(floats_register) }
}

pub fn asm_f32_i32(input: Vec<f32>) -> Vec<i32> {
    input.into_iter().map(|input| f32_to_i32(input)).collect()
}
