use std::rc::Rc;

use crate::gall_errors::{Error, GallError};
use crate::gall_ord::{BoundedValue, CenterOrd, GallAng, GallLoc, LocMover, PositiveDist};
use crate::gall_struct::{ChildCircle, Circle, HollowCircle};
//O1 is on a letter, O2 is on a word
pub enum VowelType {A,E,I,O1,O2,U}

pub struct GallVowel {
    loc: GallLoc,
    radius: Rc<PositiveDist>,
    thickness: Rc<PositiveDist>,
    parent_radius: Rc<PositiveDist>,
    parent_thickness: Rc<PositiveDist>,
    pub vowel_type: VowelType,
}

impl LocMover for GallVowel {
    //case for EIU on letter/word not delt with, since a None angle short circuits to desired behaviour.
    fn mut_ang(&mut self, angle:GallAng) -> Result<(), Error> {
        self.loc.mut_ang(angle)
    }
    fn mut_dist(&mut self, new_dist:f64) -> Result<(), Error> {
        let p_rad = self.parent_radius.dist();
        match self.vowel_type {
            VowelType::E|VowelType::I|VowelType::U => {
                match new_dist {
                    0.0 => self.loc.mut_dist(0.0),
                    p_rad => self.loc.mut_dist(p_rad),
                    _ => Err(Error::new(GallError::InvalidVowelDist)),
                }
            },
            A => {
                if new_dist < p_rad + self.parent_thickness.dist() + self.thickness.dist() {
                    Err(Error::new(GallError::InvalidVowelDist))
                } else {
                    self.loc.mut_dist(new_dist)
                }
            },
            O1 => {
                if new_dist > p_rad + self.parent_thickness.dist() {
                    return Err(Error::new(GallError::InvalidVowelDist));
                }
                if new_dist < p_rad - self.parent_thickness.dist() {
                    Err(Error::new(GallError::InvalidVowelDist))
                } else {
                    self.loc.mut_dist(new_dist)
                }
            },
            O2 => {
                if new_dist > p_rad - self.parent_thickness.dist() - self.radius.dist() - self.thickness.dist() {
                    Err(Error::new(GallError::InvalidVowelDist))
                } else {
                    self.loc.mut_dist(new_dist)
                }
            },
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

impl BoundedValue<f64,PositiveDist> for GallVowel {
    fn val_check(lower_bound:f64, upper_bound:f64, val: PositiveDist) -> Result<PositiveDist, Error> {
        let radius = val.dist();
        if radius >= lower_bound {
            if radius < upper_bound {
                Ok(val)
            } else {
                Err(Error::new(GallError::VowelRadiusTooLong))
            }
        } else {
            Err(Error::new(GallError::VowelRadiusTooShort))
        }
    }

    fn mut_val(&mut self, new_radius: PositiveDist) -> Result<(),Error> {
        let radius = GallVowel::val_check(
            self.parent_thickness.dist() + self.thickness.dist(), 
            self.parent_radius.dist()- self.parent_thickness.dist(),
            new_radius
        )?.dist();
        self.radius.mut_val(radius)
    }
}

impl ChildCircle for GallVowel {
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

impl Circle for GallVowel {
    fn get_radius(&self) -> Rc<PositiveDist> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        self.radius.mut_val(new_radius)
    }
}

impl HollowCircle for GallVowel {
    fn get_thickness(&self) -> Rc<PositiveDist> {
        self.thickness.clone()
    }
}