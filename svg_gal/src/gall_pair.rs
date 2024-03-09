use crate::gall_config::Config;
use crate::gall_node::GallNode;

pub struct GallLinePair<'a> {
    pub node1: &'a GallNode,
    pub node2: &'a GallNode,
    pub thickness: i16,
}

impl <'a>GallLinePair<'a> {
    pub fn new(node1:&'a GallNode, node2: &'a GallNode) -> GallLinePair<'a>{
        GallLinePair { node1, node2, thickness: Config::DEF_PAIR_THICK }
    }
}