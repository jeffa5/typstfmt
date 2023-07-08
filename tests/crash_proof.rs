#[macro_use]
mod common;

test_snippet_reformat! { fuzzed_0001, "*h\n*h", }
test_snippet_reformat! { fuzzed_0002, "//", }
test_snippet_reformat! { fuzzed_0003, "#a (", }
