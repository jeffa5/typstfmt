#![warn(missing_docs)]

//! # Typst format
//!
//! Format typst code.
//!
//! [`format()`] is the main point of interest, with [`Config`] for adding some options on how things
//! get formatted.

use tracing::debug;
use typst::syntax::{parse, SyntaxKind, SyntaxNode};

mod config;
mod render;
mod writer;

pub use config::Config;
use render::Renderer;
use writer::Writer;

/// Errors generated during formatting.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    /// Invalid code was given, we don't try to format erroneous things.
    #[error("The input contained errors, not formatting")]
    ErroneousInput,
    /// The formatter produced an invalid output, not letting it get written out.
    #[error("An internal error produced an erroneous output")]
    ProducedErroneousOutput,
    /// The formatter failed to find a fixed point, formatting again will change it.
    #[error("Failed to find fixed point, reformat will not pass check")]
    FailedToFindFixedPoint,
}

/// Check the parsed tree for error nodes, not relying on the in-built `erroneous` field/function.
/// e.g. https://github.com/typst/typst/issues/1690
fn erroneous(node: &SyntaxNode) -> bool {
    if node.kind() == SyntaxKind::Error {
        return true;
    }
    node.children().any(erroneous)
}

/// Format some typst code.
///
/// This first ensures that it is valid typst, returning an error if not.
/// After validation, it traverses the Abstract Syntax Tree, applying formatting along the way.
pub fn format(input: &str, config: Config) -> Result<String, FormatError> {
    debug!("input: {input:?}");
    let init = parse(input);
    // don't try to format things that aren't valid
    if erroneous(&init) {
        debug!(?init, "Not formatting erroneous input");
        let errors = init.errors();
        for error in errors {
            debug!(?error, "error");
        }
        return Err(FormatError::ErroneousInput);
    }
    debug!("parsed: {init:?}");
    let writer = Writer::new(config);

    let mut renderer = Renderer { writer };
    renderer.render(init);

    let output = renderer.finish();

    let reparsed = parse(&output);
    if erroneous(&reparsed) {
        debug!(?output, "Formatted text contained errors!");
        let errors = reparsed.errors();
        for error in errors {
            debug!(?error, "error");
        }
        return Err(FormatError::ProducedErroneousOutput);
    }

    debug!(?output, "checking for fixed point");
    let writer2 = Writer::new(config);
    let mut renderer2 = Renderer { writer: writer2 };
    renderer2.render(reparsed);
    let output2 = renderer2.finish();
    if output != output2 {
        debug!(?output, ?output2, "Formatted text would not pass check");
        return Err(FormatError::FailedToFindFixedPoint);
    }

    Ok(output)
}
