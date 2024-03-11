use std::cell::Cell;
use std::rc::Rc;

use crate::gall_ang;
use crate::gall_circle::Circle;
use crate::gall_config::Config;
use crate::gall_loc::Location;
use crate::gall_node::GallNode;
use crate::gall_ord::PolarOrdinate;
use crate::gall_sentence::GallSentence;

pub struct GallLine<'a> {
    pub node: &'a GallNode,
    pub thickness: i16,
    sent_radius: Rc<Cell<f64>>,
    sent_cent:Rc<Cell<(f64,f64)>>,
}

pub struct GallLinePair<'a> {
    pub node1: &'a GallNode,
    pub node2: &'a GallNode,
    pub thickness: i16,
}

impl <'a>GallLine<'a> {
    pub fn new(node:&'a GallNode, radius:Rc<Cell<f64>>, center:Rc<Cell<(f64,f64)>>) -> GallLine<'a>{
        GallLine { 
            node, 
            thickness: Config::DEF_PAIR_THICK, 
            sent_radius: radius,
            sent_cent: center,
        }
    }
    pub fn get_endpoint(&self) -> (f64,f64) {
        let gall_ang = self.node.ang().expect("Node can't be at center");
        let (y_d,x_d) = gall_ang::gall_ang2svg_ang(gall_ang).sin_cos();
        let (x_c,y_c) = self.sent_cent.get();
        let (x_0,y_0) = self.node.get_center().get();
        let r_s = self.sent_radius.get();
        let (x_diff, y_diff) = (x_0-x_c,y_0-y_c);
        //----quadratic formula----(b=2d)
        let d = x_diff*x_d+y_diff*y_d;
        let c = x_diff*x_diff + y_diff*y_diff - r_s*r_s; 
        let n = (d*d-c).sqrt()-d;
        //----debug----
        // println!("gall_ang: {}, n: {}",gall_ang,n);
        // println!("x_d: {}, y_d: {}",x_d,y_d);
        // println!("x_0: {}, y_0: {}",x_0,y_0);
        (n*x_d+x_0,n*y_d+y_0)
    }
}

impl <'a>GallLinePair<'a> {
    pub fn new(node1:&'a GallNode, node2: &'a GallNode) -> GallLinePair<'a>{
        GallLinePair { node1, node2, thickness: Config::DEF_PAIR_THICK }
    }
}
