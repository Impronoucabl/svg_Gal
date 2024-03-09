use std::cell::Cell;
use std::rc::Rc;

use crate::gall_circle::{Circle, HollowCircle};
use crate::gall_errors::Error;
use crate::gall_fn;
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_word::GallWord;

pub struct GallSentence {
    loc: GallLoc,
    pub words:Vec<GallWord>,
    //pub dash_pairs: Vec<CircleGallLine>,
    radius:Rc<Cell<f64>>,
    thickness: Rc<Cell<f64>>,
}

impl GallSentence {
    pub fn new(loc:GallLoc, radius:f64, thickness: f64) -> GallSentence {
        GallSentence{
            loc,
            words: Vec::new(),
            radius: Rc::new(Cell::new(radius)),
            thickness: Rc::new(Cell::new(thickness)),
        }
    }
    pub fn generate(&mut self, word_list:Vec<(String,usize)>) {
        let word_len = word_list.len();
        for (num,words) in word_list.into_iter().enumerate() {
            let (w_radius, w_thick, word_ang, dist) = gall_fn::default_layouts(word_len, num);
            //create word struct
            let loc = GallLoc::new(
                word_ang,
                dist,
                self.pos_ref(),
            );
            self.words.push(GallWord::new(words.0,words.1, loc, w_radius, w_thick));
        }
    }
}

impl HollowCircle for GallSentence {
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
impl Circle for GallSentence {
    fn radius(&self) -> f64 {
        self.radius.get()
    }
    fn get_radius(&self) -> Rc<Cell<f64>> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        //self.check_radius(new_radius)?;
        self.radius.set(new_radius);
        Ok(())
    }
}
impl Location for GallSentence {
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
impl PolarOrdinate for GallSentence {
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
    fn get_dist(&self) -> Rc<Cell<f64>> {
        self.loc.get_dist()
    }
}