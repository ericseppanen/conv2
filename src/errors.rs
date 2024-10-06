//! This module defines the various error types that can be produced by a
//! failed conversion.
//!
//! In addition, it also defines some extension traits to make working with
//! failable conversions more ergonomic (see the `Unwrap*` traits).

use crate::misc::{InvalidSentinel, Saturated, SignedInfinity};
use core::fmt::{self, Debug, Display};

/// A general error enumeration that subsumes all other conversion errors.
///
/// This exists primarily as a "catch-all" for reliably unifying various
/// different kinds of conversion errors.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
pub enum GeneralError<T> {
    /// Input was too negative for the target type.
    #[error("conversion resulted in negative overflow")]
    NegOverflow(T),

    /// Input was too positive for the target type.
    #[error("conversion resulted in positive overflow")]
    PosOverflow(T),

    /// Input was not representable in the target type.
    #[error("could not convert unrepresentable value")]
    Unrepresentable(T),
}

impl<T> GeneralError<T> {
    /// Returns the value stored in this error.
    pub fn into_inner(self) -> T {
        match self {
            GeneralError::NegOverflow(v)
            | GeneralError::PosOverflow(v)
            | GeneralError::Unrepresentable(v) => v,
        }
    }
}

impl<T> From<NoError> for GeneralError<T> {
    fn from(_: NoError) -> Self {
        unreachable!();
    }
}

impl<T> From<Unrepresentable<T>> for GeneralError<T> {
    fn from(e: Unrepresentable<T>) -> Self {
        GeneralError::Unrepresentable(e.0)
    }
}

impl<T> From<NegOverflow<T>> for GeneralError<T> {
    fn from(e: NegOverflow<T>) -> Self {
        GeneralError::NegOverflow(e.0)
    }
}

impl<T> From<PosOverflow<T>> for GeneralError<T> {
    fn from(e: PosOverflow<T>) -> Self {
        GeneralError::PosOverflow(e.0)
    }
}

impl<T> From<RangeError<T>> for GeneralError<T> {
    fn from(e: RangeError<T>) -> Self {
        match e {
            RangeError::NegOverflow(v) => GeneralError::NegOverflow(v),
            RangeError::PosOverflow(v) => GeneralError::PosOverflow(v),
        }
    }
}

impl<T> From<FloatError<T>> for GeneralError<T> {
    fn from(e: FloatError<T>) -> GeneralError<T> {
        use self::FloatError as F;
        use self::GeneralError as G;
        match e {
            F::NegOverflow(v) => G::NegOverflow(v),
            F::PosOverflow(v) => G::PosOverflow(v),
            F::NotANumber(v) => G::Unrepresentable(v),
        }
    }
}

/// A general error enumeration that subsumes all other conversion errors,
/// but discards all input payloads the errors may be carrying.
///
/// This exists primarily as a "catch-all" for reliably unifying various
/// different kinds of conversion errors, and between different input types.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, thiserror::Error)]
pub enum GeneralErrorKind {
    /// Input was too negative for the target type.
    #[error("conversion resulted in negative overflow")]
    NegOverflow,

    /// Input was too positive for the target type.
    #[error("conversion resulted in positive overflow")]
    PosOverflow,

    /// Input was not representable in the target type.
    #[error("could not convert unrepresentable value")]
    Unrepresentable,
}

impl From<NoError> for GeneralErrorKind {
    fn from(_: NoError) -> Self {
        unreachable!();
    }
}

impl<T> From<Unrepresentable<T>> for GeneralErrorKind {
    fn from(_: Unrepresentable<T>) -> Self {
        GeneralErrorKind::Unrepresentable
    }
}

impl<T> From<NegOverflow<T>> for GeneralErrorKind {
    fn from(_: NegOverflow<T>) -> Self {
        GeneralErrorKind::NegOverflow
    }
}

impl<T> From<PosOverflow<T>> for GeneralErrorKind {
    fn from(_: PosOverflow<T>) -> Self {
        GeneralErrorKind::PosOverflow
    }
}

impl From<RangeErrorKind> for GeneralErrorKind {
    fn from(e: RangeErrorKind) -> Self {
        match e {
            RangeErrorKind::NegOverflow => GeneralErrorKind::NegOverflow,
            RangeErrorKind::PosOverflow => GeneralErrorKind::PosOverflow,
        }
    }
}
impl<T> From<RangeError<T>> for GeneralErrorKind {
    fn from(e: RangeError<T>) -> Self {
        match e {
            RangeError::NegOverflow(..) => GeneralErrorKind::NegOverflow,
            RangeError::PosOverflow(..) => GeneralErrorKind::PosOverflow,
        }
    }
}
impl<T> From<GeneralError<T>> for GeneralErrorKind {
    fn from(e: GeneralError<T>) -> Self {
        match e {
            GeneralError::NegOverflow(..) => GeneralErrorKind::NegOverflow,
            GeneralError::PosOverflow(..) => GeneralErrorKind::PosOverflow,
            GeneralError::Unrepresentable(..) => GeneralErrorKind::Unrepresentable,
        }
    }
}

impl<T> From<FloatError<T>> for GeneralErrorKind {
    fn from(e: FloatError<T>) -> GeneralErrorKind {
        use self::FloatError as F;
        use self::GeneralErrorKind as G;
        match e {
            F::NegOverflow(..) => G::NegOverflow,
            F::PosOverflow(..) => G::PosOverflow,
            F::NotANumber(..) => G::Unrepresentable,
        }
    }
}

/// Indicates that it is not possible for the conversion to fail.
///
/// You can use the [`UnwrapOk::unwrap_ok`] method to discard the (statically impossible)
/// `Err` case from a `Result<_, NoError>`, without using `Result::unwrap` (which is
/// typically viewed as a "code smell").
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum NoError {}

impl Display for NoError {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        unreachable!()
    }
}

impl std::error::Error for NoError {}

/// Indicates that the conversion failed because the value was not representable.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
#[error("could not convert unrepresentable value")]
pub struct Unrepresentable<T>(pub T);

impl<T> From<NoError> for Unrepresentable<T> {
    fn from(_: NoError) -> Self {
        unreachable!();
    }
}

/// Indicates that the conversion failed due to a negative overflow.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
#[error("conversion resulted in negative overflow")]
pub struct NegOverflow<T>(pub T);

impl<T> From<NoError> for NegOverflow<T> {
    fn from(_: NoError) -> Self {
        unreachable!();
    }
}

/// Indicates that the conversion failed due to a positive overflow.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
#[error("conversion resulted in positive overflow")]
pub struct PosOverflow<T>(pub T);

impl<T> From<NoError> for PosOverflow<T> {
    fn from(_: NoError) -> Self {
        unreachable!();
    }
}

/// Indicates that a conversion from a floating point type failed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
pub enum FloatError<T> {
    /// Input was too negative for the target type.
    #[error("conversion resulted in negative overflow")]
    NegOverflow(T),

    /// Input was too positive for the target type.
    #[error("conversion resulted in positive overflow")]
    PosOverflow(T),

    /// Input was not-a-number, which the target type could not represent.
    #[error("conversion target does not support not-a-number")]
    NotANumber(T),
}

impl<T> FloatError<T> {
    /// Returns the value stored in this error.
    pub fn into_inner(self) -> T {
        match self {
            FloatError::NegOverflow(v) | FloatError::PosOverflow(v) | FloatError::NotANumber(v) => {
                v
            }
        }
    }
}

impl<T> From<NoError> for FloatError<T> {
    fn from(_: NoError) -> Self {
        unreachable!();
    }
}

impl<T> From<NegOverflow<T>> for FloatError<T> {
    fn from(e: NegOverflow<T>) -> Self {
        FloatError::NegOverflow(e.0)
    }
}

impl<T> From<PosOverflow<T>> for FloatError<T> {
    fn from(e: PosOverflow<T>) -> Self {
        FloatError::PosOverflow(e.0)
    }
}

impl<T> From<RangeError<T>> for FloatError<T> {
    fn from(e: RangeError<T>) -> Self {
        match e {
            RangeError::NegOverflow(v) => FloatError::NegOverflow(v),
            RangeError::PosOverflow(v) => FloatError::PosOverflow(v),
        }
    }
}

/// Indicates that a conversion failed due to a range error.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, thiserror::Error)]
pub enum RangeError<T> {
    /// Input was too negative for the target type.
    #[error("conversion resulted in negative overflow")]
    NegOverflow(T),

    /// Input was too positive the target type.
    #[error("conversion resulted in positive overflow")]
    PosOverflow(T),
}

impl<T> From<NoError> for RangeError<T> {
    fn from(_: NoError) -> Self {
        unreachable!();
    }
}

impl<T> From<NegOverflow<T>> for RangeError<T> {
    fn from(e: NegOverflow<T>) -> Self {
        RangeError::NegOverflow(e.0)
    }
}

impl<T> From<PosOverflow<T>> for RangeError<T> {
    fn from(e: PosOverflow<T>) -> Self {
        RangeError::PosOverflow(e.0)
    }
}

/// Indicates that a conversion failed due to a range error.
///
/// This is a variant of `RangeError` that does not retain the input value
/// which caused the error. It exists to help unify some utility methods
/// and should not generally be used directly, unless you are targeting the
/// `Unwrap*` traits.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, thiserror::Error)]
pub enum RangeErrorKind {
    /// Input was too negative for the target type.
    #[error("conversion resulted in negative overflow")]
    NegOverflow,

    /// Input was too positive the target type.
    #[error("conversion resulted in positive overflow")]
    PosOverflow,
}

impl From<NoError> for RangeErrorKind {
    fn from(_: NoError) -> Self {
        unreachable!();
    }
}

impl<T> From<NegOverflow<T>> for RangeErrorKind {
    fn from(_: NegOverflow<T>) -> Self {
        RangeErrorKind::NegOverflow
    }
}

impl<T> From<PosOverflow<T>> for RangeErrorKind {
    fn from(_: PosOverflow<T>) -> Self {
        RangeErrorKind::PosOverflow
    }
}

impl<T> From<RangeError<T>> for RangeErrorKind {
    fn from(e: RangeError<T>) -> Self {
        match e {
            RangeError::NegOverflow(..) => RangeErrorKind::NegOverflow,
            RangeError::PosOverflow(..) => RangeErrorKind::PosOverflow,
        }
    }
}

/// Saturates a `Result`.
pub trait Saturate {
    /// The result of saturating.
    type Output;

    /// Replaces an overflow error with a saturated value.
    ///
    /// Unlike `unwrap_or_saturate`, this method can be used in cases where the
    /// `Result` error type can encode failures *other* than overflow and
    /// underflow. For example, you cannot saturate a float-to-integer conversion using `unwrap_or_saturate` as the error might be `NotANumber`, which doesn't have a meaningful saturation "direction".
    ///
    /// The output of this method will be a `Result` where the error type *does not* contain overflow conditions. What conditions remain must still be dealt with in some fashion.
    fn saturate(self) -> Self::Output;
}

impl<T, U> Saturate for Result<T, FloatError<U>>
where
    T: Saturated,
{
    type Output = Result<T, Unrepresentable<U>>;

    fn saturate(self) -> Self::Output {
        use self::FloatError::*;
        match self {
            Ok(v) => Ok(v),
            Err(NegOverflow(_)) => Ok(T::saturated_min()),
            Err(PosOverflow(_)) => Ok(T::saturated_max()),
            Err(NotANumber(v)) => Err(Unrepresentable(v)),
        }
    }
}

impl<T, U> Saturate for Result<T, RangeError<U>>
where
    T: Saturated,
{
    type Output = Result<T, NoError>;

    fn saturate(self) -> Self::Output {
        use self::RangeError::*;
        match self {
            Ok(v) => Ok(v),
            Err(NegOverflow(_)) => Ok(T::saturated_min()),
            Err(PosOverflow(_)) => Ok(T::saturated_max()),
        }
    }
}

impl<T> Saturate for Result<T, RangeErrorKind>
where
    T: Saturated,
{
    type Output = Result<T, NoError>;

    fn saturate(self) -> Self::Output {
        use self::RangeErrorKind::*;
        match self {
            Ok(v) => Ok(v),
            Err(NegOverflow) => Ok(T::saturated_min()),
            Err(PosOverflow) => Ok(T::saturated_max()),
        }
    }
}

/// Safely unwrap a `Result` that cannot contain an error.
pub trait UnwrapOk<T> {
    /// Unwraps a `Result` without possibility of failing.
    ///
    /// Technically, this is not necessary; it's provided simply to make user
    /// code a little clearer.
    fn unwrap_ok(self) -> T;
}

impl<T> UnwrapOk<T> for Result<T, NoError> {
    fn unwrap_ok(self) -> T {
        match self {
            Ok(v) => v,
            Err(no_error) => match no_error {},
        }
    }
}

/// Unwrap a conversion by saturating to infinity.
pub trait UnwrapOrInf {
    /// The result of unwrapping.
    type Output;

    /// Either unwraps the successfully converted value, or saturates to
    /// infinity in the "direction" of overflow.
    fn unwrap_or_inf(self) -> Self::Output;
}

/// Unwrap a conversion by replacing a failure with an invalid sentinel value.
pub trait UnwrapOrInvalid {
    /// The result of unwrapping.
    type Output;

    /// Either unwraps the successfully converted value, or returns the output
    /// type's invalid sentinel value.
    fn unwrap_or_invalid(self) -> Self::Output;
}

/// Unwrap a conversion by saturating.
pub trait UnwrapOrSaturate {
    /// The result of unwrapping.
    type Output;

    /// Either unwraps the successfully converted value, or saturates in the
    /// "direction" of overflow.
    fn unwrap_or_saturate(self) -> Self::Output;
}

impl<T, E> UnwrapOrInf for Result<T, E>
where
    T: SignedInfinity,
    E: Into<RangeErrorKind>,
{
    type Output = T;
    fn unwrap_or_inf(self) -> T {
        use self::RangeErrorKind::*;
        match self.map_err(Into::into) {
            Ok(v) => v,
            Err(NegOverflow) => T::neg_infinity(),
            Err(PosOverflow) => T::pos_infinity(),
        }
    }
}

impl<T, E> UnwrapOrInvalid for Result<T, E>
where
    T: InvalidSentinel,
{
    type Output = T;
    fn unwrap_or_invalid(self) -> T {
        match self {
            Ok(v) => v,
            Err(..) => T::invalid_sentinel(),
        }
    }
}

impl<T, E> UnwrapOrSaturate for Result<T, E>
where
    T: Saturated,
    E: Into<RangeErrorKind>,
{
    type Output = T;
    fn unwrap_or_saturate(self) -> T {
        use self::RangeErrorKind::*;
        match self.map_err(Into::into) {
            Ok(v) => v,
            Err(NegOverflow) => T::saturated_min(),
            Err(PosOverflow) => T::saturated_max(),
        }
    }
}
