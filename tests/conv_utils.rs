use conv2::prelude::*;

#[cfg(feature = "std")]
#[test]
fn test_approx() {
    use conv2::DefaultApprox;
    assert_eq!((1.5f32).approx(), Ok(1i32));
    assert_eq!((1.5f32).approx_by::<DefaultApprox>(), Ok(1));
    assert_eq!((1.5f32).approx_as::<i32>(), Ok(1));
    assert_eq!((1.5f32).approx_as_by::<i32, DefaultApprox>(), Ok(1));
}

#[test]
fn test_into() {
    let v = "ABC".into_as::<Vec<u8>>();
    assert_eq!(&*v, &[0x41, 0x42, 0x43]);
}

#[test]
fn test_value() {
    assert_eq!((123u32).value_as::<u8>(), Ok(123));
}

#[cfg(feature = "std")]
#[test]
fn test_whizzo() {
    use conv2::errors::Unrepresentable;
    assert_eq!(
        (-1.0f32).approx_as::<u8>().saturate(),
        Ok::<_, Unrepresentable<_>>(0u8)
    );
    assert_eq!((-1i32).value_as::<u8>().saturate().unwrap_ok(), 0u8);
}
