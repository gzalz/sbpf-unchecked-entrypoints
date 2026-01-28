#[allow(dead_code)]
const MAX_TX_ACCOUNTS: usize = (u8::MAX - 1) as usize;

#[macro_export]
macro_rules! conditional_entrypoint {
    ($checked_handler: expr, $is_unchecked:expr, $unchecked_handler:expr ) => {
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            if $is_unchecked(input) {
                return $unchecked_handler(input);
            }
            pinocchio::entrypoint::process_entrypoint::<MAX_TX_ACCOUNTS>(input, $checked_handler)
        }
    };
}

#[macro_export]
macro_rules! unchecked_entrypoint {
    ( $unchecked_handler:expr ) => {
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            return $unchecked_handler(input);
        }
    };
}
