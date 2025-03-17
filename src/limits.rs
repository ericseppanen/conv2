// Some floating-point-to-integer conversions start to lose precision before they
// reach the min/max values in the destination type. These values represent the
// safety limits of `to_int_unchecked`.

/// Maximum `f32` that can be represented in an `i32`.
#[doc(hidden)]
pub const MAX_F32_I32: f32 = 2.1474835e9;
/// Minimum `f32` that can be represented in an `i32`.
#[doc(hidden)]
pub const MIN_F32_I32: f32 = -2.1474836e9;
/// Maximum `f32` that can be represented in an `i64`.
#[doc(hidden)]
pub const MAX_F32_I64: f32 = 9.2233715e18;
/// Minimum `f32` that can be represented in an `i64`.
#[doc(hidden)]
pub const MIN_F32_I64: f32 = -9.223372e18;
/// Maximum `f32` that can be represented in a `u32`.
#[doc(hidden)]
pub const MAX_F32_U32: f32 = 4.294967e9;
/// Maximum `f32` that can be represented in a `u64`.
#[doc(hidden)]
pub const MAX_F32_U64: f32 = 1.8446743e19;

#[cfg(target_pointer_width = "32")]
/// Maximum `f32` that can be represented in an `isize`.
#[doc(hidden)]
pub const MAX_F32_ISIZE: f32 = MAX_F32_I32;
#[cfg(target_pointer_width = "32")]
/// Minimum `f32` that can be represented in an `isize`.
#[doc(hidden)]
pub const MIN_F32_ISIZE: f32 = MIN_F32_I32;
#[cfg(target_pointer_width = "32")]
/// Maximum `f32` that can be represented in a `usize`.
#[doc(hidden)]
pub const MAX_F32_USIZE: f32 = MAX_F32_U32;

#[cfg(target_pointer_width = "64")]
/// Maximum `f32` that can be represented in an `isize`.
#[doc(hidden)]
pub const MAX_F32_ISIZE: f32 = MAX_F32_I64;
#[cfg(target_pointer_width = "64")]
/// Minimum `f32` that can be represented in an `isize`.
#[doc(hidden)]
pub const MIN_F32_ISIZE: f32 = MIN_F32_I64;
#[cfg(target_pointer_width = "64")]
/// Maximum `f32` that can be represented in a `usize`.
#[doc(hidden)]
pub const MAX_F32_USIZE: f32 = MAX_F32_U64;

/// Maximum `f64` that can be represented in an `i64`.
#[doc(hidden)]
pub const MAX_F64_I64: f64 = 9.223372036854775e18;
/// Minimum `f32` that can be represented in an `i64`.
#[doc(hidden)]
pub const MIN_F64_I64: f64 = -9.223372036854776e18;
/// Maximum `f64` that can be represented in an `u64`.
#[doc(hidden)]
pub const MAX_F64_U64: f64 = 1.844674407370955e19;
