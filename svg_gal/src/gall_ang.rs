use std::f64::consts::TAU;

use crate::gall_errors::{Error, GallError};

//GallAng is a simple wrapper around Option<f64> to enforce
// the allowed range of 0 < angle < TAU
#[derive(PartialEq,Default,Clone,Copy)]
pub struct GallAng {
    angle: Option<f64>,
}

pub fn constrain_opt(angle:Option<f64>) -> Option<f64> {
    if let Some(ang) = angle {
        Some(constrain(ang))
    } else {
        None
    }
}

pub fn constrain(mut ang:f64) -> f64 {
    while ang >= TAU {
        ang -= TAU
    };
    while ang < 0.0 {
        ang += TAU
    };
    ang
}

impl GallAng {
    pub fn new(angle: Option<f64>) -> GallAng {
        GallAng {
            angle: constrain_opt(angle)
        }
    }
    pub fn mut_ang(&mut self, angle:Option<f64>){
        self.angle = constrain_opt(angle);
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