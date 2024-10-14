# Typst formatter

`typstfmt` is a formatter for [Typst](https://typst.app) code.

It only formats inputs that are valid Typst code.
It aims to make the code consistent.

## Install

### Cargo

```sh
cargo install --git https://github.com/jeffa5/typstfmt
```

### Nix

The flake provides an overlay which you can use with nixpkgs.

### pre-commit

Add this to your `.pre-commit-config.yaml`:

```yaml
  - repo: https://github.com/jeffa5/typstfmt
    rev: ''  # Use the sha / tag you want to point at
    hooks:
      - id: typstfmt
```

## Run

```sh
# format stdin
typstfmt
# format typst files in current directory
typstfmt *.typ
```

### Nix

```sh
nix run github:jeffa5/typstfmt
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

Then run one, e.g. for `crash_proof`:

```sh
cargo fuzz run crash_proof
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
