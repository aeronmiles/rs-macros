#![allow( unused_macros)]

// #[macro_export]
// macro_rules! assert_prntln_eq {
//     () => {
        
//     };
// }
// assert_eq!(ws.is_err(), false);

#[macro_export]
macro_rules! test_async_tokio {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}