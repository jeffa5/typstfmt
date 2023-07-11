#[macro_use]
mod common;

test_snippet_unchanged!(function, "#f (  a  ,  b :   c,  ..d    ) = (a+b)",);
test_snippet_unchanged!(let_binding, "#let    x       =      3",);

test_snippet_unchanged!(fuzzed_0001, "$\n\0$",);
test_snippet_unchanged!(fuzzed_0002, "#``",);
test_snippet_unchanged!(fuzzed_0003, "0.\n",);
test_snippet_unchanged!(fuzzed_0004, "=  ",);
test_snippet_unchanged!(fuzzed_0005, "-  ",);
test_snippet_unchanged!(fuzzed_0006, "+ ",);
test_snippet_unchanged!(fuzzed_0007, "xI#2.",);
test_snippet_unchanged!(fuzzed_0008, "\u{4}$ƽ#0\u{4}$ƽ",);
test_snippet_unchanged!(fuzzed_0009, "$w_\0$",);
test_snippet_unchanged!(fuzzed_0010, "$\u{2}/\np$",);
test_snippet_unchanged!(fuzzed_0011, "#04#5",);
test_snippet_unchanged!(fuzzed_0012, "```n```",);
test_snippet_unchanged!(fuzzed_0013, "/ :",);
test_snippet_unchanged!(fuzzed_0014, "$#L/.$",);
test_snippet_unchanged!(fuzzed_0015, "#2[]",);
test_snippet_unchanged!(fuzzed_0016, "$| |$",);
test_snippet_unchanged!(fuzzed_0017, "#break",);
test_snippet_unchanged!(fuzzed_0018, "#(..)",);
test_snippet_unchanged!(fuzzed_0019, "#()=A",);
test_snippet_unchanged!(fuzzed_0020, "#(..A)",);
test_snippet_unchanged!(fuzzed_0021, "#z(..E)",);
test_snippet_unchanged!(fuzzed_0022, "l/*",);
test_snippet_unchanged!(fuzzed_0023, "#(: )-",);
test_snippet_unchanged!(fuzzed_0024, "#(V:p)",);
test_snippet_unchanged!(fuzzed_0025, "#return{}- u",);
test_snippet_unchanged!(fuzzed_0026, "$nu(::)$",);
test_snippet_unchanged!(fuzzed_0027, "$nu(-;:)$",);
