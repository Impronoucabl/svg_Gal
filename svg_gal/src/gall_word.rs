use std::{cell::Cell, f64::consts::PI};
use std::rc::Rc;

use crate::gall_circle::{Circle, HollowCircle, ParentCircle};
use crate::gall_errors::{Error, GallError};
use crate::gall_loc::GallLoc;
use crate::gall_tainer::GallTainer;

pub struct GallWord {
    loc: GallLoc,
    tainer_vec: Vec<GallTainer>,
    radius: Rc<Cell<f64>>,
    thickness: Rc<Cell<f64>>,
}

impl GallWord {
    pub fn new(text:String, loc:GallLoc) -> GallWord {
        let len_guess = text.len();
        let tainer_vec = Vec::with_capacity(len_guess);
        let mut word = GallWord{
            loc,
            tainer_vec,
            radius: Rc::new(Cell::new(350.0)),
            thickness: Rc::new(Cell::new(5.0))
        };
        word.populate(text);
        word
    } 
    fn populate(&mut self, word:String) {
        //let con1 = GallTainer::new(,self);
        
        for cha in word.chars() {
            //cha
        }
    }
    fn check_radius(&self, new_radius:f64) -> Result<(),Error> {
        //todo!();
        Ok(())
    }
}

impl ParentCircle for GallWord {}
impl HollowCircle for GallWord {
    fn thick(&self) -> f64 {
        self.thickness.get()
    }
    fn get_thickness(&self) -> Rc<Cell<f64>> {
        self.thickness.clone()
    }
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error> {
        self.thickness.set(new_thick);
        Ok(())
    }
}
impl Circle for GallWord {
    fn radius(&self) -> f64 {
        self.radius.get()
    }
    fn get_radius(&self) -> Rc<Cell<f64>> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        self.check_radius(new_radius)?;
        self.radius.set(new_radius);
        Ok(())
    }
}