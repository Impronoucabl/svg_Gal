const COLLISION_DIST: f64 = 0.001;

use std::cell::{Cell, OnceCell};
use std::rc::Rc;

use crate::gall_ang::GallAng;
use crate::gall_circle::{HollowCircle, ParentCircle};
use crate::gall_errors::{Error, GallError};
use crate::gall_fn::{LetterMark, LetterType};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_stem::{Stem, StemType};
use crate::gall_word::GallWord;
//use crate::gall_struct::{ChildCircle, Circle, HollowCircle};
//use crate::gall_vowel::{GallVowel, VowelType};

pub struct GallTainer {
    ang: GallAng,
    stem_type: OnceCell<StemType>,
    pub stem: Vec<Stem>,
    pub vowel: Vec<Stem>,//GallVowel>,
    //node: Vec<GallNode>,
    //dot: Vec<GallDot>,
    //mark: Vec<GallMark>,
    parent_radius: Rc<Cell<f64>>,
    parent_thickness: Rc<Cell<f64>>,
    //mut_parent_rad_fn: fn(f64)-> Result<(),Error>,
    //mut_parent_thick_fn: fn(f64)-> Result<(),Error>,
}

impl GallTainer {
    pub fn new<T:HollowCircle>(l_type:LetterMark, parent:&T) -> GallTainer {
        let (mark_type, stem_vec, vowel_vec) = match l_type {
            LetterMark::Digit(_)   => (Some(StemType::J),Vec::with_capacity(1), Vec::new()),
            LetterMark::Stem(L)    => (Some(L),Vec::with_capacity(1), Vec::new()),
        //     LetterType::A       => (None, Vec::new(), Vec::with_capacity(1)),
        //     LetterType::O1      => (None, Vec::new(), Vec::with_capacity(1)),
        //     LetterType::O2      => (None, Vec::new(), Vec::with_capacity(1)),
        //     LetterType::EIU     => (None, Vec::new(), Vec::with_capacity(1)),
             _ => (None,Vec::new(), Vec::new())
        };
        let stem_type = OnceCell::new();
        match mark_type {
            Some(stem) => {stem_type.get_or_init(||stem);},
            None => {},
        };
        //let thick_fn_ptr: fn(f64)->Result<(),Error> = parent.get_mut_thick_fn_ptr();
        //let radius_fn_ptr: fn(f64)->Result<(),Error> = parent.get_mut_rad_fn_ptr();
        GallTainer {
            ang: GallAng::new(None),
            stem_type,
            stem:stem_vec,
            vowel:vowel_vec,
            //node: Vec::new(),
            //dot: Vec::new(),
            parent_radius:parent.get_radius(),
            parent_thickness: parent.get_thickness(),
            //mut_parent_rad_fn: radius_fn_ptr,
            //mut_parent_thick_fn: thick_fn_ptr,
        }
    }
    pub fn init(&mut self, stem_type:StemType, mut con_count:usize, ang:f64) {
        self.stem_type.get_or_init(||stem_type);
        self.ang.mut_ang(Some(con_count as f64 * ang));
        con_count += 1;
    }
    pub fn stem_type(&self) -> Option<&StemType> {
        self.stem_type.get()
    }
    pub fn is_empty(&self) -> bool {
        self.stem.is_empty() && self.vowel.is_empty()
    }
    pub fn populate(&mut self, l_mark: LetterMark, word: &GallWord) {
        match l_mark {
            LetterMark::Stem(stem) => {},//todo!(),
            LetterMark::Digit(num) => todo!(),
            LetterMark::GallMark => {},//todo!(),
        }
    }
    pub fn create_stem(&self, stem:StemType, word: &GallWord) -> Stem {
        let dist = match stem {
            StemType::J => 200.0,
            StemType::B => 200.0,
            StemType::S => 200.0,
            StemType::Z => 200.0,
        };
        let loc = GallLoc::new(
            self.ang.ang().unwrap(),
            dist,
            word.get_center(),
        );
        Stem::new(
            loc,
            200.0,
            5.0,
            stem,
            word
        )
    }
    // pub fn add_stem(&mut self, new_stem: Stem) -> Result<(),Error> {
    //     let s_type = match self.stem_type {
    //         Some(s_type) => s_type,
    //         None => return Err(Error::new(GallError::TainerMissingStem)),
    //     };
    //     if self.stem.len() > 2 {
    //         return Err(Error::new(GallError::TainerMissingStem)) //TODO: TooManyStem
    //     }
    //     if s_type == new_stem.stem_type {
    //         self.stem.push(new_stem);
    //         Ok(())
    //     } else {
    //         Err(Error::new(GallError::BadTainerStem))
    //     }
    // }
    // pub fn add_vowel(&mut self, new_vowel: GallVowel) -> Result<(),Error> {
    //     if self.vowel.len() == 0 {
    //         self.vowel.push(new_vowel);
    //         Ok(())
    //     } else {
    //         let v_type = self.vowel[0].vowel_type;
    //         let same_vow = match (new_vowel.vowel_type, v_type) {
    //             (VowelType::A,VowelType::A) => true,
    //             (VowelType::O1,VowelType::O1) => true,
    //             (VowelType::O2,VowelType::O2) => true,
    //             (VowelType::E|VowelType::I|VowelType::U,VowelType::E|VowelType::I|VowelType::U) => true,
    //             _ => false,
    //         };            
    //         if same_vow {
    //             self.vowel.push(new_vowel);
    //             Ok(())
    //         } else {
    //             Err(Error::new(GallError::BadVowelType))
    //         }
    //     }
    // }
}

// impl ChildCircle for GallTainer {
// }