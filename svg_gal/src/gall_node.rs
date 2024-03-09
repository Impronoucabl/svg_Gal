use std::cell::Cell;
use std::f64::consts::TAU;
use std::rc::Rc;

use crate::gall_errors::Error;
use crate::gall_fn;
use crate::gall_loc::{GallRelLoc, Location};
use crate::gall_ord::PolarOrdinate;

pub struct GallNode {
    loc: GallRelLoc,
    l_dist: Rc<Cell<f64>>,
    w_rad: Rc<Cell<f64>>,
}

//TODO:make Gall_pair

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