use std::{cell::OnceCell, f64::consts::TAU, rc::Rc};

use crate::gall_errors::{Error, GallError};

//GallAng is a simple wrapper around Option<f64> to enforce
// the allowed range of 0 < angle < TAU
#[derive(PartialEq,Default,Clone)]
pub struct GallAng {
    angle: Rc<OnceCell<f64>>,
}

impl GallAng {
    pub fn new(angle: Option<f64>) -> GallAng {
        let val = OnceCell::new();
        if let Some(ang) = angle {
            val.set(GallAng::constrain(ang));
        }
        GallAng {
            angle: Rc::new(val)
        }
    }
    pub fn from_ref(angle:Rc<OnceCell<f64>>) -> GallAng {
        GallAng {
            angle
        }
    }
    fn constrain(mut ang:f64) -> f64 {
        while ang >= TAU {
            ang -= TAU
        };
        while ang < 0.0 {
            ang += TAU
        };
        ang
    }
    pub fn mut_ang(&mut self, angle:Option<f64>){
        _ = self.angle.take();
        if let Some(ang) = angle {
            self.angle.set(GallAng::constrain(ang));
        }
    }
    pub fn rotate(&mut self, angle:f64) -> Result<(),Error>{
        match self.angle.get() {
            Some(ang) => Ok(self.mut_ang(Some(ang + angle))),
            None => Err(Error::new(GallError::AngleUndefined))
        } 
    }
    pub fn ang(&self) -> Option<&f64> {
        self.angle.get()
    }    
    pub fn get_ang(&self) -> Rc<OnceCell<f64>>{
        self.angle
    }
    pub fn set_ang(&mut self, new_ang_ref:Rc<OnceCell<f64>>) {
        //_ = self.angle.take(); //For debugging
        self.angle = new_ang_ref
    }
}