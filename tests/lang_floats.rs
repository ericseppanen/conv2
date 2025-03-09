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

    // This is just to verify that DefaultApprox rounds toward zero, because
    // this means that it's incorrect because it tries to do the rounding
    // after the bounds check.
    assert_eq!((255.5f32).approx_as::<u16>(), Ok(255)); // FIXME: delete this once bug is fixed.

    //assert_eq!((255.5f32).approx_as::<u8>(), Ok(255)); // FIXME: incorrect?
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
