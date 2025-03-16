//! Experimentally derives, for each (int, float) pair, the largest and
//! smallest integer values that survive round-tripping through the float
//! types.
//!
//! This is to verify *exactly* what the safe range for float-to-int
//! conversions is.

use hexfloat2::HexFloat;
use ieee754::Ieee754;

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
                    println!("safe {} max: {}, {:e}, {:}",
                        stringify!($src => $dst), cur, cur, HexFloat(cur));
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
                    println!("safe {} min: {:+}, {:+e}, {}",
                        stringify!($src => $dst), cur, cur, HexFloat(cur));
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
