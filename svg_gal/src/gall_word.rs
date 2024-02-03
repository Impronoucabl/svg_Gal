use std::error::Error;
use std::rc::Rc;

use crate::gall_fn::{self, LetterType};
use crate::gall_ord::{BoundedValue, CenterOrd, GallAng, GallLoc, LocMover, PositiveDist};
use crate::gall_stem::Stem;
use crate::gall_struct::{Circle, HollowCircle};
use crate::gall_tainer::GallTainer;

pub struct GallWord2 {
    loc: GallLoc,
    tainer_vec: Vec<GallTainer>,
    radius: Rc<PositiveDist>,
    thickness: Rc<PositiveDist>,
}
impl HollowCircle for GallWord2 {
    fn get_thickness(&self) -> Rc<PositiveDist> {
        self.thickness.clone()
    }
}
impl GallWord2 {
    pub fn new(loc:GallLoc, radius: PositiveDist, thickness: PositiveDist) -> GallWord2 {
        let tainer_vec = Vec::with_capacity(1);
        let radius = Rc::new(radius);
        let thickness = Rc::new(thickness);
        GallWord2{
            loc,
            tainer_vec,
            radius,
            thickness
        }
    }
    fn reset(self) -> GallWord2 {
        GallWord2::new(self.loc,*self.radius,*self.thickness)
    }
    fn make_tainer(&mut self) -> GallTainer {
        let mut syl = GallTainer::new(gall_fn::LetterType::Punctuation,*self);
        self.tainer_vec.push(syl);
        syl
    }
    //Assume String already parsed
    pub fn populate(&mut self, text: String) -> Result<(), Error> {
        let syl = self.make_tainer();
        let mut text_iter = text.chars(); 
        while let Some(letter) = text_iter.next() {
            //lookup letter
            let (stem, repeat) = gall_fn::stem_lookup(&letter);
            match stem {
                LetterType::BStem|LetterType::JStem|LetterType::SStem|LetterType::ZStem => {
                    let stem1 = Stem::new(loc,radius,thickness,stem,self)?;
                    //attempt to add stem to tainer
                    match syl.mut_stemtype(stem) {
                        Ok(_) => {},
                        Err(_) => { //TODO create crate errors
                            syl = self.make_tainer();
                        }
                    }
                }
                LetterType::A|LetterType::EIU|LetterType::O1|LetterType::O2 => {
                    //GallVowel
                }
            }
            //create stem
            let stem1 = Stem::new(loc,radius,thickness,stem,self)?;
            //attempt to add stem to tainer
            match syl.mut_stemtype(stem) {
                Ok(_) => {},
                Err(_) => { //TODO create crate errors
                    syl = self.make_tainer();
                }
            }
            syl.add_stem(stem1); 
            //let syl = GallTainer::new(stem,*self);
            //push tainer into word
            syl.add_stem(stem1)?;
            if repeat {
                let stem_copy = Stem::new(loc,radius,thickness,stem,self)?;
                syl.add_stem(stem_copy)?;
            }
            //add nodes
            //add dots

            if let Some(letter) = text_iter.next() {
                //lookup letter
                let (stem2, repeat2) = gall_fn::stem_lookup(&letter);
                //check if can be added to current tainer
                
                
                if stem == stem2 {
                    //create stem
                    //add to current tainer
                } else {
                    continue
                }
            } else {
                //end of text
                break
            }
        }
        Ok(())
    }
}
impl LocMover for GallWord2 {
    fn mut_ang(&mut self, angle:GallAng) -> Result<(), Error> {
        self.loc.mut_ang(angle)
    }
    fn mut_dist(&mut self, new_dist:f64) -> Result<(), Error> {
        self.loc.mut_dist(new_dist)
    }
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    //This won't change any centers that have been cloned off this one
    fn set_center(&mut self, new_center:CenterOrd) {
        self.loc.set_center(new_center)
    }
    fn get_ang(&self) -> GallAng {
        self.loc.get_ang()
    }
    fn get_dist(&self) -> f64 {
        self.loc.get_dist()
    }
    fn get_center(&self) -> Rc<(f64,f64)> {
        self.loc.get_center()
    }
}
impl Circle for GallWord2 {
    fn get_radius(&self) -> Rc<PositiveDist> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        self.radius.mut_val(new_radius)
    }
}
