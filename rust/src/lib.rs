#[no_mangle]
pub extern "C" fn rust_test_panic(_some_value: isize) -> isize {
    match ::std::panic::catch_unwind(|| {
        println!("Throwing panic...");
        panic!("test panic")
    }) {
        Ok(()) => (),
        Err(cause) => {
            let error_message = if let Some(cause) = cause.downcast_ref::<&str>() {
                cause.to_string()
            } else if let Some(cause) = cause.downcast_ref::<String>() {
                cause.to_string()
            } else {
                format!("{:?}", cause)
            };

            println!("Panic successfully caught: {}", error_message);
        }
    }

    0
}
