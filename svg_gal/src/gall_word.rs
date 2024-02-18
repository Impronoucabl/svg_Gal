use std::f64::consts::TAU;
use std::{cell::Cell, f64::consts::PI};
use std::rc::Rc;

use crate::gall_circle::{Circle, HollowCircle, ParentCircle};
use crate::gall_errors::{Error, GallError};
use crate::gall_fn::{self, LetterMark};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_stem::StemType;
use crate::gall_tainer::GallTainer;

pub struct GallWord {
    loc: GallLoc,
    tainer_vec: Vec<GallTainer>,
    radius: Rc<Cell<f64>>,
    thickness: Rc<Cell<f64>>,
}

impl GallWord {
    pub fn new(text:String, len_guess:usize, loc:GallLoc) -> GallWord {
        let tainer_vec = Vec::with_capacity(len_guess);
        let mut word = GallWord{
            loc,
            tainer_vec,
            radius: Rc::new(Cell::new(350.0)),
            thickness: Rc::new(Cell::new(5.0))
        };
        word.populate(text, len_guess)
    } 
    fn populate(mut self, word:String, len_guess:usize) -> GallWord {
        let mut clock = 0.0;
        let tainer_ang = TAU/(len_guess as f64); 
        let mut con_count:usize = 0;
        let mut con = self.get_con();
        for cha in word.chars() {
            let (l_mark, repeat) = gall_fn::stem_lookup(&cha);
            if con.stem_type().is_none() && con.is_empty() {
                match l_mark {
                    LetterMark::Stem(stem) => {con.init(stem, con_count, tainer_ang)}
                    _ => {}
                }
            } else {
                match &l_mark {
                    LetterMark::Stem(stem) => if Some(stem) != con.stem_type() {
                        self.tainer_vec.push(con);
                        con = self.get_con();
                        con.init(*stem, con_count, tainer_ang);
                    }
                    LetterMark::Digit(_) => {todo!()},
                    LetterMark::GallMark => {}, 
                }
            } //At this point the con tainer should be initialised.
            con.populate(l_mark, &self);
            println!("{}",con_count);
        }
        self
    }
    fn get_con(&self) -> GallTainer {
        GallTainer::new(LetterMark::GallMark, self)
    }
    fn check_radius(&self, new_radius:f64) -> Result<(),Error> {
        //todo!();
        Ok(())
    }
}

//impl ParentCircle for GallWord {}
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
impl Location for GallWord {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    fn set_center(&mut self, new_center:Rc<Cell<(f64,f64)>>) {
        self.loc.set_center(new_center)
    }
    fn get_center(&self) -> Rc<Cell<(f64,f64)>> {
        self.loc.get_center()
    }
    fn x(&self) -> f64 {
        self.loc.x()
    }
    fn y(&self) -> f64 {
        self.loc.y()
    }
    fn pos_ref(&self) -> Rc<Cell<(f64,f64)>> {
        self.loc.pos_ref()
    }
}
impl PolarOrdinate for GallWord {
    fn mut_ang(&mut self, new_ang:f64) {
        self.loc.mut_ang(new_ang)
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error> {
        self.loc.mut_dist(new_dist)
    }
    fn ang(&self) -> Option<f64> {
        self.loc.ang()
    }
    fn dist(&self) -> f64 {
        self.loc.dist()
    }
}