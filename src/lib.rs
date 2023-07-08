use tracing::debug;
use typst::syntax::{parse, LinkedNode};

mod config;
mod render;
mod writer;

pub use config::Config;
use render::Renderer;
use writer::Writer;

#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    #[error("The input contained errors, not formatting")]
    ErroneousInput,
}

pub fn format(s: &str, config: Config) -> Result<String, FormatError> {
    let init = parse(s);
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
