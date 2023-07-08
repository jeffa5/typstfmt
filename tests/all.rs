#[macro_use]
mod common;

test_snippet!(comments, expect = "// a line comment", "// a line comment",);

test_snippet!(codeblock_single, expect = "#{a([])}", "#{\na()[]\n}",);
test_snippet!(
    codeblock_multi,
    expect = "#{\n    a([])\n    b([])\n}",
    "#{\na()[]\nb()[]\n}",
);

test_snippet!(let_binding, expect = "#let x = 4", "#let x=4",);

test_snippet!(
    function,
    expect = "#f(\n    a,\n    b,\n    c,\n) = {a + (b + c)}",
    "#f(a, b, c) = {a + (b + c)}",
);

test_snippet!(
    content,
    expect = "*strong*\nnormal\n_emph_\n\nnew para",
    "*strong*\nnormal\n_emph_\n\nnew para",
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
#show: letter.with(sender:[Jane Smith, Universal Exports, 1 Heavy Plaza, Morristown, NJ 07964,],recipient: [Mr. John Doe \ Acme Corp. \ 123 Glennwood Ave \ Quarto Creek, VA 22438],date: [Morristown, June 9th, 2023,],subject: [test],name: [Jane Smith \Regional Director],)

Dear Joe,

#lorem(9)

Best,"#,
);
