
use std::cell::OnceCell;
use std::rc::Rc;

use crate::gall_ang::GallAng;
use crate::gall_errors::{Error, GallError};

pub trait PolarOrdinate {
    fn get_ang(&self) -> OnceCell<f64>;
    fn get_dist(&self) -> OnceCell<f64>;
    fn mut_ang(&mut self, new_ang:f64);
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error>;
    fn ang(&self) -> Option<&f64>;
    fn dist(&self) -> Option<&f64>;
    fn mut_ccw(&mut self, angle:f64) -> Result<(),Error> {
        match self.ang() {
            None => Err(Error::new(GallError::AngleUndefined)),
            Some(ang) => Ok(self.mut_ang(angle + ang))
        }
    }
    fn mut_cw(&mut self, angle:f64) -> Result<(),Error> {
        self.mut_ccw(-angle)
    }
}

#[derive(PartialEq,Default,Clone)]
pub struct GallOrd {
    angle: GallAng,
    distance: OnceCell<f64>,
} 

impl GallOrd {
    pub fn new(angle:f64, distance:f64) -> GallOrd {
        let dist = OnceCell::new();
        dist.set(distance);
        GallOrd { 
            angle: GallAng::new(Some(angle)), 
            distance: dist
        }
    }
    pub fn new_ref(angle:OnceCell<f64>, distance:OnceCell<f64>) -> GallOrd {
        let ang = GallAng::from_ref(angle);
        GallOrd { angle:ang , distance }
    } 
    //Checks if distance is 0, and if it is, set angle to 0.
    fn ord_check(&mut self) -> Result<(), Error> {
        if let Some(&dist) = self.dist() {
            if dist == 0.0 {
                _ = self.angle.take_ang();
                Ok(())
            } else {
                match self.ang() {
                    Some(_) => Ok(()),
                    None => Err(Error::new(GallError::AngleUndefined))
                }
            }
        } else {
            Err(Error::new(GallError::ValueNotSet))
        }
    }  
    pub fn set_ang(&mut self, new_ang_ref: OnceCell<f64>) {
        self.angle.set_ang(new_ang_ref)
    }
    pub fn take_ang(&mut self) -> f64 {
        self.angle.take_ang()
    }  
}

impl PolarOrdinate for GallOrd {
    fn ang(&self) -> Option<&f64> {
        self.angle.ang() 
    }
    fn dist(&self) -> Option<&f64> {
        self.distance.get()
    }
    fn get_dist(&self) -> OnceCell<f64> {
        self.distance.clone()
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error>{
        if new_dist.is_sign_negative() {
            return Err(Error::new(GallError::NegativeDistanceErr))
        }
        _ = self.distance.take();
        self.distance.set(new_dist)
            .expect("this shouldn't happen.");            
        self.ord_check()
    }    
    fn mut_ang(&mut self, new_angle:f64) {
        self.angle.mut_ang(Some(new_angle))
    }
    fn get_ang(&self) -> OnceCell<f64> {
        self.angle.get_ang()
    }
}

