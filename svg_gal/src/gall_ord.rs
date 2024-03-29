
use std::cell::Cell;
use std::rc::Rc;

use crate::gall_ang::GallAng;
use crate::gall_errors::{Error, GallError};

pub trait PolarOrdinate {
    fn mut_ang(&mut self, new_ang:f64);
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error>;
    fn ang(&self) -> Option<f64>;
    fn dist(&self) -> f64;
    fn get_dist(&self) -> Rc<Cell<f64>>;
    fn mut_ccw(&mut self, angle:f64) -> Result<(),Error> {
        match self.ang() {
            None => Err(Error::new(GallError::AngleUndefined)),
            Some(ang) => Ok(self.mut_ang(angle + ang))
        }
    }
    fn mut_cw(&mut self, angle:f64) -> Result<(),Error> {
        self.mut_ccw(-angle)
    }
    fn mut_ang_d(&mut self, new_dist:f64, new_ang:f64) {
        self.mut_ang(new_ang);
        let _ = self.mut_dist(new_dist);
    }
}

pub trait OrdHolder {
    fn ord(&self) -> &GallOrd;
    fn mut_ord(&mut self) -> &mut GallOrd;
}
#[derive(PartialEq,Default,Clone)]
pub struct GallOrd {
    angle: GallAng,
    distance: Rc<Cell<f64>>,
} 

impl GallOrd {
    pub fn new(angle:f64, distance:f64) -> GallOrd {
        GallOrd { 
            angle: GallAng::new(Some(angle)), 
            distance: Rc::new(Cell::new(distance)),
        }
    } 
    //Checks if distance is 0, and if it is, set angle to 0.
    fn ord_check(&mut self) -> Result<(), Error> {
        if self.distance.get() == 0.0 {
            self.angle = GallAng::new(None);
            Ok(())
        } else {
            match self.ang() {
                Some(_) => Ok(()),
                None => return Err(Error::new(GallError::AngleUndefined))
            }
        }
    }    
}

impl PolarOrdinate for GallOrd {
    fn ang(&self) -> Option<f64> {
        self.angle.ang()
    }
    fn dist(&self) -> f64 {
        self.distance.get()
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error>{
        if new_dist.is_sign_negative() {
            return Err(Error::new(GallError::NegativeDistanceErr))
        }
        self.distance.set(new_dist);            
        self.ord_check()
    }    
    fn mut_ang(&mut self, new_angle:f64) {
        self.angle.mut_ang(Some(new_angle))
    }
    fn get_dist(&self) -> Rc<Cell<f64>> {
        self.distance.clone()
    }
}

impl<T:OrdHolder> PolarOrdinate for T {
    fn mut_ang(&mut self, new_ang:f64) {
        self.mut_ord().mut_ang(new_ang)
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error> {
        self.mut_ord().mut_dist(new_dist)
    }
    fn ang(&self) -> Option<f64> {
        self.ord().ang()
    }
    fn dist(&self) -> f64 {
        self.ord().dist()
    }
    fn get_dist(&self) -> Rc<Cell<f64>> {
        self.ord().get_dist()
    }
}