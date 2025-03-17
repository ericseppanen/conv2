macro_rules! approx_blind {
    (($($attrs:tt)*), $src:ty, $dst:ty, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl crate::ApproxFrom<$src, $scheme> for $dst {
                type Err = crate::errors::NoError;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    Ok(src as $dst)
                }
            }
        }
    };
}

macro_rules! approx_z_to_dmax {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl crate::ApproxFrom<$src, $scheme> for $dst {
                type Err = crate::errors::RangeError<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if src < 0 {
                        return Err(crate::errors::RangeError::NegOverflow(src));
                    }
                    if src > $dst::MAX as $src {
                        return Err(crate::errors::RangeError::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
    };
}

macro_rules! approx_to_dmax {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl crate::ApproxFrom<$src, $scheme> for $dst {
                type Err = crate::errors::PosOverflow<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if src > $dst::MAX as $src {
                        return Err(crate::errors::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
    };
}

macro_rules! approx_dmin_to_dmax {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl crate::ApproxFrom<$src, $scheme> for $dst {
                type Err = crate::errors::RangeError<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if src < $dst::MIN as $src  {
                        return Err(crate::errors::RangeError::NegOverflow(src));
                    }
                    if src > $dst::MAX as $src {
                        return Err(crate::errors::RangeError::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
    }
}

macro_rules! approx_z_up {
    (($($attrs:tt)*), $src:ty, $dst:ident, $scheme:ty) => {
        as_item! {
            $($attrs)*
            impl crate::ApproxFrom<$src, $scheme> for $dst {
                type Err = crate::errors::NegOverflow<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if src < 0 {
                        return Err(crate::errors::NegOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
    };
}

/// A fallible float->int conversion, with an explicit rounding step.
macro_rules! impl_float2int_round {
    ($src:ty, $dst:ident, [$min:expr, $max:expr], $scheme:ty, approx: |$src_name:ident| $conv:expr) => {
        as_item! {
            impl crate::ApproxFrom<$src, $scheme> for $dst {
                type Err = crate::errors::FloatError<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if src.is_nan() {
                        return Err(crate::errors::FloatError::NotANumber(src));
                    }
                    let approx = { let $src_name = src; $conv };
                    if approx < $min  {
                        return Err(crate::errors::FloatError::NegOverflow(src));
                    }
                    if approx > $max {
                        return Err(crate::errors::FloatError::PosOverflow(src));
                    }
                    Ok(unsafe { approx.to_int_unchecked::<$dst>() })
                }
            }
        }
    };
}

/// A fallible float->int conversion, with an implicit truncation for large integers.
///
/// By "large integers" we mean those that won't have any fractional part in the
/// floating point source value.
///
/// Limits are specified as the min/max values that succeed.
macro_rules! impl_float2int_trunc_large {
    ($src:ty, $dst:ident, [$min:expr, $max:expr], $scheme:ty) => {
        as_item! {
            impl crate::ApproxFrom<$src, $scheme> for $dst {
                type Err = crate::errors::FloatError<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if src.is_nan() {
                        return Err(crate::errors::FloatError::NotANumber(src));
                    }
                    if src < $min  {
                        return Err(crate::errors::FloatError::NegOverflow(src));
                    }
                    if src > $max {
                        return Err(crate::errors::FloatError::PosOverflow(src));
                    }
                    Ok(unsafe { src.to_int_unchecked::<$dst>() })
                }
            }
        }
    };
}

/// A fallible float->int conversion, with an implicit truncation for small integers.
///
/// Limits are specified as the first value that fails, rather than
/// the first value that succeeds.
macro_rules! impl_float2int_trunc {
    ($src:ty, $dst:ident, [$min:expr, $max:expr], $scheme:ty) => {
        as_item! {
            impl crate::ApproxFrom<$src, $scheme> for $dst {
                type Err = crate::errors::FloatError<$src>;
                #[inline]
                fn approx_from(src: $src) -> Result<$dst, Self::Err> {
                    if src.is_nan() {
                        return Err(crate::errors::FloatError::NotANumber(src));
                    }
                    if src <= $min  {
                        return Err(crate::errors::FloatError::NegOverflow(src));
                    }
                    if src >= $max {
                        return Err(crate::errors::FloatError::PosOverflow(src));
                    }
                    Ok(unsafe { src.to_int_unchecked::<$dst>() })
                }
            }
        }
    };
}

macro_rules! num_conv {
    (@ $src:ty=> $(,)*) => {};

    (@ $src:ty=> #[32] $($tail:tt)*) => {
        num_conv! { @ $src=> (#[cfg(target_pointer_width="32")]) $($tail)* }
    };

    (@ $src:ty=> #[64] $($tail:tt)*) => {
        num_conv! { @ $src=> (#[cfg(target_pointer_width="64")]) $($tail)* }
    };

    (@ $src:ty=> e   $($tail:tt)*) => { num_conv! { @ $src=> () e   $($tail)* } };
    (@ $src:ty=> n+  $($tail:tt)*) => { num_conv! { @ $src=> () n+  $($tail)* } };
    (@ $src:ty=> n   $($tail:tt)*) => { num_conv! { @ $src=> () n   $($tail)* } };
    (@ $src:ty=> w+  $($tail:tt)*) => { num_conv! { @ $src=> () w+  $($tail)* } };
    (@ $src:ty=> w   $($tail:tt)*) => { num_conv! { @ $src=> () w   $($tail)* } };
    (@ $src:ty=> aW  $($tail:tt)*) => { num_conv! { @ $src=> () aW  $($tail)* } };
    (@ $src:ty=> nf  $($tail:tt)*) => { num_conv! { @ $src=> () nf  $($tail)* } };
    (@ $src:ty=> fan $($tail:tt)*) => { num_conv! { @ $src=> () fan $($tail)* } };

    // Exact conversion
    (@ $src:ty=> ($($attrs:tt)*) e $dst:ty, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, crate::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, crate::Wrapping }

            $($attrs)*
            impl crate::ValueFrom<$src> for $dst {
                type Err = crate::errors::NoError;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Narrowing a signed type *into* an unsigned type where the destination type's maximum value is representable by the source type.
    (@ $src:ty=> ($($attrs:tt)*) n+ $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_z_to_dmax! { ($($attrs)*), $src, $dst, crate::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, crate::Wrapping }

            $($attrs)*
            impl crate::ValueFrom<$src> for $dst {
                type Err = crate::errors::RangeError<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if src < 0 {
                        return Err(crate::errors::RangeError::NegOverflow(src));
                    }
                    if src > $dst::MAX as $src {
                        return Err(crate::errors::RangeError::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Narrowing an unsigned type *into* a type where the destination type's maximum value is representable by the source type.
    (@ $src:ty=> ($($attrs:tt)*) n- $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_to_dmax! { ($($attrs)*), $src, $dst, crate::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, crate::Wrapping }

            $($attrs)*
            impl crate::ValueFrom<$src> for $dst {
                type Err = crate::errors::PosOverflow<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if src > $dst::MAX as $src {
                        return Err(crate::errors::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Narrowing where the destination type's bounds are representable by the source type.
    (@ $src:ty=> ($($attrs:tt)*) n $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_dmin_to_dmax! { ($($attrs)*), $src, $dst, crate::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, crate::Wrapping }

            $($attrs)*
            impl crate::ValueFrom<$src> for $dst {
                type Err = crate::errors::RangeError<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if src < $dst::MIN as $src {
                        return Err(crate::errors::RangeError::NegOverflow(src));
                    }
                    if src > $dst::MAX as $src {
                        return Err(crate::errors::RangeError::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Widening a signed type *into* an unsigned type.
    (@ $src:ty=> ($($attrs:tt)*) w+ $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_z_up! { ($($attrs)*), $src, $dst, crate::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, crate::Wrapping }

            $($attrs)*
            impl crate::ValueFrom<$src> for $dst {
                type Err = crate::errors::NegOverflow<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if src < 0 {
                        return Err(crate::errors::NegOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Widening.
    (@ $src:ty=> ($($attrs:tt)*) w $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, crate::DefaultApprox }
            approx_blind! { ($($attrs)*), $src, $dst, crate::Wrapping }

            $($attrs)*
            impl crate::ValueFrom<$src> for $dst {
                type Err = crate::errors::NoError;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    // Narrowing *into* a floating-point type where the conversion is only exact within a given range.
    (@ $src:ty=> ($($attrs:tt)*) nf [+- $bound:expr] $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, crate::DefaultApprox }

            $($attrs)*
            impl crate::ValueFrom<$src> for $dst {
                type Err = crate::errors::RangeError<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if src < -$bound {
                        return Err(crate::errors::RangeError::NegOverflow(src));
                    }
                    if src > $bound {
                        return Err(crate::errors::RangeError::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    (@ $src:ty=> ($($attrs:tt)*) nf [, $max:expr] $dst:ident, $($tail:tt)*) => {
        as_item! {
            approx_blind! { ($($attrs)*), $src, $dst, crate::DefaultApprox }

            $($attrs)*
            impl crate::ValueFrom<$src> for $dst {
                type Err = crate::errors::PosOverflow<$src>;
                #[inline]
                fn value_from(src: $src) -> Result<$dst, Self::Err> {
                    if src > $max {
                        return Err(crate::errors::PosOverflow(src));
                    }
                    Ok(src as $dst)
                }
            }
        }
        num_conv! { @ $src=> $($tail)* }
    };

    ($src:ty=> $($tail:tt)*) => {
        num_conv! { @ $src=> $($tail)*, }
    };
}

macro_rules! num_conv_float2int {
    // Convert float->int, using explicit min and max range limits.
    ($src:ty => [$min:expr, $max:expr] $dst:ident) => {
        as_item! {
            impl_float2int_trunc_large! { $src, $dst, [$min, $max], crate::DefaultApprox }
            impl_float2int_trunc_large! { $src, $dst, [$min, $max], crate::RoundToZero }
            impl_float2int_round! { $src, $dst, [$min, $max], crate::RoundToNearest, approx: |s| s.round() }
            impl_float2int_round! { $src, $dst, [$min, $max], crate::RoundToNegInf, approx: |s| s.floor() }
            impl_float2int_round! { $src, $dst, [$min, $max], crate::RoundToPosInf, approx: |s| s.ceil() }
        }
    };

    // Convert float->int, using the destination type's MIN and MAX as the allowed range.
    ($src:ty => $dst:ident) => {
        as_item! {
            impl_float2int_trunc! { $src, $dst, [$dst::MIN as $src - 1.0, $dst::MAX as $src + 1.0], crate::DefaultApprox }
            impl_float2int_trunc! { $src, $dst, [$dst::MIN as $src - 1.0, $dst::MAX as $src + 1.0], crate::RoundToZero }
            impl_float2int_round! { $src, $dst, [$dst::MIN as $src, $dst::MAX as $src], crate::RoundToNearest, approx: |s| s.round() }
            impl_float2int_round! { $src, $dst, [$dst::MIN as $src, $dst::MAX as $src], crate::RoundToNegInf, approx: |s| s.floor() }
            impl_float2int_round! { $src, $dst, [$dst::MIN as $src, $dst::MAX as $src], crate::RoundToPosInf, approx: |s| s.ceil() }
        }
    };
}

mod lang_ints {
    num_conv! { i8=>  w i16, w i32, w i64, w+u8, w+u16, w+u32, w+u64, w isize, w+usize }
    num_conv! { i16=> n i8, w i32, w i64, n+u8, w+u16, w+u32, w+u64, w isize, w+usize }
    num_conv! { i32=> n i8, n i16, w i64, n+u8, n+u16, w+u32, w+u64 }
    num_conv! { i64=> n i8, n i16, n i32, n+u8, n+u16, n+u32, w+u64 }
    num_conv! { i32=> #[32] e isize, #[64] w isize, w+usize }
    num_conv! { i64=> #[32] n isize, #[64] e isize, #[32] n+usize, #[64] w+usize }

    num_conv! { u8=> n-i8, w i16, w i32, w i64, w u16, w u32, w u64, w isize, w usize }
    num_conv! { u16=> n-i8, n-i16, w i32, w i64, n-u8, w u32, w u64, w isize, w usize }
    num_conv! { u32=> n-i8, n-i16, n-i32, w i64, n-u8, n-u16, w u64 }
    num_conv! { u64=> n-i8, n-i16, n-i32, n-i64, n-u8, n-u16, n-u32 }
    num_conv! { u32=> #[32] n-isize, #[64] w isize, #[32] e usize, #[64] w usize }
    num_conv! { u64=> n-isize, #[32] n-usize, #[64] e usize }

    num_conv! { isize=> n i8, n i16, #[32] e i32, #[32] w i64, #[64] n i32, #[64] e i64 }
    num_conv! { isize=> n+u8, n+u16, #[32] w+u32, #[32] w+u64, #[64] n+u32, #[64] w+u64 }
    num_conv! { isize=> w+usize }

    num_conv! { usize=> n-i8, n-i16, #[32] n-i32, #[32] w i64, #[64] n-i32, #[64] n-i64 }
    num_conv! { usize=> n-u8, n-u16, #[32] e u32, #[32] w u64, #[64] n-u32, #[64] e u64 }
    num_conv! { usize=> n-isize }
}

mod lang_floats {
    use crate::errors::{NoError, RangeError};
    use crate::ValueFrom;
    use crate::{ApproxFrom, ApproxScheme};

    // f32 -> f64: strictly widening
    impl<Scheme> ApproxFrom<f32, Scheme> for f64
    where
        Scheme: ApproxScheme,
    {
        type Err = NoError;
        #[inline]
        fn approx_from(src: f32) -> Result<f64, Self::Err> {
            Ok(src as f64)
        }
    }

    impl ValueFrom<f32> for f64 {
        type Err = NoError;
        #[inline]
        fn value_from(src: f32) -> Result<f64, Self::Err> {
            Ok(src as f64)
        }
    }

    // f64 -> f32: narrowing, approximate
    impl ApproxFrom<f64> for f32 {
        type Err = RangeError<f64>;
        #[inline]
        fn approx_from(src: f64) -> Result<f32, Self::Err> {
            if !src.is_finite() {
                return Ok(src as f32);
            }
            if src < f32::MIN as f64 {
                return Err(RangeError::NegOverflow(src));
            }
            if src > f32::MAX as f64 {
                return Err(RangeError::PosOverflow(src));
            }
            Ok(src as f32)
        }
    }
}

mod lang_int_to_float {
    num_conv! { i8=>  w f32, w f64 }
    num_conv! { i16=> w f32, w f64 }
    num_conv! { i32=> nf [+- 16_777_216] f32, w f64 }
    num_conv! { i64=> nf [+- 16_777_216] f32, nf [+- 9_007_199_254_740_992] f64 }

    num_conv! { u8=>  w f32, w f64 }
    num_conv! { u16=> w f32, w f64 }
    num_conv! { u32=> nf [, 16_777_216] f32, w f64 }
    num_conv! { u64=> nf [, 16_777_216] f32, nf [, 9_007_199_254_740_992] f64 }

    num_conv! { isize=> nf [+- 16_777_216] f32,
    #[32] w f64, #[64] nf [+- 9_007_199_254_740_992] f64 }
    num_conv! { usize=> nf [, 16_777_216] f32,
    #[32] w f64, #[64] nf [, 9_007_199_254_740_992] f64 }
}

mod lang_float_to_int {

    // Some limits need to be specified as floating point values, because it's
    // possible for an integer limit to be imprecise once cast to floating point.
    // Some of these values are equal to the integer MIN and MAX values cast to
    // floats, and some of them are not, because the cast-to-float sometimes
    // rounds up past the value that can safely be cast back to integer.

    num_conv_float2int!(f32 => i8);
    num_conv_float2int!(f32 => i16);
    num_conv_float2int!(f32 => [crate::MIN_F32_I32, crate::MAX_F32_I32] i32);
    num_conv_float2int!(f32 => [crate::MIN_F32_I64, crate::MAX_F32_I64] i64);

    num_conv_float2int!(f32 => u8);
    num_conv_float2int!(f32 => u16);
    num_conv_float2int!(f32 => [0.0, crate::MAX_F32_U32] u32);
    num_conv_float2int!(f32 => [0.0, crate::MAX_F32_U64] u64);

    num_conv_float2int!(f32 => [crate::MIN_F32_ISIZE, crate::MAX_F32_ISIZE] isize);
    num_conv_float2int!(f32 => [0.0, crate::MAX_F32_USIZE] usize);

    num_conv_float2int!(f64 => i8);
    num_conv_float2int!(f64 => i16);
    num_conv_float2int!(f64 => i32);
    num_conv_float2int!(f64 => [crate::MIN_F64_I64, crate::MAX_F64_I64] i64);

    num_conv_float2int!(f64 => u8);
    num_conv_float2int!(f64 => u16);
    num_conv_float2int!(f64 => u32);
    num_conv_float2int!(f64 => [0.0, crate::MAX_F64_U64] u64);

    #[cfg(target_pointer_width = "32")]
    mod size32 {
        num_conv_float2int!(f64 => isize);
        num_conv_float2int!(f64 => usize);
    }

    #[cfg(target_pointer_width = "64")]
    mod size64 {
        num_conv_float2int!(f64 => [crate::MIN_F64_I64, crate::MAX_F64_I64] isize);
        num_conv_float2int!(f64 => [0.0, crate::MAX_F64_U64] usize);
    }
}
