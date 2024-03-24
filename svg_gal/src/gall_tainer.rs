use std::cell::{Cell, OnceCell};
use std::f64::consts::{PI, TAU};
use std::rc::Rc;

use crate::gall_ang::GallAng;
use crate::gall_circle::{ChildCircle, Circle, Dot, HollowCircle};
use crate::gall_config::Config;
use crate::gall_errors::{self, Error, GallError};
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
    pub mark: Vec<()>, //GallMark>,
    pub state: Option<TainerState>,
}

impl TainerState {
    pub fn new(angle: f64, letter_mark:&LetterMark, word:&GallWord) -> TainerState {
        //let angle = ;
        let stem_type = OnceCell::new();
        let w_rad = word.radius();
        let rad: f64;
        let dist = match letter_mark {
            LetterMark::Stem(stem) => {
                stem_type.set(*stem);
                rad = word.radius()*Config::LETTER_FRAC_OF_WRD;
                match stem {
                    StemType::J => w_rad*(0.7 - Config::LETTER_FRAC_OF_WRD),
                    StemType::B => w_rad*(1.2 - Config::LETTER_FRAC_OF_WRD),
                    StemType::S => w_rad + word.thick()*Config::LETTER_THICK_FRAC,
                    StemType::Z => w_rad,
                }
            },
            LetterMark::GallVowel(vow) => {
                rad = word.radius()*Config::VOWEL_FRAC_OF_WRD;
                match vow {
                    VowelType::A => w_rad*1.2,
                    VowelType::O1 => w_rad*0.6,
                    VowelType::O2 => w_rad*0.6,
                    _ => w_rad,
                }
            },
            LetterMark::Digit(_) => {
                stem_type.set(StemType::J);
                rad = word.radius()*Config::LETTER_FRAC_OF_WRD;
                w_rad*(0.7 - Config::LETTER_FRAC_OF_WRD)
            },
            LetterMark::GallMark => todo!(),
        };

        let loc = GallLoc::new(
            angle,
            dist,
            word.pos_ref(),
        );
        TainerState { 
            angle: Rc::new(Cell::new(GallAng::new(loc.ang()))), 
            stem_type, 
            letter_dist: Rc::new(Cell::new(dist)), 
            letter_rad: Rc::new(Cell::new(rad)), 
            letter_pos: loc.pos_ref(), 
            vowel: false,
        }
    }
}

impl GallTainer {
    pub fn new() -> GallTainer {
        GallTainer {
            stem:Vec::new(),
            vowel:Vec::new(),
            node: Vec::new(),
            dot: Vec::new(),
            mark: Vec::new(),
            state: None,
        }
    }
    pub fn init(&mut self, mark:&LetterMark, con_count:usize, ang:f64, word: &GallWord) -> usize {
        if let Some(state) = &mut self.state {
            panic!();
        } else {
            let state = TainerState::new(
                con_count as f64 * ang,
                mark,
                word,
            );
            self.state = Some(state);
        };
        con_count + 1
    }
    pub fn is_stateless(&self) -> bool {
        self.state.is_none()
    }
    pub fn stem_type(&self) -> Option<&StemType> {
        if let Ok(state) = self.get_state() {
            state.stem_type.get()
        } else {
            None
        }
    }
    pub fn is_empty(&self) -> bool {
        self.stem.is_empty() && self.vowel.is_empty() && self.mark.is_empty()
    }
    pub fn populate(&mut self, l_mark: LetterMark, d_mark:(Option<Decor>, i8), repeat:u8, word: &GallWord) {
        match l_mark {
            LetterMark::Stem(stem) => {
                for n in 0..=repeat {
                    self.add_stem(stem, word, n);
                };                
            },
            LetterMark::GallVowel(vow) => {
                for n in 0..=repeat {
                    self.add_vowel(vow,word, n);
                };
            },
            LetterMark::Digit(mut num) => {
                self.add_digit(word);
                println!("{}",num);
                if num.is_negative() {
                    //add negative mark
                    num = num.abs();
                };
                if num >= 5 {
                    self.add_dot(1, word.get_radius());
                    num -= 5;
                };
                for _ in 0..=num {
                    //add marks
                }
            },
            LetterMark::GallMark => {},//todo!(),
        }
        if let Some(dot) = d_mark.0 {
            if dot == Decor::Dot {
                for n in 0..d_mark.1 {
                    self.add_dot(n - 1, word.get_radius());
                }    
            } else {
                for n in 0..d_mark.1 {
                    self.add_dash(n - 1, word.get_radius());
                }
            }
        }
    }
    // pub fn populate_o1(&mut self, repeat:bool, word: &GallWord) {
    //     let letter = self.create_vowel(VowelType::O1,word);
    //     if repeat {
    //         self.add_vowel(self.create_vowel(VowelType::O1,word));
    //     };
    //     self.add_vowel(letter);
    // }
    fn init_state_vow(&mut self, vow:VowelType, word: &GallWord) -> GallLoc {
        let mut loc = GallLoc::new(
            self.ang(),
            1.0,
            word.pos_ref(),
        ); 
        let p_rad = word.radius();
        if let Some(state) = &mut self.state {
            let dist = match vow {
                VowelType::A => p_rad*1.2,
                VowelType::O1 => p_rad*0.6,
                VowelType::O2 => p_rad*0.6,
                _ => state.letter_dist.get(),
            };
            _ = loc.mut_dist(dist);
            let rad = word.radius()*Config::VOWEL_FRAC_OF_WRD;
            state.letter_rad = Rc::new(Cell::new(rad));
            state.vowel = true;
        }
        loc
    }
    pub fn add_dot(&mut self, num: i8, w_rad: Rc<Cell<f64>>) -> Result<(), Error>{
        self.dot.push(Dot::new(
            GallRelLoc::new(
                self.get_ang(),
                PI + Config::DEF_DOT_SPREAD * f64::from(num),
                self.get_state()?.letter_rad.clone(),
                0.0,
                self.get_state()?.letter_pos.clone(),
            ),
            Config::DOT_RADIUS,
            w_rad,     
        ));
        Ok(())
    }
    pub fn add_dash(&mut self, num: i8, w_rad: Rc<Cell<f64>>) -> Result<(), Error> {
        self.node.push(GallNode::new(
            GallRelLoc::new(
                self.get_ang(),
                PI + num as f64 * Config::DEF_DOT_SPREAD,
                self.get_state()?.letter_rad.clone(),
                0.0,
                self.get_state()?.letter_pos.clone(),
            ),
            self.get_state()?.letter_dist.clone(),
            w_rad,     
        ));
        Ok(())
    }
    pub fn add_stem(&mut self, stem: StemType, word: &GallWord, repeat: u8) {
        let rank = self.stem.len();
        let thick = word.thick()*Config::LETTER_THICK_FRAC + f64::from(rank as u8 - repeat) * Config::CONSEC_LETT_GROWTH;
        let ang = self.ang();
        if let Some(state) = &mut self.state {
            let rad = state.letter_rad.clone();
            let new_rad = rad.get() + Config::STACK_SEP_DIST+2.0*thick;
            state.letter_rad = Rc::new(Cell::new(new_rad));
            self.stem.push(Stem::new(
                GallLoc::new(
                    ang,
                    state.letter_dist.get(),
                    word.pos_ref(),
                ),
                state.letter_rad.clone(),
                thick,
                stem,
                word
            ));
        };
    }
    pub fn add_vowel(&mut self, vow:VowelType, word: &GallWord, repeat:u8) {
        let rank = self.vowel.len();
        let thick = word.thick()*Config::VOWEL_THICK_FRAC + f64::from(rank as u8 - repeat) * Config::CONSEC_LETT_GROWTH;
        let loc = if rank <= 0 {
            self.init_state_vow(vow, word)
        } else {
            let ang = self.ang();
            let state = self.get_mut_state().expect("existing vowels should set state");
            let rad = state.letter_rad.clone();
            let new_rad = rad.get() + Config::STACK_SEP_DIST+2.0*thick;
            state.letter_rad = Rc::new(Cell::new(new_rad));
            GallLoc::new(
                ang,
                state.letter_dist.get(),
                word.pos_ref(),
            )
        };
        // if let Some(stem) = self.state.stem_type.get() {
        //     match stem {
        //         // StemType::J|StemType::B => match vow {
        //         //     VowelType::E|VowelType::I|VowelType::U => {
        //         //         new_vowel.center_on_stem(&self.stem[0]);
        //         //     },
        //         //     VowelType::O1 => {
        //         //         new_vowel.o_attach_init(&self.stem[0]);
        //         //     },
        //         //     _ => {},
        //         // },
        //         _ => {},
        //     }
        // }
        self.vowel.push(GallVowel::new(
            loc,
            self.get_state().unwrap().letter_rad.clone(),
            thick,
            vow,
            word
        ))
    }
    pub fn add_digit(&mut self, word: &GallWord) {
        let rank = self.stem.len();
        let thick = word.thick()*Config::DIGIT_THICK_FRAC;
        let ang = self.ang();
        if let Some(state) = &mut self.state {
            let rad = state.letter_rad.clone();
            let new_rad = rad.get() + Config::NUM_SEP_DIST+2.0*thick;
            state.letter_rad = Rc::new(Cell::new(new_rad));
            self.stem.push(Stem::new(
                GallLoc::new(
                    ang,
                    state.letter_dist.get(),
                    word.pos_ref(),
                ),
                state.letter_rad.clone(),
                thick,
                StemType::J,
                word,
            ));
        };
    }
    pub fn thi_calc(&self) -> Result<(f64,f64), Error> {
        let (stem1,stem2) = self.stack_check()?;
        Ok((stem1.inner_thi()?,stem2.outer_thi()?))
        
    }
    pub fn theta_calc(&self) -> Result<(f64,f64), Error> {
        let (stem1,stem2) = self.stack_check()?;
        Ok((stem1.inner_theta()?,stem2.outer_theta()?))
    }
    pub fn stack_check(&self) -> Result<(&Stem, &Stem), Error> {
        if let (Some(stem1), Some(stem2)) = (self.stem.first(), self.stem.last()) {
            if self.stem_type() == Some(&StemType::B) {
                Ok((stem1, stem1))
            } else {
                Ok((stem1, stem2))
            }
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
        if let Some(state) = &mut self.state {
            state.angle.set(GallAng::new(new_ang));
        } else {
            println!("Warning: angle not set - tainer not init")
        }
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
        let state = self.get_mut_state()?;
        let mut ang = state.angle.get();
        ang.rotate(angle).expect("tainer ang is None");
        state.angle.set(ang);
        for stem in &mut self.stem {
            stem.mut_ccw(angle)?;
        }
        for vowel in &mut self.vowel {
            vowel.mut_ccw(angle)?;
        }
        for node in &mut self.node {
            node.update()
        }
        for dot in &mut self.dot {
            dot.update()
        }
        Ok(())
    }
    pub fn get_state(&self) -> Result<&TainerState, Error> {
        if let Some(state) = &self.state {
            Ok(state)
        } else {
            Err(Error::new(GallError::TainerNotInit))
        }
    }
    pub fn get_mut_state(&mut self) -> Result<&mut TainerState, Error> {
        if let Some(state) = &mut self.state {
            Ok(state)
        } else {
            Err(Error::new(GallError::TainerNotInit))
        }
    }
    pub fn ang(&self) -> f64 {
        self.get_state().unwrap().angle.get().ang().unwrap()
    }
    pub fn get_ang(&self) -> Rc<Cell<GallAng>> {
        self.get_state().unwrap().angle.clone()
    }
    pub fn step_ccw(&mut self) -> Result<(), Error> {
        self.bound_ccw_rotate(Config::COLLISION_DIST)
    }
    pub fn step_cw(&mut self) -> Result<(), Error> {
        self.bound_cw_rotate(Config::COLLISION_DIST)
    }
}
