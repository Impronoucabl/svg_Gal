use std::cell::Cell;
use std::f64::consts::{PI, TAU};
use std::rc::Rc;

use crate::gall_errors::Error;
use crate::{gall_ang, gall_fn};
use crate::gall_loc::{GallRelLoc, Location};
use crate::gall_ord::PolarOrdinate;

pub struct GallNode {
    loc: GallRelLoc,
    l_dist: Rc<Cell<f64>>,
    w_rad: Rc<Cell<f64>>,
}

impl GallNode  {
    pub fn new(loc:GallRelLoc, l_dist:Rc<Cell<f64>>, word_ord: Rc<Cell<f64>>) -> GallNode { //letter_ord: &'a GallOrd
        GallNode {
            loc,
            l_dist,
            w_rad: word_ord,
        }
    }
    pub fn thi(&self) -> Result<f64, Error> {
        gall_fn::thi(self.l_dist.get(), self.loc.dist(), self.w_rad.get())
    }
    pub fn theta(&self) -> Result<f64, Error> {
        gall_fn::theta(self.l_dist.get(),self.loc.dist(),self.w_rad.get())
    }
    fn broken_gap(&self) -> bool {
        if let (Some(ang),Ok(thi)) = (self.ang(),self.thi()) {
            ang + PI - thi > TAU || ang - PI + thi < 0.0 
        } else {
            false
        }
    }
    fn ang_bounds(&self) -> (f64,f64) {
        if let (Some(ang),Ok(theta)) = (self.ang(),self.theta()) {
            (gall_ang::constrain(ang - theta + PI), 
            gall_ang::constrain(ang + theta - PI))
        } else {(0.0,TAU)}
    }
    pub fn node_test(&self, node2:&GallNode)-> bool {
        //pass on true
        self.center_test(node2) &&
        self.node_angle_test(node2)
    }
    pub fn center_test(&self, node2: &GallNode) -> bool {
        self.get_center() != node2.get_center()
    }
    pub fn node_angle_test(&self, node2:&GallNode) -> bool {
        let ang = self.cent_ang2cent_ang(node2);
        self.angle_test(ang)
    }
    pub fn angle_test(&self, ang:f64) -> bool {
        let (cw, ccw) = self.ang_bounds();
        if self.broken_gap() {
            ang < cw || ang > ccw
        } else {
            ang > cw && ang < ccw
        }
    }
}
impl PolarOrdinate for GallNode {
    fn mut_ang(&mut self, new_ang:f64) {
        self.loc.mut_ang(new_ang)
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error> {
        self.loc.mut_dist(new_dist)
    }
    fn ang(&self) -> Option<f64> {
        self.loc.ang()
    }
    fn dist(&self) -> f64 {
        self.loc.dist()
    }
    fn get_dist(&self) -> Rc<Cell<f64>> {
        self.l_dist.clone()
    }
}
impl Location for GallNode  {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    fn set_center(&mut self, new_center: Rc<Cell<(f64,f64)>>) {
        self.loc.set_center(new_center)
    }
    fn get_center(&self) -> Rc<Cell<(f64,f64)>> {
        self.loc.get_center()
    }
    fn x(&self) -> f64 {
        self.loc.x()
    }
    fn y(&self) -> f64 {
        self.loc.y()
    }
    fn pos_ref(&self) -> Rc<Cell<(f64,f64)>> {
        self.loc.pos_ref()
    }
    fn update(&mut self) {
        self.loc.update()
    }
}