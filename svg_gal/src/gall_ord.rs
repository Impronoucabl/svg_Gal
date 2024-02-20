
use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_8, TAU};

use crate::gall_ang::GallAng;
use crate::gall_errors::{Error, GallError};

pub trait PolarOrdinate {
    fn mut_ang(&mut self, new_ang:f64);
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error>;
    fn ang(&self) -> Option<f64>;
    fn dist(&self) -> f64;
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

#[derive(PartialEq,Default,Clone,Copy)]
pub struct GallOrd {
    angle: GallAng,
    distance: f64,
} 

impl GallOrd {
    pub fn new(angle:f64, distance:f64) -> GallOrd {
        GallOrd { 
            angle: GallAng::new(Some(angle)), 
            distance
        }
    } 
    //Checks if distance is 0
    fn ord_check(&mut self) -> Result<(), Error> {
        self.angle = match self.distance {
            0.0 => GallAng::new(None),
            Other => match self.ang() {
                Some(_) => self.angle,
                None => return Err(Error::new(GallError::AngleUndefined))
            },
        };
        Ok(())
    }    
    //mut self can't fail since ang can't be undef
    pub fn mut_self(&mut self, new_dist:f64, new_ang:f64) {
        self.mut_ang(new_ang);
        self.mut_dist(new_dist);
    }
}

impl PolarOrdinate for GallOrd {
    fn ang(&self) -> Option<f64> {
        self.angle.ang()
    }
    fn dist(&self) -> f64 {
        self.distance
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error>{
        if new_dist.is_sign_negative() {
            return Err(Error::new(GallError::NegativeDistanceErr))
        }
        self.distance = new_dist;            
        self.ord_check()
    }    
    fn mut_ang(&mut self, new_angle:f64) {
        self.angle.mut_ang(Some(new_angle))
    }
}

