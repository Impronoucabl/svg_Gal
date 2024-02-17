const COLLISION_DIST: f64 = 0.001;

use std::rc::Rc;

use crate::gall_errors::{Error, GallError};
use crate::gall_ord::{BoundedValue, PositiveDist};
use crate::gall_fn::LetterType;
use crate::gall_stem::{Stem, StemType};
use crate::gall_struct::{ChildCircle, Circle, HollowCircle};
use crate::gall_vowel::{GallVowel, VowelType};

pub struct GallTainer {
    stem_type: Option<StemType>,
    pub stem: Vec<Stem>,
    vowel: Vec<GallVowel>,
    //node: Vec<GallNode>,
    //dot: Vec<GallDot>,
    //mark: Vec<GallMark>,
    parent_radius: Rc<PositiveDist>,
    parent_thickness: Rc<PositiveDist>,
}

impl GallTainer {
    pub fn new<T:HollowCircle>(l_type:LetterType, parent:T) -> GallTainer {
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
            parent_radius:parent.get_radius(),
            parent_thickness: parent.get_thickness(),
        }
    }
    pub fn mut_stemtype(&mut self, s_type: Option<StemType>) -> Result<(), Error> {
        if self.stem_type == s_type {
            Ok(())
        } else if self.stem.is_empty() && self.vowel.is_empty() {
            self.stem_type = s_type;
            Ok(())
        } else {
            Err(Error::new(GallError::StemAlreadySet))
        }
    }
    pub fn add_stem(&mut self, new_stem: Stem) -> Result<(),Error> {
        let s_type = match self.stem_type {
            Some(s_type) => s_type,
            None => return Err(Error::new(GallError::TainerMissingStem)),
        };
        if self.stem.len() > 2 {
            return Err(Error::new(GallError::TainerMissingStem)) //TODO: TooManyStem
        }
        if s_type == new_stem.stem_type {
            self.stem.push(new_stem);
            Ok(())
        } else {
            Err(Error::new(GallError::BadTainerStem))
        }
    }
    pub fn add_vowel(&mut self, new_vowel: GallVowel) -> Result<(),Error> {
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
                Err(Error::new(GallError::BadVowelType))
            }
        }
    }
}

impl Circle for GallTainer {
    //This can panic, since it's not a real circle
    fn get_radius(&self) -> Rc<PositiveDist> {
        if self.stem.is_empty() {
            //return Err(Error::new(GallError::TainerMissingStem))
            panic!()
        };
        let mut result = -0.1;
        for letter in self.stem {
            let radius = letter.get_outer_radius();
            if  radius.dist() > result {
                result = radius.dist();
            }
        }
        Rc::new(PositiveDist::new(result).unwrap())
    }
    fn mut_radius(&mut self, val:f64) -> Result<(), Error> {
        Err(Error::new(GallError::DoNotMutTainer))
    }
}

impl ChildCircle for GallTainer {
    fn get_parent_radius(&self) -> f64 {
        self.parent_radius.dist()
    }
    fn get_parent_thick(&self) -> f64 {
        self.parent_thickness.dist()
    }
    fn mut_stored_parent_radius(&mut self, new_radius:f64) -> Result<(),Error> {
        self.parent_radius.mut_val(new_radius)
    }
    fn mut_stored_parent_thick(&mut self, new_radius:f64) -> Result<(),Error> {
        self.parent_thickness.mut_val(new_radius)
    }
}