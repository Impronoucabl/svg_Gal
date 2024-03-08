use std::f64::consts::TAU;

use crate::gall_errors::{Error, GallError};

//GallAng is a simple wrapper around Option<f64> to enforce
// the allowed range of 0 < angle < TAU
#[derive(PartialEq,Default,Clone,Copy)]
pub struct GallAng {
    angle: Option<f64>,
}

impl GallAng {
    pub fn new(angle: Option<f64>) -> GallAng {
        GallAng {
            angle: GallAng::constrain(angle)
        }
    }
    fn constrain(angle:Option<f64>) -> Option<f64> {
        match angle {
            Some(mut ang) => {
                while ang >= TAU {
                    ang -= TAU
                };
                while ang < 0.0 {
                    ang += TAU
                };
                Some(ang)
            }
            None => None
        }
    }
    pub fn mut_ang(&mut self, angle:Option<f64>){
        self.angle = GallAng::constrain(angle);
    }
    pub fn rotate(&mut self, angle:f64) -> Result<(),Error>{
        match self.angle {
            Some(ang) => Ok(self.mut_ang(Some(ang + angle))),
            None => Err(Error::new(GallError::AngleUndefined))
        } 
    }
    pub fn ang(self) -> Option<f64> {
        self.angle
    }    
}