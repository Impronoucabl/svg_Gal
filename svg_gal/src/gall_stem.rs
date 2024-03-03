use std::cell::OnceCell;

use crate::gall_errors::{Error, GallError};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_circle::{ChildCircle, Circle, HollowCircle};
use crate::gall_ord::PolarOrdinate;

#[derive(PartialEq,Clone,Copy)]
pub enum StemType {J,B,S,Z}

pub struct Stem<'a> {
    loc: GallLoc,
    radius: OnceCell<f64>,
    thickness: OnceCell<f64>,
    parent_radius: Option<&f64>,
    parent_thickness: Option<&f64>,
    pub stem_type: StemType,
}
impl <'a>Stem<'a> {
    pub fn new(loc:GallLoc, radius: f64, thickness:f64, stem_type: StemType, parent_radius: Option<&f64>, parent_thickness: Option<&f64>) -> Option<Stem<'a>> {
        let rad = OnceCell::new();
        let thick = OnceCell::new();
        rad.set(radius);
        thick.set(thickness);
        Some(Stem {
            loc,
            radius: rad,
            thickness: thick,
            parent_radius,
            parent_thickness,
            stem_type,
        })
    }
    fn radius_limits(&self) -> Option<(f64,f64)> {
        Some(match &self.stem_type {
            StemType::J => (self.parent_inner() - self.dist()? - 2.0*self.thick()?,0.0),
            StemType::B => (self.parent_outer() + self.parent_thick() - self.dist()? - self.thick()?, 
                self.parent_outer() - self.dist()? - self.thick()?),
            StemType::S => (f64::INFINITY,0.0),
            StemType::Z => (2.0*self.parent_outer() - self.parent_radius() + self.dist()? - self.thick()?, 0.0),
        })
    }
    fn dist_limits(&self) -> Option<(f64,f64)> {
        Some(match &self.stem_type {
            StemType::J => (self.parent_inner() - self.outer_radius()? - self.thick()?, 0.0),
            StemType::B => (self.parent_outer() + self.parent_thick() - self.outer_radius()?, 
                self.parent_outer() - self.outer_radius()?),
            StemType::S => (self.parent_inner() + self.inner_radius()?,self.parent_inner()),
            StemType::Z => (self.parent_outer(),self.parent_inner()),
        })
    }
    fn thick_limits(&self) -> Option<(f64,f64)> {
        //todo!()
        Some((2.0,200.0))
    }
    fn check_radius(&self, test_val:f64) -> Result<(), Error> {
        if let Some((upper_limit, lower_limit)) = self.radius_limits() {
            if test_val > upper_limit {
                Err(Error::new(GallError::RadiusTooLong))
            } else if test_val < lower_limit {
                Err(Error::new(GallError::RadiusTooShort))
            } else {
                Ok(())
            }
        } else {
            Err(Error::new(GallError::ValueNotSet))
        }
        
    }
    fn check_dist(&self, test_val:f64) -> Result<(), Error> {
        if let Some((upper_limit, lower_limit)) = self.dist_limits() {
            if test_val > upper_limit {
                Err(Error::new(GallError::DistTooLong))
            } else if test_val < lower_limit {
                Err(Error::new(GallError::DistTooShort))
            } else {
                Ok(())
            }
        } else {
            Err(Error::new(GallError::ValueNotSet))
        }
    }
    fn check_thick(&self, test_val:f64) -> Result<(), Error> {
        if let Some((upper_limit, lower_limit)) = self.thick_limits() {
            if test_val > upper_limit {
                Err(Error::new(GallError::TooThick))
            } else if test_val < lower_limit {
                Err(Error::new(GallError::NotThickEnough))
            } else {
                Ok(())
            }
        } else {
            Err(Error::new(GallError::ValueNotSet))
        }
    }
}
impl PolarOrdinate for Stem<'_> {
    fn ang(&self) -> Option<&f64> {
        self.loc.ang()
    }
    fn dist(&self) -> Option<&f64> {
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
    fn get_ang(&self) -> OnceCell<f64> {
        self.loc.get_ang()
    }
    fn get_dist(&self) -> OnceCell<f64> {
        self.loc.get_dist()
    }
}
impl Location for Stem<'_> {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    fn set_center(&mut self, new_center:OnceCell<(f64,f64)>) {
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
    fn pos_ref(&self) -> OnceCell<(f64,f64)> {
        self.loc.pos_ref()
    }
    fn center_ords(&self) -> (f64,f64) {
        self.loc.center_ords()
    }
}
impl ChildCircle for Stem<'_> {
    fn parent_radius(&self) -> f64 {
        *self.parent_radius.unwrap()
    }
    fn parent_thick(&self) -> f64 {
        *self.parent_thickness.unwrap()
    }
    fn get_parent_radius(&self) -> Option<&f64> {
        self.parent_radius
    }
    fn get_parent_thick(&self) -> Option<&f64> {
        self.parent_thickness
    }
}
impl Circle for Stem<'_> {
    fn radius(&self) -> Option<&f64> {
        self.radius.get()
    }
    fn get_radius(&self) -> OnceCell<f64> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        self.check_radius(new_radius)?;
        self.radius.set(new_radius);
        Ok(())
    }
}
impl HollowCircle for Stem<'_> {
    fn get_thickness(&self) -> OnceCell<f64> {
        self.thickness.clone()
    }
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error> {
        self.check_thick(new_thick)?;
        self.thickness.set(new_thick);
        Ok(())
    }
    fn thick(&self) -> Option<&f64> {
        self.thickness.get()
    }
}