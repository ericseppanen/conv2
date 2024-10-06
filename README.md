
`conv2` is a fork of [`conv`](https://github.com/DanielKeep/rust-conv/),
written by Daniel Keep. It's in the progress of being updated to modern
idiomatic Rust.

<!-- cargo-sync-readme start -->


This crate provides a number of conversion traits with more specific
semantics than those provided by `as` or `From`/`Into`.

The goal with the traits provided here is to be more specific about what
generic code can rely on, as well as provide reasonably self-describing
alternatives to the standard `From`/`Into` traits. For example, the
although `T: From<U>` might be satisfied, it imposes no restrictions on
the *kind* of conversion being implemented. As such, the traits in this
crate try to be very specific about what conversions are allowed. This
makes them less generally applicable, but more useful where they *do*
apply.

In addition, `From`/`Into` requires all conversions to succeed or panic.
All conversion traits in this crate define an associated error type,
allowing code to react to failed conversions as appropriate.

## Compatibility

`conv2` is compatible with Rust 1.61 and higher.

# Overview

The following traits are used to define various conversion semantics:

- [`ApproxFrom`] - approximate conversions, with selectable approximation
  scheme (see [`ApproxScheme`]).
- [`ValueFrom`] - exact, value-preserving conversions.

When *defining* a conversion, try to implement the `*From` trait variant
where possible. When *using* a conversion, try to depend on the `*Into`
trait variant where possible. This is because the `*Into` traits
automatically use `*From` implementations, but not the reverse.
Implementing `*From` and using `*Into` ensures conversions work in as many
contexts as possible.

These extension methods are provided to help with some common cases:

- [`ConvUtil::approx_as`] - approximates to `Dst` with the `DefaultApprox`
  scheme.
- [`ConvUtil::approx_as_by`] - approximates to `Dst` with the scheme `S`.
- [`ConvUtil::into_as<Dst>`] - converts to `Dst` using `Into::into`.
- [`ConvUtil::try_as<Dst>`] - converts to `Dst` using `TryInto::try_into`.
- [`ConvUtil::value_as<Dst>`] - converts to `Dst` using
  `ValueInto::value_into`.
- [`ConvAsUtil::approx`] - approximates to an inferred destination type
  with the `DefaultApprox` scheme.
- [`ConvAsUtil::approx_by`] - approximates to an inferred destination type
  with the scheme `S`.
- [`Saturate::saturate`]- saturates on overflow.
- [`UnwrapOk::unwrap_ok`] - unwraps results from conversions that cannot
  fail.
- [`UnwrapOrInf::unwrap_or_inf`] - saturates to ±∞ on failure.
- [`UnwrapOrInvalid::unwrap_or_invalid`] - substitutes the target type's
  "invalid" sentinel value on failure.
- [`UnwrapOrSaturate::unwrap_or_saturate`] - saturates to the maximum or
  minimum value of the target type on failure.

## Provided Implementations

The crate provides several blanket implementations:

- `*From<A> for A` (all types can be converted from and into themselves).
- `*Into<Dst> for Src where Dst: *From<Src>` (`*From` implementations imply
  a matching `*Into` implementation).

Conversions for the builtin numeric (integer and floating point) types are
provided. In general, `ValueFrom` conversions exist for all pairs except
for float → integer (since such a conversion is generally unlikely to
*exactly* succeed) and `f64 → f32` (for the same reason). `ApproxFrom`
conversions with the `DefaultApprox` scheme exist between all pairs.
`ApproxFrom` with the `Wrapping` scheme exist between integers.

## Errors

A number of error types are defined in the [`errors`] module. Generally,
conversions use whichever error type most *narrowly* defines the kinds of
failures that can occur. For example:

- `ValueFrom<u8> for u16` cannot possibly fail, and as such it uses
  `NoError`.
- `ValueFrom<i8> for u16` can *only* fail with a negative overflow, thus it
  uses the `NegOverflow` type.
- `ValueFrom<i32> for u16` can overflow in either direction, hence it uses
  `RangeError`.
- Finally, `ApproxFrom<f32> for u16` can overflow (positive or negative),
  or attempt to convert NaN; `FloatError` covers those three cases.

Because there are *numerous* error types, the `GeneralError` enum is
provided. `From<E, T> for GeneralError<T>` exists for each error type
`E<T>` defined by this crate (even for `NoError`!), allowing errors to be
translated automatically by the `?` operator. In fact, all errors can be
"expanded" to *all* more general forms (*e.g.* `NoError` → `NegOverflow`,
`PosOverflow` → `RangeError` → `FloatError`).

Aside from `NoError`, the various error types wrap the input value that you
attempted to convert. This is so that non-`Copy` types do not need to be
pre-emptively cloned prior to conversion, just in case the conversion
fails. A downside is that this means there are many, *many* incompatible
error types.

To help alleviate this, there is also `GeneralErrorKind`, which is simply
`GeneralError<T>` without the payload, and all errors can be converted
into it directly.

The reason for not just using `GeneralErrorKind` in the first place is to
statically reduce the number of potential error cases you need to deal
with. It also allows the `Unwrap*` extension traits to be defined *without*
the possibility for runtime failure (*e.g.* you cannot use
`unwrap_or_saturate` with a `FloatError`, because what do you do if the
error is `NotANumber`; saturate to max or to min?  Or panic?).

# Examples

```rust

// This *cannot* fail, so we can use `unwrap_ok` to discard the `Result`.
assert_eq!(u8::value_from(0u8).unwrap_ok(), 0u8);

// This *can* fail. Specifically, it can overflow toward negative infinity.
assert_eq!(u8::value_from(0i8),     Ok(0u8));
assert_eq!(u8::value_from(-1i8),    Err(NegOverflow(-1)));

// This can overflow in *either* direction; hence the change to `RangeError`.
assert_eq!(u8::value_from(-1i16),   Err(RangeError::NegOverflow(-1)));
assert_eq!(u8::value_from(0i16),    Ok(0u8));
assert_eq!(u8::value_from(256i16),  Err(RangeError::PosOverflow(256)));

// We can use the extension traits to simplify this a little.
assert_eq!(u8::value_from(-1i16).unwrap_or_saturate(),  0u8);
assert_eq!(u8::value_from(0i16).unwrap_or_saturate(),   0u8);
assert_eq!(u8::value_from(256i16).unwrap_or_saturate(), 255u8);

// Obviously, all integers can be "approximated" using the default scheme (it
// doesn't *do* anything), but they can *also* be approximated with the
// `Wrapping` scheme.
assert_eq!(
    <u8 as ApproxFrom<_, DefaultApprox>>::approx_from(400u16),
    Err(PosOverflow(400)));
assert_eq!(
    <u8 as ApproxFrom<_, Wrapping>>::approx_from(400u16),
    Ok(144u8));

// This is rather inconvenient; as such, there are a number of convenience
// extension methods available via `ConvUtil` and `ConvAsUtil`.
assert_eq!(400u16.approx(),                       Err::<u8, _>(PosOverflow(400)));
assert_eq!(400u16.approx_by::<Wrapping>(),        Ok::<u8, _>(144u8));
assert_eq!(400u16.approx_as::<u8>(),              Err(PosOverflow(400)));
assert_eq!(400u16.approx_as_by::<u8, Wrapping>(), Ok(144));

// Integer -> float conversions *can* fail due to limited precision.
// Once the continuous range of exactly representable integers is exceeded, the
// provided implementations fail with overflow errors.
assert_eq!(f32::value_from(16_777_216i32), Ok(16_777_216.0f32));
assert_eq!(f32::value_from(16_777_217i32), Err(RangeError::PosOverflow(16_777_217)));

// Float -> integer conversions have to be done using approximations. Although
// exact conversions are *possible*, "advertising" this with an implementation
// is misleading.
//
// Note that `DefaultApprox` for float -> integer uses whatever rounding
// mode is currently active (*i.e.* whatever `as` would do).
assert_eq!(41.0f32.approx(), Ok(41u8));
assert_eq!(41.3f32.approx(), Ok(41u8));
assert_eq!(41.5f32.approx(), Ok(41u8));
assert_eq!(41.8f32.approx(), Ok(41u8));
assert_eq!(42.0f32.approx(), Ok(42u8));

assert_eq!(255.0f32.approx(), Ok(255u8));
assert_eq!(256.0f32.approx(), Err::<u8, _>(FloatError::PosOverflow(256.0)));

// Sometimes, it can be useful to saturate the conversion from float to
// integer directly, then account for NaN as input separately. The `Saturate`
// extension trait exists for this reason.
assert_eq!((-23.0f32).approx_as::<u8>().saturate(), Ok(0));
assert_eq!(302.0f32.approx_as::<u8>().saturate(), Ok(255u8));
assert!(std::f32::NAN.approx_as::<u8>().saturate().is_err());

// If you really don't care about the specific kind of error, you can just rely
// on automatic conversion to `GeneralErrorKind`.
fn too_many_errors() -> Result<(), GeneralErrorKind> {
    let x: u8 = 0u8.value_into()?;
    assert_eq!(x, 0);
    let y: i8 = 0u8.value_into()?;
    assert_eq!(y, 0);
    let z: i16 = 0u8.value_into()?;
    assert_eq!(z, 0);

    let x: u8 = 0.0f32.approx()?;
    assert_eq!(x, 0u8);
    Ok(())
}
too_many_errors().unwrap();
```

<!-- cargo-sync-readme end -->
