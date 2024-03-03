use std::rc::Rc;
use std::cell::OnceCell;

use crate::gall_errors::Error;
use crate::gall_loc::{GallOffLoc, Location};
use crate::gall_ord::PolarOrdinate;

pub struct Dot {
    loc: GallOffLoc,
    radius: OnceCell<f64>,
} 
pub trait Circle {
    fn radius(&self) -> Option<&f64>;
    fn mut_radius(&mut self, new_radius:f64) -> Result<(),Error>;
    fn get_radius(&self) -> OnceCell<f64>;
}
pub trait HollowCircle: Circle {
    fn thick(&self) -> Option<&f64>;
    fn get_thickness(&self) -> OnceCell<f64>;
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error>;
    fn outer_radius(&self) -> Option<f64> {
        Some(self.radius()? + self.thick()?)
    }
    fn inner_radius(&self) -> Option<f64> {
        Some(self.radius()? - self.thick()?)
    }
}
pub trait ChildCircle{
    fn parent_radius(&self) -> f64;
    fn parent_thick(&self) -> f64;
    fn get_parent_radius(&self) -> Option<&f64>;
    fn get_parent_thick(&self) -> Option<&f64>;
    fn parent_inner(&self) -> f64 {
        self.parent_radius() - self.parent_thick()
    }
    fn parent_outer(&self) -> f64 {
        self.parent_radius() + self.parent_thick()
    }
}
impl Dot {
    pub fn new(loc: GallOffLoc, radius:f64) -> Dot{
        let rad = OnceCell::new();
        rad.set(radius);
        Dot {
            loc,
            radius: rad,
        }
    }
}
impl Circle for Dot {
    fn radius(&self) -> Option<&f64> {
        self.radius.get()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(),Error> {
        _ = self.radius.take();
        _ = self.radius.set(new_radius);
        Ok(())
    }
    fn get_radius(&self) -> OnceCell<f64> {
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
    fn ang(&self) -> Option<&f64> {
        self.loc.ang()
    }
    fn dist(&self) -> Option<&f64> {
        self.loc.dist()
    }
    fn get_ang(&self) -> OnceCell<f64> {
        self.loc.get_ang()
    }
    fn get_dist(&self) -> OnceCell<f64> {
        self.loc.get_dist()
    }
}
impl Location for Dot {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    fn set_center(&mut self, new_center: OnceCell<(f64,f64)>) {
        self.loc.set_center(new_center)
    }
    fn get_center(&self) -> OnceCell<(f64,f64)> {
        self.loc.get_center()
    }
    fn x(&self) -> f64 {
        self.loc.x()
    }
    fn y(&self) -> f64 {
        self.loc.y()
    }
    fn pos_ref(&self) -> OnceCell<(f64, f64)> {
        self.loc.pos_ref()
    }
    fn center_ords(&self) -> (f64,f64) {
        todo!()
    }
}