# Typst formatter

`typstfmt` is a formatter for [`typst`](https://typst.app) code.
It essentially is a pretty AST printer, using the typst Rust library for parsing.

It only formats inputs that are valid typst code.

## Fuzzing

List some fuzz targets:

```sh
cargo fuzz list
```

Then run one, e.g. for `nofmt_unchanged`:

```sh
cargo fuzz run nofmt_unchanged
```

## Acknowledgements

`typstfmt` is a rewrite of [`typst-fmt`](https://github.com/astrale-sharp/typst-fmt) which aims to retain all original text whilst also be able to be flexible in its configuration.
I tried writing some rules for that formatter before beginning the redesign present here.