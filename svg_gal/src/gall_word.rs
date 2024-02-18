use std::{cell::Cell, f64::consts::PI};
use std::rc::Rc;

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
    pub fn new(word:String, loc:GallLoc) -> GallWord {
        let len_guess = word.len();
        let mut tainer_vec = Vec::with_capacity(len_guess);
        GallWord{
            loc,
            tainer_vec,
            radius: Rc::new(Cell::new(350.0)),
            thickness: Rc::new(Cell::new(5.0))
        }
    } 
    fn populate(&self, word:String, tainer_vec:&mut Vec<GallTainer>) {
        //let con1 = GallTainer::new(,self);
        
        for cha in word.chars() {
            //cha
        }
    }
}