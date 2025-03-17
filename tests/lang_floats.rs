#[macro_use]
mod util;

use conv2::*;

use conv2::FloatError::NegOverflow as FU;
use conv2::FloatError::PosOverflow as FO;

#[test]
fn test_f32() {
    check!(f32, f32=> fident; qv: *;);
    check!(f32, f64=> fident; qv: *;);
}

#[test]
fn test_f32_to_int() {
    check!(f32, i8=>  sidenta; qa: i8=>  a: -129.0, !FU; a: 128.0, !FO;);
    check!(f32, i16=> sidenta; qa: i16=> a: -32_769.0, !FU; a: 32_768.0, !FO;);
    check!(f32, i32=> sidenta; qa: i32=>
        a: -2.1474836e9, -2147483648; a: 2.1474835e9, 2147483520;
        a: -2_147_500_000.0, !FU; a: 2_147_500_000.0, !FO;);
    check!(f32, i64=> sidenta; qa: i64=>
        a: -9.223372e18, -9223372036854775808; a: 9.2233715e18, 9223371487098961920;
        a: -9_223_373_000_000_000_000.0, !FU; a: 9_223_373_000_000_000_000.0, !FO;);
    check!(f32, u8=>  uidenta; qa: u8=>  a: -1.0, !FU; a: 256.0, !FO;);
    check!(f32, u16=> uidenta; qa: u16=> a: -1.0, !FU; a: 65_536.0, !FO;);
    check!(f32, u32=> uidenta; qa: u32=>
        a: 4.294967e9, 4294967040;
        a: -1.0, !FU; a: 4_294_968_000.0, !FO;);
    check!(f32, u64=> uidenta; qa: u64=>
        a: 1.8446743e19, 18446742974197923840;
        a: -1.0, !FU; a: 18_446_746_000_000_000_000.0, !FO;);
}

#[test]
fn test_f64_to_int() {
    check!(f64, i8=>  sidenta; qa: i8=>  a: -129.0, !FU; a: 128.0, !FO;);
    check!(f64, i16=> sidenta; qa: i16=> a: -32_769.0, !FU; a: 32_768.0, !FO;);
    check!(f64, i32=> sidenta; qa: i32=> a: -2_147_483_649.0, !FU; a: 2_147_483_648.0, !FO;);
    check!(f64, i64=> sidenta; qa: i64=>
        a: -9.223372036854776e18, -9223372036854775808;
        a: 9.223372036854775e18, 9223372036854774784;
        a: -9_223_372_036_854_778_000.0, !FU; a: 9_223_372_036_854_778_000.0, !FO;);
    check!(f64, u8=>  uidenta; qa: u8=>  a: -1.0, !FU; a: 256.0, !FO;);
    check!(f64, u16=> uidenta; qa: u16=> a: -1.0, !FU; a: 65_536.0, !FO;);
    check!(f64, u32=> uidenta; qa: u32=> a: -1.0, !FU; a: 4_294_967_296.0, !FO;);
    check!(f64, u64=> uidenta; qa: u64=>
        a: 1.844674407370955e19;
        a: -1.0, !FU; a: 18_446_744_073_709_560_000.0, !FO;);
}

#[test]
fn test_f64() {
    use conv2::RangeError::NegOverflow as RU;
    use conv2::RangeError::PosOverflow as RO;

    // A value just barely too negative for an f32.
    const F32UNDER: f64 = f32::MIN as f64 - 1e23;
    // A value just barely too positive for an f32.
    const F32OVER: f64 = f32::MAX as f64 + 1e23;

    check!(f64, f32=> fidenta; qa: f32=>  a: F32UNDER, !RU; a: F32OVER, !RO;);
    check!(f64, f64=> fident; qv: *;);
}

#[test]
fn test_rounding() {
    use conv2::{RoundToNearest, RoundToNegInf, RoundToPosInf, RoundToZero};

    assert_eq!((8.5f32).approx_as::<u8>(), Ok(8));
    assert_eq!((8.5f32).approx_as_by::<u8, RoundToZero>(), Ok(8));
    assert_eq!((8.5f32).approx_as_by::<u8, RoundToNegInf>(), Ok(8));
    assert_eq!((8.5f32).approx_as_by::<u8, RoundToPosInf>(), Ok(9));
    assert_eq!((8.5f32).approx_as_by::<u8, RoundToNearest>(), Ok(9));

    assert_eq!((-8.5f32).approx_as::<i8>(), Ok(-8));
    assert_eq!((-8.5f32).approx_as_by::<i8, RoundToZero>(), Ok(-8));
    assert_eq!((-8.5f32).approx_as_by::<i8, RoundToNegInf>(), Ok(-9));
    assert_eq!((-8.5f32).approx_as_by::<i8, RoundToPosInf>(), Ok(-8));
    assert_eq!((-8.5f32).approx_as_by::<i8, RoundToNearest>(), Ok(-9));

    // When converting float to int, it's possible that different rounding modes
    // will cause the same input to overflow the output type.

    // Verify that DefaultApprox rounds toward zero, and allows values
    // that start out 0.9 beyond the maximum allowed.
    // This can't be done for some combinations, e.g. f32->i32 because
    // there are no f32 values with a fractional part that exceed the
    // min/max i32 value.
    assert_eq!((127.9f32).approx_as::<i8>(), Ok(127));
    assert_eq!((-128.9f32).approx_as::<i8>(), Ok(-128));
    assert_eq!((255.9f32).approx_as::<u8>(), Ok(255));
    assert_eq!((32767.9f32).approx_as::<i16>(), Ok(32767));
    assert_eq!((-32768.9f32).approx_as::<i16>(), Ok(-32768));
    assert_eq!((65535.9f32).approx_as::<u16>(), Ok(65535));

    assert_eq!((2147483647.9f64).approx_as::<i32>(), Ok(2147483647));
    assert_eq!((-2147483648.9f64).approx_as::<i32>(), Ok(-2147483648));
    assert_eq!((4294967295.9f64).approx_as::<u32>(), Ok(4294967295));

    // Test the other rounding modes.
    assert_eq!((255.5f32).approx_as_by::<u8, RoundToZero>(), Ok(255));
    assert_eq!((255.5f32).approx_as_by::<u8, RoundToNegInf>(), Ok(255));
    assert_eq!(
        (255.5f32).approx_as_by::<u8, RoundToPosInf>(),
        Err(FloatError::PosOverflow(255.5))
    );
    assert_eq!(
        (255.5f32).approx_as_by::<u8, RoundToNearest>(),
        Err(FloatError::PosOverflow(255.5))
    );
}

/// Increment an f32 by the minimum possible step.
///
/// The `step` input should be 1.0 or -1.0; we will increase
/// it until it actually changes the value.
fn step(x: f32, mut step: f32) -> f32 {
    let mut y = x;
    loop {
        y += step;
        if y != x {
            return y;
        }
        step *= 2.0;
    }
}

/// Increment an f32 by the minimum possible step.
///
/// The `step` input should be 1.0 or -1.0; we will increase
/// it until it actually changes the value.
fn step64(x: f64, mut step: f64) -> f64 {
    let mut y = x;
    loop {
        y += step;
        if y != x {
            return y;
        }
        step *= 2.0;
    }
}

#[test]
fn test_limits() {
    use conv2::{MAX_F32_I32, MAX_F32_I64, MIN_F32_I32, MIN_F32_I64};

    // Verify that the min and max values we use are the actual limits:
    // they succeed, and anything further from zero will fail.
    // Additionally, verify that anything further away will exceed
    // integer limits for that type.

    // f32 -> i32/u32

    assert_eq!(MAX_F32_I32.approx_as::<i32>(), Ok(2147483520));
    assert!(step(MAX_F32_I32, 1.0).approx_as::<i32>().is_err());
    assert!(step(MAX_F32_I32, 1.0) as i64 > i32::MAX as i64);

    assert_eq!(MIN_F32_I32.approx_as::<i32>(), Ok(-2147483648));
    assert!(step(MIN_F32_I32, -1.0).approx_as::<i32>().is_err());
    assert!((step(MIN_F32_I32, -1.0) as i64) < i32::MIN as i64);

    assert_eq!(MAX_F32_U32.approx_as::<u32>(), Ok(4294967040));
    assert!(step(MAX_F32_U32, 1.0).approx_as::<u32>().is_err());
    assert!(step(MAX_F32_U32, 1.0) as u64 > u32::MAX as u64);

    assert_eq!(0.0f32.approx_as::<u32>(), Ok(0));
    assert_eq!((-0.0f32).approx_as::<u32>(), Ok(0));

    // f32 -> i64/u64

    assert_eq!(MAX_F32_I64.approx_as::<i64>(), Ok(9223371487098961920));
    assert!(step(MAX_F32_I64, 1.0).approx_as::<i64>().is_err());
    assert!(step(MAX_F32_I64, 1.0) as i128 > i64::MAX as i128);

    assert_eq!(MIN_F32_I64.approx_as::<i64>(), Ok(-9223372036854775808));
    assert!(step(MIN_F32_I64, -1.0).approx_as::<i64>().is_err());
    assert!((step(MIN_F32_I64, -1.0) as i128) < i64::MIN as i128);

    assert_eq!(MAX_F32_U64.approx_as::<u64>(), Ok(18446742974197923840));
    assert!(step(MAX_F32_U64, 1.0).approx_as::<u64>().is_err());
    assert!(step(MAX_F32_U64, 1.0) as u128 > u64::MAX as u128);

    assert_eq!(0.0f32.approx_as::<u64>(), Ok(0));
    assert_eq!((-0.0f32).approx_as::<u64>(), Ok(0));

    // f64 -> i64/u64

    assert_eq!(MAX_F64_I64.approx_as::<i64>(), Ok(9223372036854774784));
    assert!(step64(MAX_F64_I64, 1.0).approx_as::<i64>().is_err());
    assert!(step64(MAX_F64_I64, 1.0) as i128 > i64::MAX as i128);

    assert_eq!(MIN_F64_I64.approx_as::<i64>(), Ok(-9223372036854775808));
    assert!(step64(MIN_F64_I64, -1.0).approx_as::<i64>().is_err());
    assert!((step64(MIN_F64_I64, -1.0) as i128) < i64::MIN as i128);

    assert_eq!(MAX_F64_U64.approx_as::<u64>(), Ok(18446744073709549568));
    assert!(step64(MAX_F64_U64, 1.0).approx_as::<u64>().is_err());
    assert!(step64(MAX_F64_U64, 1.0) as u128 > u64::MAX as u128);

    assert_eq!(0.0f32.approx_as::<u64>(), Ok(0));
    assert_eq!((-0.0f32).approx_as::<u64>(), Ok(0));
}
