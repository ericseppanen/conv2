extern crate conv;

#[macro_use] mod util;

use conv::*;

use conv::Underflow as Uf;
use conv::Overflow as Of;
use conv::RangeError::Underflow as RU;
use conv::RangeError::Overflow as RO;

#[test]
fn test_i8() {
    check!(i8, i8; sident; qv: *;
    );
    check!(i8, i16; sident; qv: *;
    );
    check!(i8, i32; sident; qv: *;
    );
    check!(i8, i64; sident; qv: *;
    );
    check!(i8, u8; uident; qv: +;
        v: -1, !Uf;
    );
    check!(i8, u16; uident; qv: +;
        v: -1, !Uf;
    );
    check!(i8, u32; uident; qv: +;
        v: -1, !Uf;
    );
    check!(i8, u64; uident; qv: +;
        v: -1, !Uf;
    );
    check!(i8, isize; sident; qv: *;
    );
    check!(i8, usize; uident; qv: +;
        v: -1, !Uf;
    );
}

#[test]
fn test_i16() {
    check!(i16, i8; sident; qv: i8;
        v: -129, !RU; v: 128, !RO;
    );
    check!(i16, i16; sident; qv: *;
    );
    check!(i16, i32; sident; qv: *;
    );
    check!(i16, i64; sident; qv: *;
    );
    check!(i16, u8; uident; qv: u8;
        v: -1, !RU;
    );
    check!(i16, u16; uident; qv: u16, i16;
        v: -1, !Uf;
    );
    check!(i16, u32; uident; qv: +;
        v: -1, !Uf;
    );
    check!(i16, u64; uident; qv: +;
        v: -1, !Uf;
    );
    check!(i16, isize; sident; qv: *;
    );
    check!(i16, usize; uident; qv: +;
        v: -1, !Uf;
    );
}

#[test]
fn test_i32() {
    check!(i32, i8; sident; qv: i8;
        v: -129, !RU; v: 128, !RO;
    );
    check!(i32, i16; sident; qv: i16;
        v: -32_769, !RU; v: 32_768, !RO;
    );
    check!(i32, i32; sident; qv: *;
    );
    check!(i32, i64; sident; qv: *;
    );
    check!(i32, u8; uident; qv: u8;
        v: -1, !RU;
    );
    check!(i32, u16; uident; qv: u16;
        v: -1, !RU;
    );
    check!(i32, u32; uident; qv: +;
        v: -1, !Uf;
    );
    check!(i32, u64; uident; qv: +;
        v: -1, !Uf;
    );
    if cfg!(target_pointer_width = "32") {
        check!(i32, isize; sident; qv: *;
        );
        check!(i32, usize; uident; qv: +;
            v: -1, !Uf;
        );
    } else {
        unimplemented!()
    }
}

#[test]
fn test_i64() {
    check!(i64, i8; sident; qv: i8;
        v: -129, !RU; v: 128, !RO;
    );
    check!(i64, i16; sident; qv: i16;
        v: -32_769, !RU; v: 32_768, !RO;
    );
    check!(i64, i32; sident; qv: i32;
        v: -2_147_483_649, !RU; v: 2_147_483_648, !RO;
    );
    check!(i64, i64; sident; qv: *;
    );
    check!(i64, u8; uident; qv: u8;
        v: -1, !RU;
    );
    check!(i64, u16; uident; qv: u16;
        v: -1, !RU;
    );
    check!(i64, u32; uident; qv: u32;
        v: -1, !RU;
    );
    check!(i64, u64; uident; qv: +;
        v: -1, !Uf;
    );
    if cfg!(target_pointer_width = "32") {
        check!(i64, isize; sident; qv: isize;
            v: -2_147_483_649, !RU; v: 2_147_483_648, !RO;
        );
        check!(i64, usize; uident; qv: usize;
            v: -1, !RU; v: 4_294_967_296, !RO;
        );
    } else {
        unimplemented!();
    }
}

#[test]
fn test_u8() {
    check!(u8, i8; uident; qv: +i8;
        v: 127; v: 128, !Of;
    );
    check!(u8, i16; uident; qv: *;
    );
    check!(u8, i32; uident; qv: *;
    );
    check!(u8, i64; uident; qv: *;
    );
    check!(u8, u8; uident; qv: *;
    );
    check!(u8, u16; uident; qv: *;
    );
    check!(u8, u32; uident; qv: *;
    );
    check!(u8, u64; uident; qv: *;
    );
    check!(u8, isize; uident; qv: *;
    );
    check!(u8, usize; uident; qv: *;
    );
}

#[test]
fn test_u16() {
    check!(u16, i8; uident; qv: +i8;
        v: 128, !Of;
    );
    check!(u16, i16; uident; qv: +i16;
        v: 32_768, !Of;
    );
    check!(u16, i32; uident; qv: *;
    );
    check!(u16, i64; uident; qv: *;
    );
    check!(u16, u8; uident; qv: u8;
        v: 256, !Of;
    );
    check!(u16, u16; uident; qv: *;
    );
    check!(u16, u32; uident; qv: *;
    );
    check!(u16, u64; uident; qv: *;
    );
    check!(u16, isize; uident; qv: *;
    );
    check!(u16, usize; uident; qv: *;
    );
}

#[test]
fn test_u32() {
    check!(u32, i8; uident; qv: +i8;
        v: 128, !Of;
    );
    check!(u32, i16; uident; qv: +i16;
        v: 32_768, !Of;
    );
    check!(u32, i32; uident; qv: +i32;
        v: 2_147_483_648, !Of;
    );
    check!(u32, i64; uident; qv: *;
    );
    check!(u32, u8; uident; qv: u8;
        v: 256, !Of;
    );
    check!(u32, u16; uident; qv: u16;
        v: 65_536, !Of;
    );
    check!(u32, u32; uident; qv: *;
    );
    check!(u32, u64; uident; qv: *;
    );
    if cfg!(target_pointer_width = "32") {
        check!(u32, isize; uident; qv: +isize;
            v: 2_147_483_647; v: 2_147_483_648, !Of;
        );
        check!(u32, usize; uident; qv: *;
        );
    } else {
        unimplemented!()
    }
}

#[test]
fn test_u64() {
    check!(u64, i8; uident; qv: +i8;
        v: 128, !Of;
    );
    check!(u64, i16; uident; qv: +i16;
        v: 32_768, !Of;
    );
    check!(u64, i32; uident; qv: +i32;
        v: 2_147_483_648, !Of;
    );
    check!(u64, i64; uident; qv: +i64;
        v: 9_223_372_036_854_775_808, !Of;
    );
    check!(u64, u8; uident; qv: u8;
        v: 256, !Of;
    );
    check!(u64, u16; uident; qv: u16;
        v: 65_536, !Of;
    );
    check!(u64, u32; uident; qv: u32;
        v: 4_294_967_296, !Of;
    );
    check!(u64, u64; uident; qv: *;
    );
    if cfg!(target_pointer_width = "32") {
        check!(u64, isize; uident; qv: +isize;
            v: 2_147_483_648, !Of;
        );
        check!(u64, usize; uident; qv: usize;
            v: 4_294_967_296, !Of;
        );
    } else {
        unimplemented!()
    }
}

#[test]
fn test_isize() {
    check!(isize, i8; sident; qv: i8;
        v: -129, !RU; v: 128, !RO;
    );
    check!(isize, i16; sident; qv: i16;
        v: -32_769, !RU; v: 32_768, !RO;
    );
    check!(isize, u8; uident; qv: u8;
        v: -1, !RU; v: 256, !RO;
    );
    check!(isize, u16; uident; qv: u16;
        v: -1, !RU; v: 65_536, !RO;
    );
    check!(isize, isize; sident; qv: *;
    );
    if cfg!(target_pointer_width = "32") {
        check!(isize, i32; sident; qv: *;
        );
        check!(isize, i64; sident; qv: *;
        );
        check!(isize, u32; uident; qv: +;
            v: -1, !Uf;
        );
        check!(isize, u64; uident; qv: +;
            v: -1, !Uf;
        );
        check!(isize, usize; uident; qv: +;
            v: -1, !Uf;
        );
    } else {
        unimplemented!()
    }
}

#[test]
fn test_usize() {
    check!(usize, i8; uident; qv: +i8;
        v: 128, !Of;
    );
    check!(usize, i16; uident; qv: +i16;
        v: 32_768, !Of;
    );
    check!(usize, u8; uident; qv: u8;
        v: 256, !Of;
    );
    check!(usize, u16; uident; qv: u16;
        v: 65_536, !Of;
    );
    check!(usize, usize; uident; qv: *;
    );
    if cfg!(target_pointer_width = "32") {
        check!(usize, i32; uident; qv: +i32;
        );
        check!(usize, i64; uident; qv: *;
        );
        check!(usize, u32; uident; qv: *;
        );
        check!(usize, u64; uident; qv: *;
        );
        check!(usize, isize; uident; qv: +isize;
        );
    } else {
        unimplemented!()
    }
}