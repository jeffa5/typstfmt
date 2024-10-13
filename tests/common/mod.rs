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
            let snippet = $snippet.trim_start();
            let expected = $expected.trim_start();
            let formatted = typstfmt::format(snippet, typstfmt::Config::default()).unwrap();
            similar_asserts::assert_eq!(formatted, expected, "first format");
            let reformatted = typstfmt::format(&formatted, typstfmt::Config::default()).unwrap();
            similar_asserts::assert_eq!(reformatted, expected, "second format");
        }
    };
}

#[allow(unused_macros)]
macro_rules! test_snippet_reformat {
    (
        $test_name:ident,
        $snippet:expr,
    ) => {
        #[test]
        fn $test_name() {
            let _ = tracing_subscriber::fmt()
                .with_test_writer()
                .with_max_level(tracing::Level::DEBUG)
                .try_init();
            let formatted = typstfmt::format($snippet, typstfmt::Config::default()).unwrap();
            println!("first formatting done, produced:");
            println!("{:?}", formatted);
            let reformatted = typstfmt::format(&formatted, typstfmt::Config::default()).unwrap();
            println!("second formatting done, produced:");
            println!("{:?}", reformatted);
            similar_asserts::assert_eq!(reformatted, formatted);
        }
    };
}
