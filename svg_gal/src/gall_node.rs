use std::cell::{Cell, OnceCell};
use std::f64::consts::TAU;
use std::rc::Rc;

use crate::gall_errors::Error;
use crate::gall_fn;
use crate::gall_loc::{GallOffLoc, Location};
use crate::gall_ord::{GallOrd, PolarOrdinate};

pub struct GallNode {
    pub loc: GallOffLoc,
    l_dist: Rc<OnceCell<f64>>,
    w_rad: Rc<OnceCell<f64>>,
}

//TODO:make Gall_pair

impl GallNode  {
    pub fn new(loc:GallOffLoc, l_dist: Rc<OnceCell<f64>>, w_rad: Rc<OnceCell<f64>>) -> GallNode {
        GallNode {
            loc,
            l_dist,
            w_rad,
        }
    }
    pub fn thi(&self) ->  f64 {
        gall_fn::thi(self.l_dist.get()?,self.loc.dist()?,self.w_rad.get()?)
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
    fn dist(&self) -> Option<f64> {
        self.loc.dist()
    }
    fn get_ang(&self) -> Rc<OnceCell<f64>> {
        todo!()
    }
    fn get_dist(&self) -> Rc<OnceCell<f64>> {
        todo!()
    }
}
impl Location for GallNode  {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    fn set_center(&mut self, new_center: Rc<OnceCell<(f64,f64)>>) {
        self.loc.set_center(new_center)
    }
    fn get_center(&self) -> Rc<OnceCell<(f64,f64)>> {
        self.loc.get_center()
    }
    fn x(&self) -> f64 {
        self.loc.x()
    }
    fn y(&self) -> f64 {
        self.loc.y()
    }
    fn pos_ref(&self) -> Rc<OnceCell<(f64,f64)>> {
        self.loc.pos_ref()
    }
    fn center_ords(&self) -> (f64,f64) {
        self.loc.center_ords()
    }
}