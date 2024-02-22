use std::cell::OnceCell;

use crate::gall_ang::GallAng;
use crate::gall_circle::{ChildCircle, Circle, HollowCircle};
use crate::gall_config::Config;
use crate::gall_fn::{self, LetterMark};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_stem::{Stem, StemType};
use crate::gall_vowel::GallVowel;
use crate::gall_word::GallWord;
//use crate::gall_vowel::{GallVowel, VowelType};

pub struct GallTainer {
    ang: GallAng,
    stem_type: OnceCell<StemType>,
    pub stem: Vec<Stem>,
    pub vowel: Vec<GallVowel>,
    //node: Vec<GallNode>,
    //dot: Vec<GallDot>,
    //mark: Vec<GallMark>,
    //parent_radius: Rc<Cell<f64>>,
    //parent_thickness: Rc<Cell<f64>>,
    //mut_parent_rad_fn: fn(f64)-> Result<(),Error>,
    //mut_parent_thick_fn: fn(f64)-> Result<(),Error>,
}

impl GallTainer {
    pub fn new(l_type:LetterMark) -> GallTainer {
        let (mark_type, stem_vec, vowel_vec) = match l_type {
            LetterMark::Digit(_)   => (Some(StemType::J),Vec::with_capacity(1), Vec::new()),
            LetterMark::Stem(mark)    => (Some(mark),Vec::with_capacity(1), Vec::new()),
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
            //parent_radius:parent.get_radius(),
            //parent_thickness: parent.get_thickness(),
            //mut_parent_rad_fn: radius_fn_ptr,
            //mut_parent_thick_fn: thick_fn_ptr,
        }
    }
    pub fn init(&mut self, stem_type:StemType, con_count:usize, ang:f64) -> usize {
        self.stem_type.get_or_init(||stem_type);
        self.ang.mut_ang(Some(con_count as f64 * ang));
        con_count + 1
    }
    pub fn stem_type(&self) -> Option<&StemType> {
        self.stem_type.get()
    }
    pub fn is_empty(&self) -> bool {
        self.stem.is_empty() && self.vowel.is_empty()
    }
    pub fn populate(&mut self, l_mark: LetterMark, d_mark:(Option<bool>, u8), word: &GallWord) {
        match l_mark {
            LetterMark::Stem(stem) => {
                let letter = self.create_stem(stem, word);
                self.add_stem(letter);
            }
            LetterMark::Digit(num) => todo!(),
            LetterMark::GallMark => {},//todo!(),
        }
        if let Some(dot) = d_mark.0 {
            let deco_fn = match dot {
                true =>{||()}, //TODO: add add_dot_fn
                false=>{||()}, //TODO: add add_dash_fn
            };
            for i in 0..d_mark.1 {
                deco_fn()
            }
        }
    }
    pub fn create_stem(&self, stem:StemType, word: &GallWord) -> Stem {
        let (p_rad, p_thick) = (word.radius(), word.thick());
        let (dist,thick) = match stem {
            StemType::J => (p_rad*(0.7 - Config::LETTER_FRAC_OF_WRD),p_thick*0.6),
            StemType::B => (p_rad*(1.2 - Config::LETTER_FRAC_OF_WRD),p_thick*0.6),
            StemType::S => (p_rad + p_thick, p_thick*0.6),
            StemType::Z => (p_rad, p_thick*0.6),
        };
        let loc = GallLoc::new(
            self.ang.ang().unwrap(),
            dist,
            word.get_center(),
        );
        Stem::new(
            loc,
            p_rad*Config::LETTER_FRAC_OF_WRD,
            thick,
            stem,
            word
        )
    }
    pub fn add_stem(&mut self, new_stem: Stem) {
        if self.stem_type.get().unwrap() != &new_stem.stem_type {
            println!("Warning! Stem has different type to tainer")
        } ;
        self.stem.push(new_stem);
        self.stem.sort_by(|a,b|b.radius().partial_cmp(&a.radius()).unwrap());
    }
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
    pub fn thi_calc(&self) -> (f64,f64) {
        let (stem1,stem2) = self.stack_check();
        let thi_inner = gall_fn::thi(
            stem1.dist(),
            stem1.outer_radius(), 
            stem1.parent_inner(),
        );
        let thi_outer = gall_fn::thi(
            stem2.dist(),
            stem2.inner_radius(), 
            stem2.parent_outer(),
        );
        (thi_inner,thi_outer)
    }
    pub fn theta_calc(&self) -> (f64,f64) {
        let (stem1,stem2) = self.stack_check();
        let theta_inner = gall_fn::theta(
            stem1.dist(),
            stem1.outer_radius(), 
            stem1.parent_inner(),
        );
        let theta_outer = gall_fn::theta(
            stem2.dist(),
            stem2.inner_radius(), 
            stem2.parent_outer(),
        );
        (theta_inner,theta_outer)
    }
    pub fn stack_check(&self) -> (&Stem, &Stem) {
        let stem1 = self.stem.first().unwrap();
        let stem2 = if Config::STACK {
            self.stem.last().unwrap()
        } else {
            stem1 // stem1 > stem2
        };
        (stem1,stem2)
    }
    fn unpack(mut self) -> (Vec<Stem>,Vec<GallVowel>) {
        self.vowel.sort_by(|a,b|b.radius().partial_cmp(&a.radius()).unwrap());
        self.stem.sort_by(|a,b|b.radius().partial_cmp(&a.radius()).unwrap());
        (self.stem,self.vowel)
    }
}

// impl ChildCircle for GallTainer {
// }