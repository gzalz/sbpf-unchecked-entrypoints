#[macro_export]
macro_rules! entrypoint {
    ( $process_instruction:expr, $is_unchecked:expr, $unchecked_handler:expr ) => {
        #[no_mangle]
        pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
            if $is_unchecked(input) {
                return $unchecked_handler(input);
            }
            $process_instruction(input)
        }
    };
}
