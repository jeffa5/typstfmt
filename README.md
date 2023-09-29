# Typst formatter

`typstfmt` is a formatter for [Typst](https://typst.app) code.

It only formats inputs that are valid Typst code.
It aims to make the code consistent.

## Install

```sh
cargo install --git https://github.com/jeffa5/typstfmt
```

## Run

```sh
typstfmt
```

## Configuration

You can configure some aspects of the formatting with a `typstfmt.toml` file in the current directory, or specify its location with the `--config-path` flag.

The default configuration is:

```toml
indent = 2 # spaces
spacing = true # whether to manage spacing
```

## Development

### Fuzzing

List some fuzz targets:

```sh
cargo fuzz list
```

Then run one, e.g. for `nofmt_unchanged`:

```sh
cargo fuzz run nofmt_unchanged
```

### Testing against the package repo

The [`typst packages`](https://github.com/typst/packages) repo is a submodule (`typst-packages`).
We can run the formatter against it to check the formatting and for erroneous outputs with:

```sh
cargo run -- typst-packages --check
```

And try to format them all (useful for manual diffing):

```sh
cargo run -- typst-packages
```

## Acknowledgements

`typstfmt` is a rewrite of [`typst-fmt`](https://github.com/astrale-sharp/typst-fmt) which aims to retain all original text whilst also be able to be flexible in its configuration.
I tried writing some rules for that formatter before beginning the redesign present here.
