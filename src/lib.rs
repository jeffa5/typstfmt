use log::info;
use typst::syntax::{parse, LinkedNode};

mod config;
mod render;
mod writer;

pub use config::Config;
use render::Renderer;
use writer::Writer;

pub fn format(s: &str, config: Config) -> String {
    let init = parse(s);
    let root = LinkedNode::new(&init);
    info!("parsed : \n{init:?}\n");
    let writer = Writer::new(config);

    let mut renderer = Renderer { writer };
    renderer.render(root);

    renderer.finish()
}
