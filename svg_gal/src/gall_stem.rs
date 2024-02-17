use std::rc::Rc;

use crate::gall_errors::{Error, GallError};
use crate::gall_ord::{BoundedValue, CenterOrd, GallAng, GallLoc, LocMover, PositiveDist};
use crate::gall_struct::{ChildCircle, Circle, HollowCircle};

#[derive(PartialEq)]
pub enum StemType {J,B,S,Z}

pub struct Stem {
    loc: GallLoc,
    radius: Rc<PositiveDist>,
    thickness: Rc<PositiveDist>,
    parent_radius: Rc<PositiveDist>,
    parent_thickness: Rc<PositiveDist>,
    pub stem_type: StemType,
}
impl Stem {
    pub fn new<T:HollowCircle>(loc:GallLoc, radius: f64, thickness:f64, stem_type: StemType, parent:T) -> Result<Stem,Error> {
        let radius = Rc::new(PositiveDist::new(radius)?);
        let thickness = Rc::new(PositiveDist::new(thickness)?);
        let parent_radius = parent.get_radius().clone();
        let parent_thickness = parent.get_thickness().clone();

        Ok(Stem {
            loc,
            radius,
            thickness,
            parent_radius,
            parent_thickness,
            stem_type,
        })
    }
}
impl LocMover for Stem {
    fn mut_ang(&mut self, angle:GallAng) -> Result<(), Error> {
        self.loc.mut_ang(angle)
    }
    fn mut_dist(&mut self, new_dist:f64) -> Result<(), Error> {
        let (p_rad, p_thick, s_rad, s_thick) = (
            self.parent_radius.dist(),
            self.parent_thickness.dist(),
            self.radius.dist(),
            self.thickness.dist(),
        );
        let upper_limit = match self.stem_type {
            J => p_rad - p_thick - s_rad - s_thick*2.0,
            B => p_rad - p_thick - s_thick,
            S => p_rad + p_thick + s_rad + s_thick,
            Z => p_rad + p_thick + s_thick,
        };
        let lower_limit = match self.stem_type {
            J => 0.0,
            B => p_rad - p_thick - s_rad - s_thick,
            S => p_rad - p_thick,
            Z => p_rad - p_thick,
        };
        if upper_limit > new_dist {
            if lower_limit < new_dist {
                self.loc.mut_dist(new_dist)
            } else {
                Err(Error::new(GallError::StemDistTooShort))
            }
        } else {
            Err(Error::new(GallError::StemDistTooLong))
        }
    }
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    //This won't change any centers that have been cloned off this one
    fn set_center(&mut self, new_center:CenterOrd) {
        self.loc.set_center(new_center)
    }
    fn get_ang(&self) -> GallAng {
        self.loc.get_ang()
    }
    fn get_dist(&self) -> f64 {
        self.loc.get_dist()
    }
    fn get_center(&self) -> Rc<(f64,f64)> {
        self.loc.get_center()
    }
}
impl BoundedValue<f64,PositiveDist> for Stem {
    fn val_check(lower_bound:f64, upper_bound:f64, val: PositiveDist) -> Result<PositiveDist, Error> {
        let radius = val.dist();
        if radius >= lower_bound {
            if radius < upper_bound {
                Ok(val)
            } else {
                Err(Error::new(GallError::StemRadiusTooLong))
            }
        } else {
            Err(Error::new(GallError::StemRadiusTooShort))
        }
    }
    fn mut_val(&mut self, new_radius: PositiveDist) -> Result<(),Error> {
        let upper = match self.stem_type {
            J => self.get_parent_radius()*0.9,
            B => self.get_parent_radius()*0.95,
            S => new_radius.dist() + 1.0, //no upper limit
            Z => self.get_parent_radius()*2.0,
        };
        let radius = Stem::val_check(0.0, upper, new_radius)?.dist();
        self.radius.mut_val(radius)
    }
}
impl ChildCircle for Stem {
    fn get_parent_radius(&self) -> f64 {
        self.parent_radius.dist()
    }
    fn get_parent_thick(&self) -> f64 {
        self.parent_thickness.dist()
    }
    fn mut_stored_parent_radius(&mut self, new_radius:f64) -> Result<(),Error> {
        self.parent_radius.mut_val(new_radius)
    }
    fn mut_stored_parent_thick(&mut self, new_radius:f64) -> Result<(),Error> {
        self.parent_thickness.mut_val(new_radius)
    }
}
impl Circle for Stem {
    fn get_radius(&self) -> Rc<PositiveDist> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        self.radius.mut_val(new_radius)
    }
}
impl HollowCircle for Stem {
    fn get_thickness(&self) -> Rc<PositiveDist> {
        self.thickness.clone()
    }
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error> {
        self.thickness.mut_val(new_thick)
    }
}