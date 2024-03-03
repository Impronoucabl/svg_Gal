use std::cell::OnceCell;
use std::f64::consts::PI;

use crate::gall_errors::{Error, GallError};
use crate::gall_circle::{ChildCircle, Circle, HollowCircle};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_stem::Stem;
//O1 is on a letter, O2 is on a word
#[derive(PartialEq, Clone, Copy)]
pub enum VowelType {A,E,I,O1,O2,U}

pub struct GallVowel<'a> {
    loc: GallLoc,
    radius: OnceCell<f64>,
    thickness: OnceCell<f64>,
    parent_radius: &'a f64,
    parent_thickness: &'a f64,
    pub vowel_type: VowelType,
}

impl <'a>GallVowel<'a> {
    pub fn new(loc:GallLoc, radius: f64, thickness:f64, vowel_type: VowelType, parent_radius:&'a f64, parent_thickness: &'a f64) -> Option<GallVowel<'a>> {
        let rad = OnceCell::new();
        let thick = OnceCell::new();
        rad.set(radius);
        thick.set(thickness);
        Some(GallVowel {
            loc,
            radius: rad,
            thickness: thick,
            parent_radius,
            parent_thickness,
            vowel_type,
        })
    }
    pub fn center_on_stem(&mut self, stem:&Stem) {
        self.set_center(stem.pos_ref());
        _ = self.mut_dist(0.0);
    }
    pub fn o_attach_init(&mut self, stem:&Stem) {
        self.set_center(stem.get_center());
        _ = self.mut_dist(*stem.radius().unwrap());
        _ = self.mut_ccw(PI);
    }
}
impl Location for GallVowel<'_> {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    //This won't change any centers that have been cloned off this one
    fn set_center(&mut self, new_center:OnceCell<(f64, f64)>) {
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
impl PolarOrdinate for GallVowel<'_> {
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
        let p_rad = self.parent_radius();
        match self.vowel_type {
            VowelType::E|VowelType::I|VowelType::U => {
                if new_dist == 0.0 {
                    self.loc.mut_dist(0.0)
                } else if new_dist == p_rad {
                    self.loc.mut_dist(p_rad)
                } else {
                    Err(Error::new(GallError::InvalidVowelDist))
                }
            },
            VowelType::A => {
                if new_dist < p_rad + self.radius().unwrap() + self.parent_thick() + self.thick().unwrap() {
                    Err(Error::new(GallError::InvalidVowelDist))
                } else {
                    self.loc.mut_dist(new_dist)
                }
            },
            VowelType::O1 => {//attached O
                //TODO: Confirm if works.
                if new_dist > p_rad + self.parent_thick() {
                    return Err(Error::new(GallError::InvalidVowelDist));
                }
                if new_dist < p_rad - self.parent_thick() {
                    Err(Error::new(GallError::InvalidVowelDist))
                } else {
                    self.loc.mut_dist(new_dist)
                }
            },
            VowelType::O2 => {
                if new_dist > p_rad - self.parent_thick() - self.radius().unwrap() - self.thick().unwrap() {
                    Err(Error::new(GallError::InvalidVowelDist))
                } else {
                    self.loc.mut_dist(new_dist)
                }
            },
        }
    }
    fn get_ang(&self) -> OnceCell<f64> {
        self.loc.get_ang()
    }
    fn get_dist(&self) -> OnceCell<f64> {
        self.loc.get_dist()
    }
}
impl ChildCircle for GallVowel<'_> {
    fn get_parent_radius(&self) -> &f64 {
        self.parent_radius
    }
    fn get_parent_thick(&self) -> &f64 {
        self.parent_thickness
    }

    fn parent_radius(&self) -> f64 {
        *self.parent_radius
    }
    fn parent_thick(&self) -> f64 {
        *self.parent_thickness
    }
}
impl Circle for GallVowel<'_> {
    fn get_radius(&self) -> OnceCell<f64> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        if new_radius < self.parent_thick() + self.thick().unwrap() {
            Err(Error::new(GallError::VowelRadiusTooShort))
        } else if new_radius >= self.parent_radius() - self.parent_thick() {
            Err(Error::new(GallError::VowelRadiusTooLong))
        } else {
            self.radius.set(new_radius);
            Ok(())
        }
    }
    fn radius(&self) -> Option<&f64> {
        self.radius.get()
    }
}
impl HollowCircle for GallVowel<'_> {
    fn get_thickness(&self) -> OnceCell<f64> {
        self.thickness.clone()
    }
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error> {
        self.thickness.set(new_thick);
        Ok(())
        //TODO:Thickness checks
    }
    fn thick(&self) -> Option<&f64> {
        self.thickness.get()
    }
}