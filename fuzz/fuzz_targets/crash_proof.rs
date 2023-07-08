#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if let Ok(formatted) = typstfmt::format(s, typstfmt::Config::default()) {
            match typstfmt::format(&formatted, typstfmt::Config::default()) {
                Ok(reformatted) => {
                    assert_eq!(reformatted, formatted, "input {:?}", s);
                }
                Err(err) => panic!("Failed reformatting: {:?} input:{:?}", err, s),
            }
        }
    }
});
