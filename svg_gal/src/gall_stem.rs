use std::cell::Cell;
use std::rc::Rc;

use crate::gall_errors::{Error, GallError};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_circle::{ChildCircle, Circle, HollowCircle};
use crate::gall_ord::PolarOrdinate;

#[derive(PartialEq,Clone,Copy)]
pub enum StemType {J,B,S,Z}

pub struct Stem {
    loc: GallLoc,
    radius: Rc<Cell<f64>>,
    thickness: Rc<Cell<f64>>,
    parent_radius: Rc<Cell<f64>>,
    parent_thickness: Rc<Cell<f64>>,
    pub stem_type: StemType,
}
impl Stem {
    pub fn new<T:HollowCircle>(loc:GallLoc, radius: f64, thickness:f64, stem_type: StemType, parent:&T) -> Stem {
        let radius = Rc::new(Cell::new(radius));
        let thickness = Rc::new(Cell::new(thickness));
        let parent_radius = parent.get_radius().clone();
        let parent_thickness = parent.get_thickness().clone();
        Stem {
            loc,
            radius,
            thickness,
            parent_radius,
            parent_thickness,
            stem_type,
        }
    }
    fn radius_limits(&self) -> (f64,f64) {
        match &self.stem_type {
            StemType::J => (self.parent_inner() - self.dist()- 2.0*self.thick(),0.0),
            StemType::B => (self.parent_outer() + self.parent_thick() - self.dist() - self.thick(), self.parent_outer() - self.dist() - self.thick()),
            StemType::S => (f64::INFINITY,0.0),
            StemType::Z => (2.0*self.parent_outer() - self.parent_radius() + self.dist() - self.thick(), 0.0),
        }
    }
    fn dist_limits(&self) -> (f64,f64) {
        match &self.stem_type {
            StemType::J => (self.parent_inner() - self.outer_radius() - self.thick(), 0.0),
            StemType::B => (self.parent_outer() + self.parent_thick() - self.outer_radius(), self.parent_outer() - self.outer_radius()),
            StemType::S => (self.parent_inner() + self.inner_radius(),self.parent_inner()),
            StemType::Z => (self.parent_outer(),self.parent_inner()),
        }
    }
    fn thick_limits(&self) -> (f64,f64) {
        //todo!()
        (2.0,200.0)
    }
    fn check_radius(&self, test_val:f64) -> Result<(), Error> {
        let (upper_limit, lower_limit) = self.radius_limits();
        if test_val > upper_limit {
            Err(Error::new(GallError::RadiusTooLong))
        } else if test_val < lower_limit {
            Err(Error::new(GallError::RadiusTooShort))
        } else {
            Ok(())
        }
    }
    fn check_dist(&self, test_val:f64) -> Result<(), Error> {
        let (upper_limit, lower_limit) = self.dist_limits();
        if test_val > upper_limit {
            Err(Error::new(GallError::DistTooLong))
        } else if test_val < lower_limit {
            Err(Error::new(GallError::DistTooShort))
        } else {
            Ok(())
        }
    }
    fn check_thick(&self, test_val:f64) -> Result<(), Error> {
        let (upper_limit, lower_limit) = self.thick_limits();
        if test_val > upper_limit {
            Err(Error::new(GallError::TooThick))
        } else if test_val < lower_limit {
            Err(Error::new(GallError::NotThickEnough))
        } else {
            Ok(())
        }
    }
}
impl PolarOrdinate for Stem {
    fn ang(&self) -> Option<f64> {
        self.loc.ang()
    }
    fn dist(&self) -> f64 {
        self.loc.dist()
    }
    fn mut_ang(&mut self, angle:f64) {
        self.loc.mut_ang(angle)
    }
    fn mut_dist(&mut self, new_dist:f64) -> Result<(), Error> {
        self.check_dist(new_dist)?;
        self.loc.mut_dist(new_dist);
        Ok(())
    }
    fn get_dist(&self) -> Rc<Cell<f64>> {
        self.loc.get_dist()
    }
}
impl Location for Stem {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    fn set_center(&mut self, new_center:Rc<Cell<(f64,f64)>>) {
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
impl ChildCircle for Stem {
    fn parent_radius(&self) -> f64 {
        self.parent_radius.get()
    }
    fn parent_thick(&self) -> f64 {
        self.parent_thickness.get()
    }
    // fn mut_parent_radius(&mut self, new_radius:f64) -> Result<(),Error> {
    //     (self.mut_parent_rad_fn)(new_radius)
    // }
    // fn mut_parent_thick(&mut self, new_thick:f64) -> Result<(),Error> {
    //     (self.mut_parent_thick_fn)(new_thick)
    // }
    fn get_parent_radius(&self) -> Rc<Cell<f64>> {
        self.parent_radius.clone()
    }
    fn get_parent_thick(&self) -> Rc<Cell<f64>> {
        self.parent_thickness.clone()
    }
}
impl Circle for Stem {
    fn radius(&self) -> f64 {
        self.radius.get()
    }
    fn get_radius(&self) -> Rc<Cell<f64>> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        self.check_radius(new_radius)?;
        self.radius.set(new_radius);
        Ok(())
    }
}
impl HollowCircle for Stem {
    fn get_thickness(&self) -> Rc<Cell<f64>> {
        self.thickness.clone()
    }
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error> {
        self.check_thick(new_thick)?;
        self.thickness.set(new_thick);
        Ok(())
    }
    fn thick(&self) -> f64 {
        self.thickness.get()
    }
}