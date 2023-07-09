#[macro_use]
mod common;

test_snippet_reformat! { fuzzed_0001, "*h\n*h", }
test_snippet_reformat! { fuzzed_0002, "//", }
test_snippet_reformat! { fuzzed_0003, "#a (", }
test_snippet_reformat! { fuzzed_0004, "#(z*j)", }

// FIXME: unsure why the AST for this includes an empty math
// test_snippet_reformat! {
//     fuzzed_0005,
//     "$JA(;)$",
// }

test_snippet_reformat! { fuzzed_0006, "//\r", }
test_snippet_reformat! { fuzzed_0007, "#k[][*\n*]", }
