#[macro_use]
mod common;

test_snippet!(
    comments,
    expect = "// a line comment\n",
    "// a line comment",
);

test_snippet!(
    codeblock_single,
    expect = "#{\n    a()[]\n}",
    "#{\na()[]\n}",
);
test_snippet!(
    codeblock_multi,
    expect = "#{\n    a()[]\n    b()[]\n}",
    "#{\na()[]\nb()[]\n}",
);

test_snippet!(plain_text, expect = "hello world", "hello   world",);

test_snippet!(let_binding, expect = "#let x = 4", "#let x=4",);

test_snippet!(
    function_single_line,
    expect = "#let f(a, b, c) = {a + (b - c)}",
    "#let f(a,b,c) = {a+(b - c)}",
);

test_snippet!(
    function_spread_arg,
    expect = "#f(..s, k: v)",
    "#f(..s, k: v)",
);

test_snippet!(function_content_arg, expect = "#k(2)[]", "#k(2)[]",);

test_snippet!(function_content_args, expect = "#k[][]", "#k[][]",);

test_snippet!(
    function_multi_line,
    expect = "#let f(\n    /// test comment\n    a,\n    /// another comment\n    b,\n    c,\n) = {a + (b - c)}",
    "#let f(/// test comment\na,\n/// another comment\nb,c) = {a+(b - c)}",
);

test_snippet!(
    content,
    expect = "*strong*\nnormal\n_emph_\n\nnew para",
    "*strong*\nnormal\n_emph_\n\nnew para",
);

test_snippet!(
    content_block_indent,
    expect = "#[\n    *strong*\n    normal\n    _emph_\n\n    new para\n]",
    "#[\n*strong*\nnormal\n_emph_\n\nnew para\n]",
);

test_snippet!(
    arg_indent,
    expect = "#let f(\n    a,\n    b,\n) = a + b",
    "#let f(\na,\nb,\n) = a + b",
);

test_snippet!(
    code_block_indent,
    expect = "#{\n    1\n    2\n    3\n}",
    "#{\n1\n2\n3\n}",
);

test_snippet!(
    complex,
    expect = r#"#import "template.typ": *
#show: letter.with(
    sender: [Jane Smith, Universal Exports, 1 Heavy Plaza, Morristown, NJ 07964,],
    recipient: [Mr. John Doe \ Acme Corp. \ 123 Glennwood Ave \ Quarto Creek, VA 22438],
    date: [Morristown, June 9th, 2023,],
    subject: [test],
    name: [Jane Smith \Regional Director],
)

Dear Joe,

#lorem(9)

Best,"#,
    r#"#import "template.typ": *
#show: letter.with(sender:[Jane Smith, Universal Exports, 1 Heavy Plaza, Morristown, NJ 07964,],
recipient: [Mr. John Doe \ Acme Corp. \ 123 Glennwood Ave \ Quarto Creek, VA 22438],date: [Morristown, June 9th, 2023,],subject: [test],name: [Jane Smith \Regional Director],)

Dear Joe,

#lorem(9)

Best,"#,
);
