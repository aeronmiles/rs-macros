#![allow( unused_macros)]

#[macro_export]
macro_rules! test_async_tokio {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}