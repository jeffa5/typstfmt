#[macro_use]
mod common;

test_snippet_unchanged!(function, "#f (  a  ,  b :   c,  ..d    ) = (a+b)\n",);
test_snippet_unchanged!(let_binding, "#let    x       =      3\n",);

test_snippet_unchanged!(fuzzed_0001, "$\n\0$\n",);
test_snippet_unchanged!(fuzzed_0002, "#``\n",);
test_snippet_unchanged!(fuzzed_0003, "0.\n",);
test_snippet_unchanged!(fuzzed_0004, "=  \n",);
test_snippet_unchanged!(fuzzed_0005, "-\n",);
test_snippet_unchanged!(fuzzed_0006, "+\n",);
test_snippet_unchanged!(fuzzed_0007, "xI#2.\n",);
test_snippet_unchanged!(fuzzed_0008, "\u{4}$ƽ#0\u{4}$ƽ\n",);
test_snippet_unchanged!(fuzzed_0009, "$w_\0$\n",);
test_snippet_unchanged!(fuzzed_0010, "$\u{2}/\np$\n",);
test_snippet_unchanged!(fuzzed_0011, "#04#5\n",);
test_snippet_unchanged!(fuzzed_0012, "```n```\n",);
test_snippet_unchanged!(fuzzed_0013, "/ :\n",);
test_snippet_unchanged!(fuzzed_0014, "$#L/.$\n",);
test_snippet_unchanged!(fuzzed_0015, "#2[]\n",);
test_snippet_unchanged!(fuzzed_0016, "$| |$\n",);
test_snippet_unchanged!(fuzzed_0017, "#break\n",);
// invalid input now!
// test_snippet_unchanged!(fuzzed_0018, "#(..)\n",);
test_snippet_unchanged!(fuzzed_0019, "#()=A\n",);
test_snippet_unchanged!(fuzzed_0020, "#(..A)\n",);
test_snippet_unchanged!(fuzzed_0021, "#z(..E)\n",);
test_snippet_unchanged!(fuzzed_0022, "l/*\n",);
test_snippet_unchanged!(fuzzed_0023, "#(: )-\n",);
test_snippet_unchanged!(fuzzed_0024, "#(V:p)\n",);
test_snippet_unchanged!(fuzzed_0025, "#return{}- u\n",);
test_snippet_unchanged!(fuzzed_0026, "$nu(::)$\n",);
