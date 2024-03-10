use crate::gall_config::Config;
use crate::gall_node::GallNode;

pub struct GallLine<'a> {
    pub node: &'a GallNode,
    pub thickness: i16,
}

pub struct GallLinePair<'a> {
    pub node1: &'a GallNode,
    pub node2: &'a GallNode,
    pub thickness: i16,
}

impl <'a>GallLine<'a> {
    pub fn new(node:&'a GallNode) -> GallLine<'a>{
        GallLine { node, thickness: Config::DEF_PAIR_THICK }
    }
}

impl <'a>GallLinePair<'a> {
    pub fn new(node1:&'a GallNode, node2: &'a GallNode) -> GallLinePair<'a>{
        GallLinePair { node1, node2, thickness: Config::DEF_PAIR_THICK }
    }
}
