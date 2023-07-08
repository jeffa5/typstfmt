#![warn(missing_docs)]

//! # Typst format
//!
//! Format typst code.
//!
//! [`format`] is the main point of interest, with [`Config`] for adding some options on how things
//! get formatted.

use tracing::debug;
use typst::syntax::{parse, LinkedNode};

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
}

/// Format some typst code.
///
/// This first ensures that it is valid typst, returning an error if not.
/// After validation, it traverses the Abstract Syntax Tree, applying formatting along the way.
pub fn format(input: &str, config: Config) -> Result<String, FormatError> {
    let init = parse(input);
    // don't try to format things that aren't valid
    if init.erroneous() {
        debug!("Not formatting erroneous input");
        return Err(FormatError::ErroneousInput);
    }
    let root = LinkedNode::new(&init);
    debug!("parsed : \n{init:?}\n");
    let writer = Writer::new(config);

    let mut renderer = Renderer { writer };
    renderer.render(root);

    Ok(renderer.finish())
}
