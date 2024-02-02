use std::error::Error;

use crate::gall_errors::{BadTainerStem, BadVowelType, DoNotMutTainer, TainerMissingStem};
use crate::gall_ord::PositiveDist;
use crate::gall_fn::LetterType;
use crate::gall_stem::{Stem, StemType};
use crate::gall_struct::Circle;
use crate::gall_vowel::{GallVowel, VowelType};

pub struct GallTainer {
    stem_type: Option<StemType>,
    stem: Vec<Stem>,
    vowel: Vec<GallVowel>,
    //node: Vec<GallNode>,
    //dot: Vec<GallDot>,
    //mark: Vec<GallMark>,
}

impl GallTainer {
    pub fn new(l_type:LetterType) -> GallTainer {
        let (stem_type, stem_vec, vowel_vec) = match l_type {
            LetterType::Digit   => (Some(StemType::J),Vec::with_capacity(1), Vec::new()),
            LetterType::BStem   => (Some(StemType::B),Vec::with_capacity(1), Vec::new()),
            LetterType::JStem   => (Some(StemType::J),Vec::with_capacity(1), Vec::new()),
            LetterType::SStem   => (Some(StemType::S),Vec::with_capacity(1), Vec::new()),
            LetterType::ZStem   => (Some(StemType::Z),Vec::with_capacity(1), Vec::new()),
            LetterType::A       => (None, Vec::new(), Vec::with_capacity(1)),
            LetterType::O1      => (None, Vec::new(), Vec::with_capacity(1)),
            LetterType::O2      => (None, Vec::new(), Vec::with_capacity(1)),
            LetterType::EIU     => (None, Vec::new(), Vec::with_capacity(1)),
            _ => (None,Vec::new(), Vec::new())
        };
        GallTainer {
            stem_type,
            stem:stem_vec,
            vowel:vowel_vec,
            //node: Vec::new(),
            //dot: Vec::new(),
        }
    }
    pub fn add_stem(&mut self, new_stem: Stem) -> Result<(),Box<dyn Error>> {
        let s_type = match self.stem_type {
            Some(s_type) => s_type,
            None => return Err(Box::new(TainerMissingStem)),
        };
        if s_type == new_stem.stem_type {
            self.stem.push(new_stem);
            Ok(())
        } else {
            Err(Box::new(BadTainerStem))
        }
    }
    pub fn add_vowel(&mut self, new_vowel: GallVowel) -> Result<(),Box<dyn Error>> {
        if self.vowel.len() == 0 {
            self.vowel.push(new_vowel);
            Ok(())
        } else {
            let v_type = self.vowel[0].vowel_type;
            let same_vow = match (new_vowel.vowel_type, v_type) {
                (VowelType::A,VowelType::A) => true,
                (VowelType::O1,VowelType::O1) => true,
                (VowelType::O2,VowelType::O2) => true,
                (VowelType::E|VowelType::I|VowelType::U,VowelType::E|VowelType::I|VowelType::U) => true,
                _ => false,
            };            
            if same_vow {
                self.vowel.push(new_vowel);
                Ok(())
            } else {
                Err(Box::new(BadVowelType))
            }
        }
    }
}

impl Circle for GallTainer {
    //This can panic, since it's not a real circle
    fn get_outer_radius(&self) -> PositiveDist {
        if self.stem.is_empty() {
            //return Err(Box::new(TainerMissingStem))
            panic!()
        };
        let mut result = -0.1;
        for letter in self.stem {
            let radius = letter.get_outer_radius();
            if  radius.dist() > result {
                result = radius.dist();
            }
        }
        PositiveDist::new(result).unwrap()
    }
    //This can panic, since it's not a real circle
    fn get_inner_radius(&self) -> PositiveDist {
        if self.stem.is_empty() {
            //return Err(Box::new(TainerMissingStem))
            panic!()
        };
        let mut result = f64::INFINITY;
        for letter in self.stem {
            let radius = letter.get_outer_radius();
            if  radius.dist() < result {
                result = radius.dist();
            }
        }
        PositiveDist::new(result).unwrap()
    }
    fn mut_radius(&mut self, val:f64) -> Result<(), Box<dyn Error>> {
        Err(Box::new(DoNotMutTainer))
    }
}