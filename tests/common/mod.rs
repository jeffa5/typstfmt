#[allow(unused_macros)]
macro_rules! test_snippet {
    (
        $test_name:ident,
        $(ignore = $ignore:tt ,)?
        expect = $expected:expr,
        $snippet:expr,
    ) => {
        #[test]
        $(#[ignore = $ignore])?
        fn $test_name() {
            let _ = tracing_subscriber::fmt().with_test_writer().with_max_level(tracing::Level::DEBUG).try_init();
            let formatted = typstfmt::format($snippet, typstfmt::Config::default()).unwrap();
            similar_asserts::assert_eq!(formatted, $expected, "first format");
            let reformatted = typstfmt::format(&formatted, typstfmt::Config::default()).unwrap();
            similar_asserts::assert_eq!(reformatted, $expected, "second format");
        }
    };
}

#[allow(unused_macros)]
macro_rules! test_snippet_unchanged {
    (
        $test_name:ident,
        $(ignore = $ignore:tt ,)?
        $snippet:expr,
    ) => {
        #[test]
        $(#[ignore = $ignore])?
        fn $test_name() {
            let _ = tracing_subscriber::fmt().with_test_writer().with_max_level(tracing::Level::DEBUG).try_init();
            let formatted = typstfmt::format($snippet, typstfmt::Config::no_changes()).unwrap();
            similar_asserts::assert_eq!(&formatted, $snippet);
        }
    };
}
