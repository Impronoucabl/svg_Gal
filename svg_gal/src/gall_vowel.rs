use std::cell::OnceCell;
use std::f64::consts::PI;
use std::rc::Rc;

use crate::gall_errors::{Error, GallError};
use crate::gall_circle::{ChildCircle, Circle, HollowCircle};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_stem::Stem;
//O1 is on a letter, O2 is on a word
#[derive(PartialEq, Clone, Copy)]
pub enum VowelType {A,E,I,O1,O2,U}

pub struct GallVowel {
    loc: GallLoc,
    radius: Rc<OnceCell<f64>>,
    thickness: Rc<OnceCell<f64>>,
    parent_radius: Rc<OnceCell<f64>>,
    parent_thickness: Rc<OnceCell<f64>>,
    pub vowel_type: VowelType,
}

impl GallVowel {
    pub fn new<T:HollowCircle>(loc:GallLoc, radius: f64, thickness:f64, vowel_type: VowelType, parent:&T) -> GallVowel {
        let rad = Rc::new(OnceCell::new());
        let thick = Rc::new(OnceCell::new());
        rad.set(radius);
        thick.set(thickness);
        let parent_radius = parent.get_radius().clone();
        let parent_thickness = parent.get_thickness().clone();
        GallVowel {
            loc,
            radius: rad,
            thickness: thick,
            parent_radius,
            parent_thickness,
            vowel_type,
        }
    }
    pub fn center_on_stem(&mut self, stem:&Stem) {
        self.set_center(stem.pos_ref());
        _ = self.mut_dist(0.0);
    }
    pub fn o_attach_init(&mut self, stem:&Stem) {
        self.set_center(stem.get_center());
        _ = self.mut_dist(stem.radius().unwrap());
        _ = self.mut_ccw(PI);
    }
}
impl Location for GallVowel {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    //This won't change any centers that have been cloned off this one
    fn set_center(&mut self, new_center:Rc<OnceCell<(f64, f64)>>) {
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
impl PolarOrdinate for GallVowel {
    fn ang(&self) -> Option<f64> {
        self.loc.ang()
    }
    fn dist(&self) -> Option<f64> {
        self.loc.dist()
    }
    fn mut_ang(&mut self, angle:f64) {
        self.loc.mut_ang(angle)
    }
    fn mut_dist(&mut self, new_dist:f64) -> Result<(), Error> {
        let p_rad = self.parent_radius().unwrap();
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
                if new_dist < p_rad + self.radius().unwrap() + self.parent_thick().unwrap() + self.thick().unwrap() {
                    Err(Error::new(GallError::InvalidVowelDist))
                } else {
                    self.loc.mut_dist(new_dist)
                }
            },
            VowelType::O1 => {//attached O
                //TODO: Confirm if works.
                if new_dist > p_rad + self.parent_thick().unwrap() {
                    return Err(Error::new(GallError::InvalidVowelDist));
                }
                if new_dist < p_rad - self.parent_thick().unwrap() {
                    Err(Error::new(GallError::InvalidVowelDist))
                } else {
                    self.loc.mut_dist(new_dist)
                }
            },
            VowelType::O2 => {
                if new_dist > p_rad - self.parent_thick().unwrap() - self.radius().unwrap() - self.thick().unwrap() {
                    Err(Error::new(GallError::InvalidVowelDist))
                } else {
                    self.loc.mut_dist(new_dist)
                }
            },
        }
    }
    fn get_ang(&self) -> Rc<OnceCell<f64>> {
        self.loc.get_ang()
    }
    fn get_dist(&self) -> Rc<OnceCell<f64>> {
        self.loc.get_dist()
    }
}
impl ChildCircle for GallVowel {
    fn get_parent_radius(&self) -> Rc<OnceCell<f64>> {
        self.parent_radius.clone()
    }
    fn get_parent_thick(&self) -> Rc<OnceCell<f64>> {
        self.parent_thickness.clone()
    }

    fn parent_radius(&self) -> Option<f64> {
        match self.parent_radius.get() {
            Some(rad) => Some(*rad),
            None => None,
        }
    }
    fn parent_thick(&self) -> Option<f64> {
        match self.parent_thickness.get() {
            Some(thick) => Some(*thick),
            None => None,
        }
    }
}
impl Circle for GallVowel {
    fn get_radius(&self) -> Rc<OnceCell<f64>> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        if new_radius < self.parent_thick().unwrap() + self.thick().unwrap() {
            Err(Error::new(GallError::VowelRadiusTooShort))
        } else if new_radius >= self.parent_radius().unwrap() - self.parent_thick().unwrap() {
            Err(Error::new(GallError::VowelRadiusTooLong))
        } else {
            self.radius.set(new_radius);
            Ok(())
        }
    }
    fn radius(&self) -> Option<f64> {
        match self.radius.get() {
            Some(rad) => Some(*rad),
            None => None,
        }
    }
}
impl HollowCircle for GallVowel {
    fn get_thickness(&self) -> Rc<OnceCell<f64>> {
        self.thickness.clone()
    }
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error> {
        self.thickness.set(new_thick);
        Ok(())
        //TODO:Thickness checks
    }
    fn thick(&self) -> Option<f64> {
        match self.thickness.get() {
            Some(thick) => Some(*thick),
            None => None,
        }
    }
}