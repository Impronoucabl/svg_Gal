use std::cell::{Cell, OnceCell};
use std::f64::consts::PI;
use std::rc::Rc;

use crate::gall_ang::GallAng;
use crate::gall_circle::{ChildCircle, Circle, Dot, HollowCircle};
use crate::gall_config::Config;
use crate::gall_errors::{Error, GallError};
use crate::gall_fn::{self, Decor, LetterMark};
use crate::gall_loc::{GallLoc, GallRelLoc, Location};
use crate::gall_node::GallNode;
use crate::gall_ord::PolarOrdinate;
use crate::gall_stem::{Stem, StemType};
use crate::gall_vowel::{GallVowel, VowelType};
use crate::gall_word::GallWord;

pub struct GallTainer {
    ang: Rc<Cell<GallAng>>,
    stem_type: OnceCell<StemType>,
    pub stem: Vec<Stem>,
    pub vowel: Vec<GallVowel>,
    pub node: Vec<GallNode>,
    pub dot: Vec<Dot>,
    //mark: Vec<GallMark>,
    pub buffer: (Rc<Cell<f64>>, Rc<Cell<f64>>, Rc<Cell<(f64,f64)>>), //letter_distance, letter radius, letter position
}

impl GallTainer {
    pub fn new(l_type:LetterMark) -> GallTainer {
        let (mark_type, stem_vec, vowel_vec) = match l_type {
            LetterMark::Digit(_) => (Some(StemType::J),Vec::with_capacity(1), Vec::new()),
            LetterMark::Stem(mark) => (Some(mark),Vec::with_capacity(1), Vec::new()),
            LetterMark::GallVowel(_) => (None, Vec::new(), Vec::with_capacity(1)),
             _ => (None,Vec::new(), Vec::new())
        };
        let stem_type = OnceCell::new();
        if let Some(stem) = mark_type {
            stem_type.get_or_init(||stem);
        };
        let ang = Rc::new(Cell::new(GallAng::new(None)));
        GallTainer {
            ang,
            stem_type,
            stem:stem_vec,
            vowel:vowel_vec,
            node: Vec::new(),
            dot: Vec::new(),
            buffer: (Rc::new(Cell::new(-0.0)), Rc::new(Cell::new(-0.0)), Rc::new(Cell::new((-0.0,-0.0))))
        }
    }
    pub fn init(&mut self, stem_type:Option<StemType>, con_count:usize, ang:f64) -> usize {
        if let Some(stem) = stem_type {
            self.stem_type.get_or_init(||stem);
        };
        self.mut_ang(Some(con_count as f64 * ang));
        con_count + 1
    }
    pub fn stem_type(&self) -> Option<&StemType> {
        self.stem_type.get()
    }
    pub fn is_empty(&self) -> bool {
        self.stem.is_empty() && self.vowel.is_empty()
    }
    pub fn populate(&mut self, l_mark: LetterMark, d_mark:(Option<Decor>, i8), word: &GallWord) {
        match l_mark {
            LetterMark::Stem(stem) => {
                let letter = self.create_stem(stem, word);
                self.add_stem(letter);
            },
            LetterMark::GallVowel(vow) => {
                let letter = self.create_vowel(vow,word);
                self.add_vowel(letter);
            },
            LetterMark::Digit(num) => todo!(),
            LetterMark::GallMark => {},//todo!(),
        }
        if let Some(dot) = d_mark.0 {
            if dot == Decor::Dot {
                for n in 0..d_mark.1 {
                    let decor = self.create_dot(n - 1, word.get_radius());
                    self.add_dot(decor);
                }    
            } else {
                for n in 0..d_mark.1 {
                    let decor = self.create_dash(n - 1, word.get_radius());
                    self.add_dash(decor);
                }
            }
        }
    }
    pub fn create_stem(&self, stem:StemType, word: &GallWord) -> Stem {
        let rank = self.stem.len();
        let (p_rad, p_thick) = (word.radius(), word.thick()*Config::LETTER_THICK_FRAC + rank as f64 * 5.0);
        let (dist,thick) = match stem {
            StemType::J => (p_rad*(0.7 - Config::LETTER_FRAC_OF_WRD),p_thick),
            StemType::B => (p_rad*(1.2 - Config::LETTER_FRAC_OF_WRD),p_thick),
            StemType::S => (p_rad + p_thick, p_thick),
            StemType::Z => (p_rad, p_thick),
        };
        Stem::new(
            GallLoc::new(
                self.ang(),
                dist,
                word.pos_ref(),
            ),
            p_rad*Config::LETTER_FRAC_OF_WRD,
            thick,
            stem,
            word
        )
    }
    pub fn create_vowel(&self, stem:VowelType, word: &GallWord) -> GallVowel {
        let rank = self.vowel.len();
        let (p_rad, p_thick) = (word.radius(), word.thick()*Config::VOWEL_THICK_FRAC + rank as f64 * 5.0);
        let (dist,thick) = match stem {
            VowelType::A => (p_rad*1.2, p_thick),
            VowelType::E => (p_rad, p_thick),
            VowelType::I => (p_rad, p_thick),
            VowelType::O1 => (p_rad*0.6, p_thick),
            VowelType::O2 => (p_rad*0.6, p_thick),
            VowelType::U => (p_rad, p_thick),
        };
        GallVowel::new(
            GallLoc::new(
                self.ang(),
                dist,
                word.pos_ref(),
            ),
            p_rad*Config::VOWEL_FRAC_OF_WRD,
            thick,
            stem,
            word
        )
    }
    pub fn create_dot(&self, num: i8, w_rad: Rc<Cell<f64>>) -> Dot {
        let (_, letter_radius,center_ref) = self.buffer.clone();
        Dot::new(
            GallRelLoc::new(
                self.get_ang(),
                PI + Config::DEF_DOT_SPREAD * num as f64,
                letter_radius,
                0.0,
                center_ref
            ),
            Config::DOT_RADIUS,
            w_rad,     
        )
    }
    pub fn create_dash(&self, num: i8, w_rad: Rc<Cell<f64>>) -> GallNode {
        let (letter_dist, letter_radius,center_ref) = self.buffer.clone();
        GallNode::new(
            GallRelLoc::new(
                self.get_ang(),
                PI + num as f64 * Config::DEF_DOT_SPREAD,
                letter_radius,
                0.0,
                center_ref
            ),
            letter_dist,
            w_rad,     
        )
    }
    pub fn add_stem(&mut self, new_stem: Stem) {
        if self.stem_type.get().unwrap() != &new_stem.stem_type {
            println!("Warning! Stem has different type to tainer")
        };
        self.buffer = (new_stem.get_dist(), new_stem.get_radius(),new_stem.pos_ref());
        self.stem.push(new_stem);
        self.stem.sort_by(|a,b|b.radius().partial_cmp(&a.radius()).unwrap());
    }
    pub fn add_vowel(&mut self, mut new_vowel: GallVowel) {
        if let Some(stem) = self.stem_type.get() {
            match stem {
                StemType::J|StemType::B => match new_vowel.vowel_type {
                    VowelType::E|VowelType::I|VowelType::U => {
                        self.buffer = (new_vowel.get_dist(), new_vowel.get_radius(),new_vowel.pos_ref());
                        new_vowel.center_on_stem(&self.stem[0]);
                    },
                    VowelType::O1 => {
                        new_vowel.o_attach_init(&self.stem[0]);
                    },
                    _ => {},
                },
                _ => {},
            }
        }
        self.vowel.push(new_vowel);
    }
    pub fn add_dot(&mut self, dot:Dot) {
        self.dot.push(dot)
    }
    pub fn add_dash(&mut self, dash: GallNode) {
        self.node.push(dash)
    }
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
    // fn unpack(mut self) -> (Vec<Stem>,Vec<GallVowel>) {
    //     self.vowel.sort_by(|a,b|b.radius().partial_cmp(&a.radius()).unwrap());
    //     self.stem.sort_by(|a,b|b.radius().partial_cmp(&a.radius()).unwrap());
    //     (self.stem,self.vowel)
    // }
    pub fn mut_ang(&mut self, new_ang:Option<f64>) {
        _ = self.ang.set(GallAng::new(new_ang))
    }
    pub fn ang(&self) -> f64 {
        self.ang.get().ang().unwrap()
    }
    pub fn get_ang(&self) -> Rc<Cell<GallAng>> {
        self.ang.clone()
    }
}
