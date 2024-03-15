use std::cell::{Cell, OnceCell};
use std::f64::consts::{PI, TAU};
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

struct TainerState {
    angle: Rc<Cell<GallAng>>,
    stem_type: OnceCell<StemType>,
    letter_dist: Rc<Cell<f64>>, 
    letter_rad: Rc<Cell<f64>>, 
    letter_pos: Rc<Cell<(f64,f64)>>,
    vowel: bool,
}

pub struct GallTainer {
    pub stem: Vec<Stem>,
    pub vowel: Vec<GallVowel>,
    pub node: Vec<GallNode>,
    pub dot: Vec<Dot>,
    //mark: Vec<GallMark>,
    state: TainerState,
}

impl TainerState {
    pub fn new() -> TainerState {
        let angle = Rc::new(Cell::new(GallAng::new(None)));
        let stem_type = OnceCell::new();
        TainerState { 
            angle, 
            stem_type, 
            letter_dist: Rc::new(Cell::new(-0.0)), 
            letter_rad: Rc::new(Cell::new(-0.0)), 
            letter_pos: Rc::new(Cell::new((-0.0,-0.0))), 
            vowel: false,
        }
    }
}

impl GallTainer {
    pub fn new(l_type:LetterMark) -> GallTainer {
        let (mark_type, stem_vec, vowel_vec) = match l_type {
            LetterMark::Digit(_) => (Some(StemType::J),Vec::with_capacity(1), Vec::new()),
            LetterMark::Stem(mark) => (Some(mark),Vec::with_capacity(1), Vec::new()),
            LetterMark::GallVowel(_) => (None, Vec::new(), Vec::with_capacity(1)),
             _ => (None,Vec::new(), Vec::new())
        };
        let buffer = TainerState::new();
        if let Some(stem) = mark_type {
            buffer.stem_type.get_or_init(||stem);
        };
        GallTainer {
            stem:stem_vec,
            vowel:vowel_vec,
            node: Vec::new(),
            dot: Vec::new(),
            state: buffer,
        }
    }
    pub fn init(&mut self, stem_type:Option<StemType>, con_count:usize, ang:f64) -> usize {
        if let Some(stem) = stem_type {
            self.state.stem_type.get_or_init(||stem);
        };
        self.set_ang(Some(con_count as f64 * ang));
        con_count + 1
    }
    pub fn stem_type(&self) -> Option<&StemType> {
        self.state.stem_type.get()
    }
    pub fn is_empty(&self) -> bool {
        self.stem.is_empty() && self.vowel.is_empty()
    }
    pub fn populate(&mut self, l_mark: LetterMark, d_mark:(Option<Decor>, i8), repeat:bool, word: &GallWord) {
        match l_mark {
            LetterMark::Stem(stem) => {
                let letter = self.create_stem(stem, word);
                if repeat {
                    let lett = self.create_stem(stem, word);
                    self.add_stem(lett);
                };
                self.add_stem(letter);
            },
            LetterMark::GallVowel(vow) => {
                let letter = self.create_vowel(vow,word);
                if repeat {
                    self.add_vowel(self.create_vowel(vow,word));
                };
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
    pub fn populate_o1(&mut self, repeat:bool, word: &GallWord) {
        let letter = self.create_vowel(VowelType::O1,word);
        if repeat {
            self.add_vowel(self.create_vowel(VowelType::O1,word));
        };
        self.add_vowel(letter);
    }
    fn init_buff_lett(&mut self, stem:StemType, word:&GallWord) -> GallLoc {
        let p_rad = word.radius();
        let p_thick = word.thick()*Config::LETTER_THICK_FRAC;
        let dist = match stem {
            StemType::J => p_rad*(0.7 - Config::LETTER_FRAC_OF_WRD),
            StemType::B => p_rad*(1.2 - Config::LETTER_FRAC_OF_WRD),
            StemType::S => p_rad + p_thick,
            StemType::Z => p_rad,
        };
        let rad = p_rad*Config::LETTER_FRAC_OF_WRD;
        let loc = GallLoc::new(
            self.ang(),
            dist,
            word.pos_ref(),
        );
        self.state.letter_rad = Rc::new(Cell::new(rad));
        loc
    }
    pub fn create_stem(&mut self, stem:StemType, word: &GallWord) -> Stem {
        let rank = self.stem.len() as f64;
        let thick = word.thick()*Config::LETTER_THICK_FRAC;
        let loc = if rank == 0.0 {
            self.init_buff_lett(stem, word)
        } else {
            let rad = self.state.letter_rad.clone();
            let new_rad = rad.get() + Config::STACK_SEP_DIST+2.0*thick;
            self.state.letter_rad = Rc::new(Cell::new(new_rad));
            GallLoc::new(
                self.ang(),
                self.state.letter_dist.get(),
                word.pos_ref(),
            )
        };
        // let (p_rad, p_thick) = (word.radius(), word.thick()*Config::LETTER_THICK_FRAC);
        // let (dist,thick) = match stem {
        //     StemType::J => (p_rad*(0.7 - Config::LETTER_FRAC_OF_WRD),p_thick),
        //     StemType::B => (p_rad*(1.2 - Config::LETTER_FRAC_OF_WRD),p_thick),
        //     StemType::S => (p_rad + p_thick, p_thick),
        //     StemType::Z => (p_rad, p_thick),
        // };
        Stem::new(
            loc,
            self.state.letter_rad.clone(),
            thick + rank * (Config::CONSEC_LETT_GROWTH),
            stem,
            word
        )
    }
    pub fn create_vowel(&self, vow:VowelType, word: &GallWord) -> GallVowel {
        let rank = self.vowel.len() as f64;
        let (p_rad, p_thick) = (word.radius(), word.thick()*Config::VOWEL_THICK_FRAC);
        let (dist,thick) = match vow {
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
            p_rad*Config::VOWEL_FRAC_OF_WRD  + rank * (Config::STACK_SEP_DIST+2.0*thick),
            thick + rank * (Config::CONSEC_LETT_GROWTH),
            vow,
            word
        )
    }
    pub fn create_dot(&self, num: i8, w_rad: Rc<Cell<f64>>) -> Dot {
        Dot::new(
            GallRelLoc::new(
                self.get_ang(),
                PI + Config::DEF_DOT_SPREAD * num as f64,
                self.state.letter_rad.clone(),
                0.0,
                self.state.letter_pos.clone(),
            ),
            Config::DOT_RADIUS,
            w_rad,     
        )
    }
    pub fn create_dash(&self, num: i8, w_rad: Rc<Cell<f64>>) -> GallNode {
        GallNode::new(
            GallRelLoc::new(
                self.get_ang(),
                PI + num as f64 * Config::DEF_DOT_SPREAD,
                self.state.letter_rad.clone(),
                0.0,
                self.state.letter_pos.clone(),
            ),
            self.state.letter_dist.clone(),
            w_rad,     
        )
    }
    pub fn add_stem(&mut self, new_stem: Stem) {
        if self.state.stem_type.get().unwrap() != &new_stem.stem_type {
            println!("Warning! Stem has different type to tainer")
        };
        self.state.letter_dist = new_stem.get_dist();
        self.state.letter_rad = new_stem.get_radius();
        self.state.letter_pos = new_stem.pos_ref();
        self.stem.push(new_stem);
    }
    pub fn add_vowel(&mut self, mut new_vowel: GallVowel) {
        if let Some(stem) = self.state.stem_type.get() {
            match stem {
                StemType::J|StemType::B => match new_vowel.vowel_type {
                    VowelType::E|VowelType::I|VowelType::U => {
                        self.state.letter_dist = new_vowel.get_dist();
                        self.state.letter_rad = new_vowel.get_radius();
                        self.state.letter_pos = new_vowel.pos_ref();
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
    pub fn thi_calc(&self) -> Result<(f64,f64), Error> {
        let (stem1,stem2) = self.stack_check()?;
        let thi_inner = gall_fn::thi(
            stem1.dist(),
            stem1.outer_radius(), 
            stem1.parent_inner(),
        )?;
        let thi_outer = gall_fn::thi(
            stem2.dist(),
            stem2.inner_radius(), 
            stem2.parent_outer(),
        )?;
        Ok((thi_inner,thi_outer))
    }
    pub fn theta_calc(&self) -> Result<(f64,f64), Error> {
        let (stem1,stem2) = self.stack_check()?;
        let theta_inner = gall_fn::theta(
            stem1.dist(),
            stem1.outer_radius(), 
            stem1.parent_inner(),
        )?;
        let theta_outer = gall_fn::theta(
            stem2.dist(),
            stem2.inner_radius(), 
            stem2.parent_outer(),
        )?;
        Ok((theta_inner,theta_outer))
    }
    pub fn stack_check(&self) -> Result<(&Stem, &Stem),Error> {
        if let Some(stem1) = self.stem.first() {
            let stem2 = if Config::STACK {
                self.stem.last().unwrap()
            } else {
                stem1 // stem1 > stem2
            };
            Ok((stem1,stem2))
        } else {
            Err(Error::new(GallError::NoStemInTainer))
        }
    }
    pub fn collect_nodes(&mut self) -> Vec<&mut GallNode> {
        let mut nodes = Vec::with_capacity(self.node.len());
        for node in &mut self.node {
            nodes.push(node)
        }
        nodes
    }
    pub fn stem_sort(&mut self) {
        self.stem.sort_by(|a,b|b.radius().partial_cmp(&a.radius()).unwrap());
    }
    // fn unpack(mut self) -> (Vec<Stem>,Vec<GallVowel>) {
    //     self.vowel.sort_by(|a,b|b.radius().partial_cmp(&a.radius()).unwrap());
    //     self.stem.sort_by(|a,b|b.radius().partial_cmp(&a.radius()).unwrap());
    //     (self.stem,self.vowel)
    // }
    pub fn set_ang(&mut self, new_ang:Option<f64>) {
        _ = self.state.angle.set(GallAng::new(new_ang))
    }
    pub fn bound_ccw_rotate(&mut self, angle: f64) -> Result<(), Error> {
        if angle < 0.0 {
            self.bound_cw_rotate(-angle)
        } else {
            if self.ang() + angle > TAU {
                Err(Error::new(GallError::NoStepSpace))
            } else {
                Ok(self.rotate(angle)?)
            }
        }
    }
    pub fn bound_cw_rotate(&mut self, angle: f64) -> Result<(), Error> {
        if angle < 0.0 {
            self.bound_ccw_rotate(-angle)
        } else {
            if self.ang() - angle < 0.0 {
                Err(Error::new(GallError::NoStepSpace))
            } else {
                Ok(self.rotate(-angle)?)
            }
        }
    }
    pub fn rotate(&mut self, angle: f64) -> Result<(), Error> {
        let mut ang = self.state.angle.take();
        ang.rotate(angle).expect("tainer ang is None");
        for stem in &mut self.stem {
            stem.mut_ccw(angle)?;
        }
        for vowel in &mut self.vowel {
            vowel.mut_ccw(angle)?;
        }
        self.state.angle.set(ang);
        for node in &mut self.node {
            node.update()
        }
        for dot in &mut self.dot {
            dot.update()
        }
        Ok(())
    }
    pub fn ang(&self) -> f64 {
        self.state.angle.get().ang().unwrap()
    }
    pub fn get_ang(&self) -> Rc<Cell<GallAng>> {
        self.state.angle.clone()
    }
    pub fn step_ccw(&mut self) -> Result<(), Error> {
        self.bound_ccw_rotate(Config::COLLISION_DIST)
    }
    pub fn step_cw(&mut self) -> Result<(), Error> {
        self.bound_cw_rotate(Config::COLLISION_DIST)
    }
}
