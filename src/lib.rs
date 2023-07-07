use log::info;
use typst::syntax::{parse, LinkedNode};

mod render;
mod writer;
use writer::Writer;

use crate::render::Renderer;

// Optimize: could return Text edit that should be applied one after the other
// instead of String
pub fn typst_format(s: &str) -> String {
    let init = parse(s);
    let root = LinkedNode::new(&init);
    info!("parsed : \n{init:?}\n");
    let mut result = String::with_capacity(1024);
    let writer = Writer::default(&mut result);

    let mut renderer = Renderer { writer };
    renderer.render(root);

    result
}
