#[macro_use]
mod common;

test_snippet!(
    comments,
    expect = "// a line comment\n",
    "// a line comment",
);

test_snippet!(
    codeblock_single,
    expect = r"
#{
  a()[]
}",
    "#{\na()[]\n}",
);
test_snippet!(
    codeblock_multi,
    expect = r"
#{
  a()[]
  b()[]
}",
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
    expect =
        "#let f(\n  /// test comment\n  a,\n  /// another comment\n  b,\n  c,\n) = {a + (b - c)}",
    "#let f(/// test comment\na,\n/// another comment\nb,c) = {a+(b - c)}",
);

test_snippet!(dict_spread_arg, expect = "#(..t, p: p)", "#(..t,p:p)",);

test_snippet!(
    content,
    expect = "*strong*\nnormal\n_emph_\n\nnew para",
    "*strong*\nnormal\n_emph_\n\nnew para",
);

test_snippet!(
    content_block_indent,
    expect = "#[\n  *strong*\n  normal\n  _emph_\n\n  new para\n]",
    "#[\n*strong*\nnormal\n_emph_\n\nnew para\n]",
);

test_snippet!(
    arg_indent,
    expect = r"
#let f(
  a,
  b,
) = a + b",
    "#let f(\na,\nb,\n) = a + b",
);

test_snippet!(
    code_block_indent,
    expect = r"
#{
  1
  2
  3
}",
    "#{\n1\n2\n3\n}",
);

test_snippet!(
    complex,
    expect = r#"
#import "template.typ": *
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

test_snippet! {
    function_content_arg_multiline,
    expect = r##"#{
  f(
  )[
    /* starts
     * the body
     */
  ]
}"##,
    r##"#{
f(
)[
/* starts
  * the body
*/
]
}"##,
}

test_snippet! {
    conditional_spacing,
    expect = r##"#if false {} else if true {} else {}"##,
    r##"#if false{}else if true{}else{}"##,
}

test_snippet! {
    conditional_spaces,
    expect = r##"#if false {} else if true {} else {}"##,
    r##"#if          false        {}       else      if      true       {}   else   {}"##,
}

test_snippet! {
    conditional_newlines,
    expect = r##"
#{
  if false {}
  else if true {}
  else {}
}"##,
    r##"#{
    if false {}
else if true {}
else {}
}"##,
}

test_snippet! {
    nested_array_with_args,
    ignore = "invalid parsing: contains error",
    expect = r##"#((d: 2),)"##,
    r##"#((d: 2,),)"##,
}

test_snippet! {
    nested_array_with_args_2,
    ignore = "invalid parsing: contains error",
    expect = r##"#((d: 2))"##,
    r##"#((d: 2 ,))"##,
}

test_snippet! {
    enumerate_indent,
    expect = r##"
+ some other text that
  is very long
"##,
    r##"
+ some other text that
  is very long
"##,
}

test_snippet! {
    list_indent,
    expect = r##"
- some other text that
  is very long
"##,
    r##"
- some other text that
  is very long
"##,
}

test_snippet! {
    termlist_indent,
    expect = r##"
/ thing: some other text that
  is very long
"##,
    r##"
/ thing: some other text that
  is very long
"##,
}

test_snippet! {
    closure_arg_spacing,
    expect = r"
#locate(loc => {
  })
",
    r"
#locate(loc=>{
})
    ",
}

test_snippet! {
    content_newlines,
    expect = r"
#[
  my text here

  my next paragraph
]
",
    r"
#[





  my text here





  my next paragraph




]
",
}

test_snippet! {
    inline_comment_single_line,
    expect = r"
#let f(/*inline comment*/a) = {}
",
    r"
#let f(/*inline comment*/a) = {}
",
}

test_snippet! {
    comment_newlines_following,
    expect = r"
// test comment

#let a = 2
",
    r"
// test comment

#let a = 2
",
}

test_snippet! {
    code_blank_lines,
    expect = r"
#{
  let a = 1

  let b = 2
}
",
    r"
#{
  let a = 1


  let b = 2
}
",
}
