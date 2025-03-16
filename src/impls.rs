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

macro_rules! impl_float2int_round {
    // A fallible float->int conversion, with an explicit rounding step.
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
            impl_float2int_round! { $src, $dst, [$min, $max], crate::DefaultApprox, approx: |s| s }
            impl_float2int_round! { $src, $dst, [$min, $max], crate::RoundToNearest, approx: |s| s.round() }
            impl_float2int_round! { $src, $dst, [$min, $max], crate::RoundToNegInf, approx: |s| s.floor() }
            impl_float2int_round! { $src, $dst, [$min, $max], crate::RoundToPosInf, approx: |s| s.ceil() }
            impl_float2int_round! { $src, $dst, [$min, $max], crate::RoundToZero, approx: |s| s.trunc() }
        }
    };

    // Convert float->int, using the destination type's MIN and MAX as the allowed range.
    ($src:ty => $dst:ident) => {
        as_item! {
            impl_float2int_round! { $src, $dst, [$dst::MIN as $src, $dst::MAX as $src], crate::DefaultApprox, approx: |s| s }
            impl_float2int_round! { $src, $dst, [$dst::MIN as $src, $dst::MAX as $src], crate::RoundToNearest, approx: |s| s.round() }
            impl_float2int_round! { $src, $dst, [$dst::MIN as $src, $dst::MAX as $src], crate::RoundToNegInf, approx: |s| s.floor() }
            impl_float2int_round! { $src, $dst, [$dst::MIN as $src, $dst::MAX as $src], crate::RoundToPosInf, approx: |s| s.ceil() }
            impl_float2int_round! { $src, $dst, [$dst::MIN as $src, $dst::MAX as $src], crate::RoundToZero, approx: |s| s.trunc() }
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
    // This could lead to incorrect results.

    num_conv_float2int!(f32 => i8);
    num_conv_float2int!(f32 => i16);
    num_conv_float2int!(f32 => [-2.1474836e9, 2.1474835e9] i32);
    num_conv_float2int!(f32 => [-9.223372e18, 9.2233715e18] i64);

    num_conv_float2int!(f32 => u8);
    num_conv_float2int!(f32 => u16);
    num_conv_float2int!(f32 => [0.0, 4.294967e9] u32);
    num_conv_float2int!(f32 => [0.0, 1.8446743e19] u64);

    #[cfg(target_pointer_width = "32")]
    mod size32 {
        num_conv_float2int!(f32 => [-2.1474836e9, 2.1474835e9] isize);
        num_conv_float2int!(f32 => [0.0, 4.294967e9] usize);
    }

    #[cfg(target_pointer_width = "64")]
    mod size64 {
        num_conv_float2int!(f32 => [-9.223372e18, 9.2233715e18] isize);
        num_conv_float2int!(f32 => [0.0, 1.8446743e19] usize);
    }

    num_conv_float2int!(f64 => i8);
    num_conv_float2int!(f64 => i16);
    num_conv_float2int!(f64 => i32);
    num_conv_float2int!(f64 => [-9.223372036854776e18, 9.223372036854775e18] i64);

    num_conv_float2int!(f64 => u8);
    num_conv_float2int!(f64 => u16);
    num_conv_float2int!(f64 => u32);
    num_conv_float2int!(f64 => [0.0, 1.844674407370955e19] u64);

    // FIXME: merge the size-width modules
    #[cfg(target_pointer_width = "32")]
    mod size32b {
        num_conv_float2int!(f64 => isize);
        num_conv_float2int!(f64 => usize);
    }

    #[cfg(target_pointer_width = "64")]
    mod size64b {
        num_conv_float2int!(f64 => [-9.223372036854776e18, 9.223372036854775e18] isize);
        num_conv_float2int!(f64 => [0.0, 1.844674407370955e19] usize);
    }
}
