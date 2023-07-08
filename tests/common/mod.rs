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
            let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
                .is_test(true)
                .try_init();
            similar_asserts::assert_eq!(typstfmt::format($snippet, typstfmt::Config::default()), $expected);
        }
    };
}
