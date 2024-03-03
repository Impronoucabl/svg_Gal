use std::f64::consts::{TAU,PI};
use std::cell::OnceCell;

use crate::gall_circle::{Circle, HollowCircle};
use crate::gall_config::Config;
use crate::gall_errors::{Error, GallError};
use crate::gall_fn::{self, LetterMark};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_tainer::GallTainer;

pub struct GallWord<'a> {
    pub loc: GallLoc,
    pub tainer_vec: Vec<GallTainer<'a>>,
    pub radius: OnceCell<f64>,
    pub thickness: OnceCell<f64>,
}

fn get_con<'a>() -> GallTainer<'a> {
    GallTainer::new(LetterMark::GallMark)
}

impl <'a>GallWord<'a> {
    pub fn new(text: String, len_guess:usize, loc:GallLoc, radius: f64, thick:f64) -> GallWord<'a> {
        let mut tainer_vec = Vec::with_capacity(len_guess);
        let rad = OnceCell::new();
        let thickness = OnceCell::new();
        rad.set(radius);
        thickness.set(thick);
        let mut word = GallWord{
            loc,
            tainer_vec,
            radius: rad,
            thickness: thickness,
        };
        //word.populate(text, len_guess);
        word
    } 
    // pub fn populate(&mut self, word:String, len_guess:usize){
    //     let tainer_ang = TAU/(len_guess as f64); 
    //     let mut con_count:usize = 0;
    //     let mut con: GallTainer = self::get_con();
    //     let (w_rad, w_thick) = match (self.radius(), self.thick()) {
    //         (Some(rad), Some(thick)) => (rad, thick),
    //         _ => panic!()
    //     };
    //     for cha in word.chars() {
    //         let (l_mark, repeat) = gall_fn::stem_lookup(&cha);
    //         let d_mark = gall_fn::dot_lookup(&cha);
    //         if con.stem_type().is_none() && con.is_empty() {
    //             if let LetterMark::Stem(stem) = l_mark {
    //                 con_count = con.init(Some(stem), con_count, tainer_ang)
    //             } else {
    //                 con_count = con.init(None,con_count,tainer_ang)
    //             }
    //         } else {
    //             match &l_mark {
    //                 LetterMark::Stem(stem) => {
    //                     if (!Config::STACK && !con.stem.is_empty()) || (Some(stem) != con.stem_type()) {
    //                         self.tainer_vec.push(con);
    //                         con = self::get_con();
    //                         con_count = con.init(Some(*stem), con_count, tainer_ang);
    //                     }
    //                 },
    //                 LetterMark::GallVowel(_) => {
    //                     if !Config::STACK && !con.vowel.is_empty() {
    //                         self.tainer_vec.push(con);
    //                         con = self::get_con();
    //                         con_count = con.init(None, con_count, tainer_ang);
    //                     }
    //                 },
    //                 LetterMark::Digit(_) => {todo!()},
    //                 LetterMark::GallMark => {},
                     
    //             }
    //         } //At this point the con tainer should be initialised.
    //         con.populate(l_mark, d_mark, w_rad, w_thick, self.get_center())
    //     }
    //     self.tainer_vec.push(con);
    // }
    
    fn check_radius(&self, new_radius:f64) -> Result<(),Error> {
        //todo!();
        Ok(())
    }
}

//impl ParentCircle for GallWord {}
impl HollowCircle for GallWord<'_> {
    fn thick(&self) -> Option<&f64> {
        self.thickness.get()
    }
    fn get_thickness(&self) -> OnceCell<f64> {
        self.thickness.clone()
    }
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error> {
        self.thickness.set(new_thick);
        Ok(())
    }
}
impl Circle for GallWord<'_> {
    fn radius(&self) -> Option<&f64> {
        self.radius.get()
    }
    fn get_radius(&self) -> OnceCell<f64> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        self.check_radius(new_radius)?;
        self.radius.set(new_radius);
        Ok(())
    }
}
impl Location for GallWord<'_> {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    fn set_center(&mut self, new_center:OnceCell<(f64,f64)>) {
        self.loc.set_center(new_center)
    }
    fn get_center(&self) -> OnceCell<(f64,f64)> {
        self.loc.get_center()
    }
    fn x(&self) -> f64 {
        self.loc.x()
    }
    fn y(&self) -> f64 {
        self.loc.y()
    }
    fn pos_ref(&self) -> OnceCell<(f64,f64)> {
        self.loc.pos_ref()
    }
    fn center_ords(&self) -> (f64,f64) {
        self.loc.center_ords()
    }
}
impl PolarOrdinate for GallWord<'_> {
    fn mut_ang(&mut self, new_ang:f64) {
        self.loc.mut_ang(new_ang)
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error> {
        self.loc.mut_dist(new_dist)
    }
    fn ang(&self) -> Option<&f64> {
        self.loc.ang()
    }
    fn dist(&self) -> Option<&f64> {
        self.loc.dist()
    }
    fn get_ang(&self) -> OnceCell<f64> {
        self.loc.get_ang()
    }
    fn get_dist(&self) -> OnceCell<f64> {
        self.loc.get_dist()
    }
}