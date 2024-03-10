use std::f64::consts::PI;

use crate::gall_config::Config;
use crate::gall_loc::Location;
use crate::gall_node::GallNode;
use crate::gall_ord::PolarOrdinate;

pub struct GallLinePair<'a> {
    pub node1: &'a GallNode,
    pub node2: &'a GallNode,
    pub thickness: i16,
}

impl <'a>GallLinePair<'a> {
    pub fn new(node1:&'a GallNode, node2: &'a GallNode) -> GallLinePair<'a>{
        GallLinePair { node1, node2, thickness: Config::DEF_PAIR_THICK }
    }
    // pub fn align(&self) {
    //     let ang1 = self.node1.cent_ang2cent_ang(self.node2);
    //     self.node1.mut_ang(ang1);
    //     self.node2.mut_ang(ang1-PI);
    // }
}