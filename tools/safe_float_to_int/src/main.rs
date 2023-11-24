//! Experimentally derives, for each (int, float) pair, the largest and smallest integer values that survive round-tripping through the float types.
//!
//! This is to verify *exactly* what the safe range for float-to-int conversions is.

use ieee754::Ieee754;
use std::fmt;

macro_rules! limits {
    ($src:ty => $dst:ident; < $min:expr, > $max:expr) => {
        limits!($src => $dst; < $min);
        limits!($src => $dst; > $max);
    };
    ($src:ty => $dst:ident; > $max:expr) => {
        {
            let mut cur: $src = $max;
            if ((cur as $dst) as $src) != cur {
                panic!("safe {} max: not found; initial limit too high!", stringify!($src => $dst));
            }
            loop {
                let next = (cur + 1.0).max(cur.next());
                let next_int = next as $dst;
                let next_rnd = next_int as $src;
                if next_rnd != next {
                    println!("safe {} max: {}, {:e}, {:+x}",
                        stringify!($src => $dst), cur, cur, FloatHex(cur));
                    break;
                } else {
                    cur = next;
                }
            }
        }
    };
    ($src:ty => $dst:ident; < $min:expr) => {
        {
            let mut cur: $src = $min;
            if ((cur as $dst) as $src) != cur {
                panic!("safe {} min: not found; initial limit too low!", stringify!($src => $dst));
            }
            loop {
                let next = (cur - 1.0).min(cur.prev());
                let next_int = next as $dst;
                let next_rnd = next_int as $src;
                if next_rnd != next {
                    println!("safe {} min: {:+}, {:+e}, {:+x}",
                        stringify!($src => $dst), cur, cur, FloatHex(cur));
                    break;
                } else {
                    cur = next;
                }
            }
        }
    };
}

fn main() {
    limits!(f32 => i8; < -120.0, > 120.0);
    limits!(f32 => i16; < -32000.0, > 32000.0);
    limits!(f32 => i32; < -2147480000.0, > 2147480000.0);
    limits!(f32 => i64; < -9223300000000000000.0, > 9223300000000000000.0);
    limits!(f32 => u8; > 250.0);
    limits!(f32 => u16; > 64000.0);
    limits!(f32 => u32; > 4290000000.0);
    limits!(f32 => u64; > 18446700000000000000.0);

    limits!(f64 => i8; < -120.0, > 120.0);
    limits!(f64 => i16; < -32000.0, > 32000.0);
    limits!(f64 => i32; < -2147480000.0, > 2147480000.0);
    limits!(f64 => i64; < -9223372036854770000.0, > 9223372036854700000.0);
    limits!(f64 => u8; > 250.0);
    limits!(f64 => u16; > 64000.0);
    limits!(f64 => u32; > 4290000000.0);
    limits!(f64 => u64; > 18446744073709500000.0);
}

fn format_hex_parts(
    f: &mut fmt::Formatter,
    leading: char,
    sig: impl Into<u64>,
    exp: impl Into<i64>,
) -> fmt::Result {
    let sig = sig.into();
    let exp = exp.into();
    let sig_fmt = format!("{:x}", sig);
    let mut sig_fmt = sig_fmt.trim_end_matches('0');
    if sig_fmt.is_empty() {
        sig_fmt = "0";
    }
    write!(f, "0x{leading}.{sig_fmt}p{exp}")
}
struct FloatHex<T>(pub T);

impl fmt::LowerHex for FloatHex<f32> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::num::FpCategory;

        let v = self.0;

        match v.classify() {
            FpCategory::Nan => {
                fmt.write_str("nan")?;
            }
            FpCategory::Infinite => {
                if v.is_sign_negative() {
                    fmt.write_str("-")?;
                } else if fmt.sign_plus() {
                    fmt.write_str("+")?;
                }
                fmt.write_str("infinity")?;
            }
            FpCategory::Zero => {
                if v.is_sign_negative() {
                    fmt.write_str("-")?;
                } else if fmt.sign_plus() {
                    fmt.write_str("+")?;
                }
                fmt.write_str("0x0p0")?;
            }
            FpCategory::Subnormal => {
                let (neg, exp, sig) = v.decompose();
                if neg {
                    fmt.write_str("-")?;
                } else if fmt.sign_plus() {
                    fmt.write_str("+")?;
                }
                format_hex_parts(fmt, '0', sig << 1, exp)?;
            }
            FpCategory::Normal => {
                let (neg, exp, sig) = v.decompose();
                if neg {
                    fmt.write_str("-")?;
                } else if fmt.sign_plus() {
                    fmt.write_str("+")?;
                }
                format_hex_parts(fmt, '1', sig << 1, exp)?;
            }
        }

        Ok(())
    }
}

impl fmt::LowerHex for FloatHex<f64> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::num::FpCategory;

        let v = self.0;

        match v.classify() {
            FpCategory::Nan => {
                fmt.write_str("nan")?;
            }
            FpCategory::Infinite => {
                if v.is_sign_negative() {
                    fmt.write_str("-")?;
                } else if fmt.sign_plus() {
                    fmt.write_str("+")?;
                }
                fmt.write_str("infinity")?;
            }
            FpCategory::Zero => {
                if v.is_sign_negative() {
                    fmt.write_str("-")?;
                } else if fmt.sign_plus() {
                    fmt.write_str("+")?;
                }
                fmt.write_str("0x0p0")?;
            }
            FpCategory::Subnormal => {
                let (neg, exp, sig) = v.decompose();
                if neg {
                    fmt.write_str("-")?;
                } else if fmt.sign_plus() {
                    fmt.write_str("+")?;
                }
                format_hex_parts(fmt, '0', sig << 1, exp)?;
            }
            FpCategory::Normal => {
                let (neg, exp, sig) = v.decompose();
                if neg {
                    fmt.write_str("-")?;
                } else if fmt.sign_plus() {
                    fmt.write_str("+")?;
                }
                format_hex_parts(fmt, '1', sig << 1, exp)?;
            }
        }

        Ok(())
    }
}
