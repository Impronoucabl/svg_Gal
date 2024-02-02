use std::f64::consts::{FRAC_PI_2, PI};
use std::rc::Rc;
use std::error::Error;

use crate::gall_errors;
use crate::gall_ord::{BoundedValue, GallAng, GallLoc, LocMover, PositiveDist};
use crate::gall_struct::{ChildCircle, Circle};

#[derive(PartialEq)]
pub enum StemType {J,B,S,Z}

pub struct Stem {
    loc: GallLoc,
    radius: PositiveDist,
    thickness: PositiveDist,
    parent_radius: Rc<PositiveDist>,
    parent_thickness: Rc<PositiveDist>,
    pub stem_type: StemType,
}

impl LocMover for Stem {
    fn mut_ang(&mut self, angle:GallAng) -> Result<(), Box<dyn Error>> {
        self.loc.mut_ang(angle)
    }

    fn mut_dist(&mut self, new_dist:f64) -> Result<(), Box<dyn Error>> {
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
                Err(Box::new(gall_errors::StemDistTooShort))
            }
        } else {
            Err(Box::new(gall_errors::StemDistTooLong))
        }
    }

    fn mut_center(&mut self, new_center:(f64,f64)) {
        self.loc.mut_center(new_center)
    }

    fn get_ang(&self) -> GallAng {
        self.loc.get_ang()
    }

    fn get_dist(&self) -> f64 {
        self.loc.get_dist()
    }

    fn get_center(&self) -> (f64,f64) {
        self.loc.get_center()
    }
}

impl BoundedValue<f64,PositiveDist> for Stem {
    fn val_check(lower_bound:f64, upper_bound:f64, val: PositiveDist) -> Result<PositiveDist, Box<dyn Error>> {
        let radius = val.dist();
        if radius >= lower_bound {
            if radius < upper_bound {
                Ok(val)
            } else {
                Err(Box::new(gall_errors::StemRadiusTooLong))
            }
        } else {
            Err(Box::new(gall_errors::StemRadiusTooShort))
        }
    }
    fn mut_val(&mut self, new_radius: PositiveDist) -> Result<(),Box<dyn Error>> {
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
    fn mut_stored_parent_radius(&mut self, new_radius:f64) -> Result<(),Box<dyn Error>> {
        self.parent_radius.mut_val(new_radius)
    }
    fn mut_stored_parent_thick(&mut self, new_radius:f64) -> Result<(),Box<dyn Error>> {
        self.parent_thickness.mut_val(new_radius)
    }
}

impl Circle for Stem {
    fn get_outer_radius(&self) -> PositiveDist {
        self.radius + self.thickness
    }
    fn get_inner_radius(&self) -> PositiveDist {
        self.radius - self.thickness
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Box<dyn Error>> {
        self.radius.mut_val(new_radius)
    }
}