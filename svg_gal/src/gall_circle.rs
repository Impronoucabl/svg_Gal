use std::rc::Rc;
use std::cell::Cell;

use crate::gall_errors::Error;
use crate::gall_loc::{GallRelLoc, Location};
use crate::gall_ord::PolarOrdinate;

pub struct Dot {
    loc: GallRelLoc,
    radius: Rc<Cell<f64>>,
    word_radius: Rc<Cell<f64>>,
} 
pub trait Circle {
    fn radius(&self) -> f64;
    fn mut_radius(&mut self, new_radius:f64) -> Result<(),Error>;
    fn get_radius(&self) -> Rc<Cell<f64>>;
}
pub trait HollowCircle: Circle {
    fn thick(&self) -> f64;
    fn get_thickness(&self) -> Rc<Cell<f64>>;
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error>;
    fn outer_radius(&self) -> f64 {
        self.radius() + self.thick()
    }
    fn inner_radius(&self) -> f64 {
        self.radius() - self.thick()
    }
}
pub trait ChildCircle{
    fn parent_radius(&self) -> f64;
    fn parent_thick(&self) -> f64;
    fn get_parent_radius(&self) -> Rc<Cell<f64>>;
    fn get_parent_thick(&self) -> Rc<Cell<f64>>;
    //fn mut_parent_radius(&mut self, new_radius:f64) -> Result<(),Error>;
    //fn mut_parent_thick(&mut self, new_thickness:f64) -> Result<(),Error>;
    fn parent_inner(&self) -> f64 {
        self.parent_radius() - self.parent_thick()
    }
    fn parent_outer(&self) -> f64 {
        self.parent_radius() + self.parent_thick()
    }
}
impl Dot {
    pub fn new(loc: GallRelLoc, radius:f64, w_rad:Rc<Cell<f64>>) -> Dot{
        Dot {
            loc,
            radius: Rc::new(Cell::new(radius)),
            word_radius: w_rad,
        }
    }
}
impl Circle for Dot {
    fn radius(&self) -> f64 {
        self.radius.get()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(),Error> {
        self.radius.set(new_radius);
        Ok(())
    }
    fn get_radius(&self) -> Rc<Cell<f64>> {
        self.radius.clone()
    }
}
impl PolarOrdinate for Dot {
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
        self.loc.get_dist()
    }
}
impl Location for Dot {
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