use std::rc::Rc;

use crate::gall_errors::{Error, GallError};
use crate::gall_fn;
use crate::gall_ord::{BoundedValue, CenterOrd, GallAng, GallLoc, LocMover, PositiveDist};
use crate::gall_stem::{Stem, StemType};
use crate::gall_struct::{Circle, HollowCircle, LetterMark};
use crate::gall_tainer::GallTainer;
use crate::gall_vowel::VowelType;

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
        let mut syl = GallTainer::new(LetterMark::GallMark,*self);
        self.tainer_vec.push(syl);
        syl
    }
    fn make_letter_loc(&self, stem:StemType, ang: f64, center: Rc<(f64,f64)>) -> GallLoc {
        let multiplier = match stem {
            J => 0.7,
            B => 0.9,
            _ => 1.0,
        };
        GallLoc::new(
            ang,
            self.get_radius()* multiplier,
            center,
        )
    }
    fn populate_stem(&mut self, s_type:StemType, syl:GallTainer, clock_loc:GallLoc, step_angle:f64, repeat: bool) -> Result<GallTainer,Error> {
        let letter_loc = self.make_letter_loc(s_type, clock_loc.get_ang().ang, clock_loc.get_center());
        let stem = Stem::new(letter_loc,100.0,2.0,s_type,self)?;
        match syl.mut_stemtype(Some(s_type)) {
            Ok(_) => {},
            Err(E) => {
                if E.error_type == GallError::BadTainerStem {
                    self.tainer_vec.push(syl);
                    clock_loc.rotate_ccw(step_angle);
                    letter_loc.rotate_ccw(step_angle);
                    syl = self.make_tainer();
                    syl.mut_stemtype(Some(s_type))?;
                } else {
                    Err(E)
                }                            
            },
        }
        syl.add_stem(stem)?;
        if repeat {
            let repeat_loc = self.make_letter_loc(s_type, clock_loc.get_ang().ang, clock_loc.get_center());
            let repeat_stem = Stem::new(repeat_loc,110.0,2.0,s_type,self)?;
            syl.add_stem(repeat_stem)?;
        }
        Ok(syl)
    } 
    fn populate_vowel(&mut self, v_type: VowelType, syl: GallTainer, clock_loc: GallLoc, step_angle: f64, repeat:bool) -> Result<GallTainer, Error> {
        
    }
    //Assume String already parsed
    pub fn populate(&mut self, text: String) -> Result<(), Error> {
        //text.len() is byte len, not # of chars
        let letter_sep_ang = std::f64::consts::TAU/(text.len() as f64);
        let clock_loc = GallLoc::new( 
            0.0, 
            self.get_radius(), 
            self.loc.svg_ord(), 
        );
        
        let syl = self.make_tainer();
        let mut text_iter = text.chars(); 
        while let Some(letter) = text_iter.next() {
            //lookup letter
            let (letter_mark, repeat) = gall_fn::stem_lookup(&letter);
            match letter_mark {
                LetterMark::Stem(s_type) => {
                    syl = self.populate_stem(s_type, syl, clock_loc, letter_sep_ang, repeat)?;
                },
                LetterMark::GallVowel(v_type) => {
                    //GallVowel
                    syl = self.populate_vowel()?;
                }
                LetterMark::Digit(num)=> {},
                LetterMark::GallMark => {},
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
    fn stem_populate(&mut self) {

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
