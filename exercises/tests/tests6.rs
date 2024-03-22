// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.


struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must be a raw pointer to a `Foo` instance allocated on the heap with `Box::into_raw`.
/// Ownership of the pointed-to `Foo` is transferred to the returned `Box<Foo>`.
/// The caller must ensure that the `ptr` is not null and points to a valid `Foo` instance.
/// The memory must not be accessed elsewhere during the lifetime of the reconstructed `Box<Foo>`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: We reconstruct the box from the raw pointer, transferring ownership.
    let ret: Box<Foo> = unsafe { Box::from_raw(ptr) };
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let initial_data = Foo { a: 1, b: Some("hello".to_owned()) };
        let data = Box::new(initial_data);

        let ptr_1 = &data.a as *const u128 as usize;
        // Convert the box into a raw pointer before passing it to the function.
        // SAFETY: We are passing an owned box of `Foo` converted using `Box::into_raw`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) };

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        // Now the assertion should succeed because 'ret' has been initialized with "hello".
        assert!(ret.b == Some("hello".to_owned()));
    }
}