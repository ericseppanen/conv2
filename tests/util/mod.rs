#![allow(unused_macros)]

/// Detects floating point NaN; which is not equal to itself.
#[cfg(test)]
#[allow(dead_code)] // This module is a submodule of multiple test modules.
pub fn is_nan<T>(val: T) -> bool
where
    T: PartialEq,
{
    val != val
}

/// Detects floating point +/- infinity by checking if `value` is greater than T::MAX or less than T::MIN.
macro_rules! is_infinite {
    ($value:expr, $type:ty) => {{
        let v = $value;
        (v > <$type>::MAX) || (v < <$type>::MIN)
    }};
}

macro_rules! SL {
    ($($tts:tt)*) => { stringify!($($tts)*) };
}

macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}

macro_rules! check {
    (@ $from:ty, $to:ty=> $(;)*) => {};

    // To unsigned int
    (@ $from:ty, $to:ty=> uident; $($tail:tt)*) => {
        check!(@ $from, $to=> v: 0;);
        check!(@ $from, $to=> v: 1;);
        check!(@ $from, $to=> $($tail)*);
    };

    // To signed int
    (@ $from:ty, $to:ty=> sident; $($tail:tt)*) => {
        check!(@ $from, $to=> v: -1;);
        check!(@ $from, $to=> v: 0;);
        check!(@ $from, $to=> v: 1;);
        check!(@ $from, $to=> $($tail)*);
    };

    // To floating point
    (@ $from:ty, $to:ty=> fident; $($tail:tt)*) => {
        check!(@ $from, $to=> v: -1.0;);
        check!(@ $from, $to=> v:  0.0;);
        check!(@ $from, $to=> v:  1.0;);
        check!(@ $from, $to=> $($tail)*);
    };

    // To unsigned int (approx), with rounding checks
    (@ $from:ty, $to:ty=> uidenta; $($tail:tt)*) => {
        check!(@ $from, $to=> a: 0.0;);
        check!(@ $from, $to=> a: 1.0;);

        check!(@ $from, $to=> aRTN: 0.00, 0;);
        check!(@ $from, $to=> aRTN: 0.25, 0;);
        check!(@ $from, $to=> aRTN: 0.50, 1;);
        check!(@ $from, $to=> aRTN: 0.75, 1;);
        check!(@ $from, $to=> aRTN: 1.00, 1;);

        check!(@ $from, $to=> aRNI:  0.00,  0;);
        check!(@ $from, $to=> aRNI:  0.25,  0;);
        check!(@ $from, $to=> aRNI:  0.50,  0;);
        check!(@ $from, $to=> aRNI:  0.75,  0;);
        check!(@ $from, $to=> aRNI:  1.00,  1;);

        check!(@ $from, $to=> aRPI:  0.00,  0;);
        check!(@ $from, $to=> aRPI:  0.25,  1;);
        check!(@ $from, $to=> aRPI:  0.50,  1;);
        check!(@ $from, $to=> aRPI:  0.75,  1;);
        check!(@ $from, $to=> aRPI:  1.00,  1;);

        check!(@ $from, $to=> aRTZ:  0.00,  0;);
        check!(@ $from, $to=> aRTZ:  0.25,  0;);
        check!(@ $from, $to=> aRTZ:  0.50,  0;);
        check!(@ $from, $to=> aRTZ:  0.75,  0;);
        check!(@ $from, $to=> aRTZ:  1.00,  1;);

        check!(@ $from, $to=> $($tail)*);
    };

    // To signed int (approx), with rounding checks
    (@ $from:ty, $to:ty=> sidenta; $($tail:tt)*) => {
        check!(@ $from, $to=> a: -1.0;);
        check!(@ $from, $to=> a:  0.0;);
        check!(@ $from, $to=> a:  1.0;);

        check!(@ $from, $to=> aRTN: -1.00, -1;);
        check!(@ $from, $to=> aRTN: -0.75, -1;);
        check!(@ $from, $to=> aRTN: -0.50, -1;);
        check!(@ $from, $to=> aRTN: -0.25,  0;);
        check!(@ $from, $to=> aRTN:  0.00,  0;);
        check!(@ $from, $to=> aRTN:  0.25,  0;);
        check!(@ $from, $to=> aRTN:  0.50,  1;);
        check!(@ $from, $to=> aRTN:  0.75,  1;);
        check!(@ $from, $to=> aRTN:  1.00,  1;);

        check!(@ $from, $to=> aRNI: -1.00, -1;);
        check!(@ $from, $to=> aRNI: -0.75, -1;);
        check!(@ $from, $to=> aRNI: -0.50, -1;);
        check!(@ $from, $to=> aRNI: -0.25, -1;);
        check!(@ $from, $to=> aRNI:  0.00,  0;);
        check!(@ $from, $to=> aRNI:  0.25,  0;);
        check!(@ $from, $to=> aRNI:  0.50,  0;);
        check!(@ $from, $to=> aRNI:  0.75,  0;);
        check!(@ $from, $to=> aRNI:  1.00,  1;);

        check!(@ $from, $to=> aRPI: -1.00, -1;);
        check!(@ $from, $to=> aRPI: -0.75,  0;);
        check!(@ $from, $to=> aRPI: -0.50,  0;);
        check!(@ $from, $to=> aRPI: -0.25,  0;);
        check!(@ $from, $to=> aRPI:  0.00,  0;);
        check!(@ $from, $to=> aRPI:  0.25,  1;);
        check!(@ $from, $to=> aRPI:  0.50,  1;);
        check!(@ $from, $to=> aRPI:  0.75,  1;);
        check!(@ $from, $to=> aRPI:  1.00,  1;);

        check!(@ $from, $to=> aRTZ: -1.00, -1;);
        check!(@ $from, $to=> aRTZ: -0.75,  0;);
        check!(@ $from, $to=> aRTZ: -0.50,  0;);
        check!(@ $from, $to=> aRTZ: -0.25,  0;);
        check!(@ $from, $to=> aRTZ:  0.00,  0;);
        check!(@ $from, $to=> aRTZ:  0.25,  0;);
        check!(@ $from, $to=> aRTZ:  0.50,  0;);
        check!(@ $from, $to=> aRTZ:  0.75,  0;);
        check!(@ $from, $to=> aRTZ:  1.00,  1;);

        check!(@ $from, $to=> $($tail)*);
    };

    // To floating point (approx)
    (@ $from:ty, $to:ty=> fidenta; $($tail:tt)*) => {
        check!(@ $from, $to=> a: -1.0;);
        check!(@ $from, $to=> a:  0.0;);
        check!(@ $from, $to=> a:  1.0;);
        check!(@ $from, $to=> $($tail)*);
    };

    // Single check using `value_into`
    (@ $from:ty, $to:ty=> v: $src:expr, !$dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, v: {}, !{}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.value_into();
            assert_eq!(dst, Err($dst(src)));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Single check using `value_into`; destination value computed using `as`
    (@ $from:ty, $to:ty=> v: $src:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, v: {}", SL!($from), SL!($to), SL!($src));
            let src: $from = $src;
            let dst: Result<$to, _> = src.value_into();
            if util::is_nan(src) {
                assert!(util::is_nan(dst.expect("error but expected NaN")));
            } else {
                assert_eq!(dst, Ok($src as $to));
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `value_into`; destination value computed using `as`
    (@ $from:ty, $to:ty=> qv: *; $($tail:tt)*) => {
        {
            println!("? {} => {}, qv: *", SL!($from), SL!($to));

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.value_into();
                if util::is_nan(v) {
                    util::is_nan(dst)
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv1 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `value_into` (may over/underflow bound); destination value computed using `as`
    (@ $from:ty, $to:ty=> qv: (+-$bound:expr); $($tail:tt)*) => {
        {
            println!("? {} => {}, qv: (+- {})", SL!($from), SL!($to), SL!($bound));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv2::FloatError<_>> = v.value_into().map_err(From::from);
                if !(-$bound as $from <= v) {
                    dst == Err(conv2::FloatError::NegOverflow(v))
                } else if !(v <= $bound as $from) {
                    dst == Err(conv2::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv2 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `value_into` (may overflow bound); destination value computed using `as`
    (@ $from:ty, $to:ty=> qv: (, $bound:expr); $($tail:tt)*) => {
        {
            println!("? {} => {}, qv: (, {})", SL!($from), SL!($to), SL!($bound));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv2::FloatError<_>> = v.value_into().map_err(From::from);
                if !(v <= $bound as $from) {
                    dst == Err(conv2::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv3 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `value_into` (positive only); destination value computed using `as`
    (@ $from:ty, $to:ty=> qv: +; $($tail:tt)*) => {
        {
            println!("? {} => {}, qv: +", SL!($from), SL!($to));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv2::FloatError<_>> = v.value_into().map_err(From::from);
                if !(0 <= v) {
                    dst == Err(conv2::FloatError::NegOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv4 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `value_into` (may overflow); destination value computed using `as`
    (@ $from:ty, $to:ty=> qv: +$max:ty=> $($tail:tt)*) => {
        {
            println!("? {} => {}, qv: +{}", SL!($from), SL!($to), SL!($max));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv2::FloatError<_>> = v.value_into().map_err(From::from);
                if !(v <= <$max>::MAX as $from) {
                    dst == Err(conv2::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv5 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `value_into` (may over/underflow); destination value computed using `as`
    (@ $from:ty, $to:ty=> qv: $bound:ty=> $($tail:tt)*) => {
        {
            println!("? {} => {}, qv: {}", SL!($from), SL!($to), SL!($bound));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv2::FloatError<_>> = v.value_into().map_err(From::from);
                if !(<$bound>::MIN as $from <= v) {
                    dst == Err(conv2::FloatError::NegOverflow(v))
                } else if !(v <= <$bound>::MAX as $from) {
                    dst == Err(conv2::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv6 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `value_into` (may over/underflow); destination value computed using `as`
    (@ $from:ty, $to:ty=> qv: $min:ty, $max:ty=> $($tail:tt)*) => {
        {
            println!("? {} => {}, qv: {}, {}", SL!($from), SL!($to), SL!($min), SL!($max));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv2::FloatError<_>> = v.value_into().map_err(From::from);
                if !(<$min>::MIN as $from <= v) {
                    dst == Err(conv2::FloatError::NegOverflow(v))
                } else if !(v <= <$max>::MAX as $from) {
                    dst == Err(conv2::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qv7 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Single check using `approx_as`; Expects error.
    (@ $from:ty, $to:ty=> a: $src:expr, !$dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, a: {}, !{}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_as();
            assert_eq!(dst, Err($dst(src)));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Single check using `approx_as`.
    (@ $from:ty, $to:ty=> a: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, a: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_as();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Single check using `approx_as`; destination value computed using `as`.
    (@ $from:ty, $to:ty=> a: $src:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, a: {}", SL!($from), SL!($to), SL!($src));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_as();
            assert_eq!(dst, Ok($src as $to));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `approx_as`; destination value computed using `as`
    (@ $from:ty, $to:ty=> qa: *; $($tail:tt)*) => {
        {
            println!("? {} => {}, qa: *", SL!($from), SL!($to));

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.approx_as();
                if util::is_nan(v) {
                    util::is_nan(dst.expect("error but expected NaN"))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa1 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `approx_as` (positive only) destination value computed using `as`.
    (@ $from:ty, $to:ty=> qa: +; $($tail:tt)*) => {
        {
            println!("? {} => {}, qa: +", SL!($from), SL!($to));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv2::FloatError<_>> = v.approx_as().map_err(From::from);
                if !(0 <= v) {
                    dst == Err(conv2::FloatError::NegOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa2 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `approx_as` (may overflow); destination value computed using `as`
    (@ $from:ty, $to:ty=> qa: +$max:ty=> $($tail:tt)*) => {
        {
            println!("? {} => {}, qa: +{}", SL!($from), SL!($to), SL!($max));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv2::FloatError<_>> = v.approx_as().map_err(From::from);
                if !(v <= <$max>::MAX as $from) {
                    dst == Err(conv2::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa3 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `approx_as` (may over/overflow bound); destination value computed using `as`
    (@ $from:ty, $to:ty=> qa: $bound:ty=> $($tail:tt)*) => {
        {
            println!("? {} => {}, qa: {}", SL!($from), SL!($to), SL!($bound));

            fn property(v: $from) -> bool {
                let dst: Result<$to, conv2::FloatError<_>> = v.approx_as().map_err(From::from);
                if util::is_nan(v) {
                    // float NaN -> integer is Err; f64::NAN -> f32::NAN.
                    dst.is_err() || util::is_nan(dst)
                } else if is_infinite!(v, $from) && dst.is_ok() {
                    dst == Ok(v as $to)
                } else if !(<$bound>::MIN as $from <= v) {
                    dst == Err(conv2::FloatError::NegOverflow(v))
                } else if !(v <= <$bound>::MAX as $from) {
                    dst == Err(conv2::FloatError::PosOverflow(v))
                } else {
                    dst == Ok(v as $to)
                }
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qa4 {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    // Quickcheck using `approx_as` with wrapping; destination value computed using `as`
    (@ $from:ty, $to:ty=> qaW: *; $($tail:tt)*) => {
        {
            println!("? {} => {}, qaW: *", SL!($from), SL!($to));

            fn property(v: $from) -> bool {
                let dst: Result<$to, _> = v.approx_as_by::<_, Wrapping>();
                dst == Ok(v as $to)
            }

            let mut qc = quickcheck::QuickCheck::new();
            match qc.quicktest(property as fn($from) -> bool) {
                Ok(_) => (),
                Err(err) => panic!("qaW {err:?}")
            }
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> aRTN: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, aRTN: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_by::<conv2::RoundToNearest>();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> aRNI: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, aRNI: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_by::<conv2::RoundToNegInf>();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> aRPI: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, aRPI: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_by::<conv2::RoundToPosInf>();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    (@ $from:ty, $to:ty=> aRTZ: $src:expr, $dst:expr; $($tail:tt)*) => {
        {
            println!("? {} => {}, aRTZ: {}, {}", SL!($from), SL!($to), SL!($src), SL!($dst));
            let src: $from = $src;
            let dst: Result<$to, _> = src.approx_by::<conv2::RoundToZero>();
            assert_eq!(dst, Ok($dst));
        }
        check!(@ $from, $to=> $($tail)*);
    };

    ($from:ty, $to:ty=> $($tail:tt)*) => {
        check! { @ $from, $to=> $($tail)*; }
    };
}

macro_rules! for_bitness {
    (32 {$($bits32:tt)*} 64 {$($bits64:tt)*}) => {
        as_expr!(
            {
                #[cfg(target_pointer_width="32")]
                fn for_bitness() {
                    $($bits32)*
                }

                #[cfg(target_pointer_width="64")]
                fn for_bitness() {
                    $($bits64)*
                }

                for_bitness()
            }
        )
    };
}
