use std::cell::{Cell, OnceCell};
use std::f64::consts::TAU;
use std::rc::Rc;

use crate::gall_errors::Error;
use crate::gall_loc::{GallRelLoc, Location};
use crate::gall_ord::{GallOrd, PolarOrdinate};

pub struct GallNode<'a> {
    pub loc: GallRelLoc,
    l_dist: &'a GallOrd,//Rc<Cell<GallOrd>>,
    w_rad: Rc<Cell<f64>>,
}

//TODO:make Gall_pair

impl GallNode<'_>  {
    pub fn new<'a>(loc:GallRelLoc, letter_ord: &'a GallOrd, word_ord: Rc<Cell<f64>>) -> GallNode<'a> {
        GallNode {
            loc,
            l_dist: letter_ord,
            w_rad: word_ord,
        }
    }
}
impl PolarOrdinate for GallNode<'_> {
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
}
impl Location for GallNode<'_>  {
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
}