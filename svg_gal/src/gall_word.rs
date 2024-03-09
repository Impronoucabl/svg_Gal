use std::cmp::{max, max_by, Ordering};
use std::f64::consts::TAU;
use std::iter::zip;
use std::{cell::Cell, f64::consts::PI};
use std::rc::Rc;

use crate::gall_circle::{Circle, HollowCircle};
use crate::gall_config::Config;
use crate::gall_errors::{Error, GallError};
use crate::gall_fn::{self, LetterMark};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_tainer::GallTainer;

pub struct GallWord {
    loc: GallLoc,
    pub tainer_vec: Vec<GallTainer>,
    radius: Rc<Cell<f64>>,
    thickness: Rc<Cell<f64>>,
}

impl GallWord {
    pub fn new(text:String, len_guess:usize, loc:GallLoc, radius: f64, thick:f64) -> GallWord {
        let tainer_vec = Vec::with_capacity(len_guess);
        let word = GallWord{
            loc,
            tainer_vec,
            radius: Rc::new(Cell::new(radius)),
            thickness: Rc::new(Cell::new(thick))
        };
        word.populate(text, len_guess)
    } 
    fn populate(mut self, word:String, len_guess:usize) -> GallWord {
        let tainer_ang = TAU/(len_guess as f64); 
        let mut con_count:usize = 0;
        let mut con = self.get_con();
        for cha in word.chars() {
            let (l_mark, repeat) = gall_fn::stem_lookup(&cha);
            let d_mark = gall_fn::dot_lookup(&cha);
            if con.stem_type().is_none() && con.is_empty() {
                if let LetterMark::Stem(stem) = l_mark {
                    con_count = con.init(Some(stem), con_count, tainer_ang)
                } else {
                    con_count = con.init(None,con_count,tainer_ang)
                }
            } else {
                match &l_mark {
                    LetterMark::Stem(stem) => {
                        if (!Config::STACK && !con.stem.is_empty()) || (Some(stem) != con.stem_type()) {
                            self.tainer_vec.push(con);
                            con = self.get_con();
                            con_count = con.init(Some(*stem), con_count, tainer_ang);
                        }
                    },
                    LetterMark::GallVowel(_) => {
                        if !Config::STACK && !con.vowel.is_empty() {
                            self.tainer_vec.push(con);
                            con = self.get_con();
                            con_count = con.init(None, con_count, tainer_ang);
                        }
                    },
                    LetterMark::Digit(_) => {todo!()},
                    LetterMark::GallMark => {},
                     
                }
            } //At this point the con tainer should be initialised.
            con.populate(l_mark, d_mark, &self)
        }
        self.tainer_vec.push(con);
        self
    }
    fn get_con(&self) -> GallTainer {
        GallTainer::new(LetterMark::GallMark)
    }
    fn check_radius(&self, new_radius:f64) -> Result<(),Error> {
        //todo!();
        Ok(())
    }
    fn gen_edge_vec(&self) -> Option<Vec<(f64,f64)>> {
        let mut edge_vec = Vec::with_capacity(self.tainer_vec.len());
        for con in &self.tainer_vec {
            if let Ok((i_thi, o_thi)) = con.thi_calc() {
                let thi = max_by(i_thi,o_thi, |a,b|a.partial_cmp(b).expect("thi is NaN"));
                let edges = (con.ang() - thi, con.ang() + thi);
                edge_vec.push(edges)
            } else {
                edge_vec.push((con.ang(),con.ang()))
            }
        }
        Some(edge_vec)
    }
    fn gen_dist_vec(&self) -> Option<Vec<f64>> {
        let mut edge_iter = self.gen_edge_vec()?.into_iter();
        let mut dists = Vec::with_capacity(self.tainer_vec.len());
        if let Some((first, mut current_edge)) = edge_iter.next() {
            while let Some((cw_edge,ccw_edge)) = edge_iter.next() {
                dists.push(cw_edge - current_edge);
                current_edge = ccw_edge;
            }
            dists.push(first + TAU - current_edge);
            Some(dists)
        } else {
            println!("Empty tainer_vec");
            None
        }
    }
    fn even_tainer_spread(&mut self) -> Option<()> {
        let dists = self.gen_dist_vec()?;
        let mut change = 0.0;
        let mut loop_iter = zip(dists, &mut self.tainer_vec);
        let (first, con1) = loop_iter.next().expect("empty?");
        let mut left = first;
        while let Some((right, con)) = loop_iter.next() {
            let movement = (right - left)/2.0;
            _ = con.bound_ccw_rotate(movement);
            if movement > change {
                change = movement
            }
            left = right;
        }
        let movement = (first - left)/2.0;
        _ = con1.bound_ccw_rotate(movement);
        if movement > change {
            change = movement
        }
        if change > 2.0*Config::STEP_DIST {
            Some(())
        } else {
            None
        }
    }
    pub fn basic(&mut self) {
        while let Some(_) = self.even_tainer_spread() {};
    }
}

impl HollowCircle for GallWord {
    fn thick(&self) -> f64 {
        self.thickness.get()
    }
    fn get_thickness(&self) -> Rc<Cell<f64>> {
        self.thickness.clone()
    }
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error> {
        self.thickness.set(new_thick);
        Ok(())
    }
}
impl Circle for GallWord {
    fn radius(&self) -> f64 {
        self.radius.get()
    }
    fn get_radius(&self) -> Rc<Cell<f64>> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        self.check_radius(new_radius)?;
        self.radius.set(new_radius);
        Ok(())
    }
}
impl Location for GallWord {
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.loc.mut_center(movement)
    }
    fn set_center(&mut self, new_center:Rc<Cell<(f64,f64)>>) {
        self.loc.set_center(new_center)
    }
    fn get_center(&self) -> Rc<Cell<(f64,f64)>> {
        self.loc.get_center()
    }
    fn x(&self) -> f64 {
        self.loc.x()
    }
    fn y(&self) -> f64 {
        self.loc.y()
    }
    fn pos_ref(&self) -> Rc<Cell<(f64,f64)>> {
        self.loc.pos_ref()
    }
    fn update(&mut self) {
        self.loc.update()
    }
}
impl PolarOrdinate for GallWord {
    fn mut_ang(&mut self, new_ang:f64) {
        self.loc.mut_ang(new_ang)
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error> {
        self.loc.mut_dist(new_dist)
    }
    fn ang(&self) -> Option<f64> {
        self.loc.ang()
    }
    fn dist(&self) -> f64 {
        self.loc.dist()
    }
    fn get_dist(&self) -> Rc<Cell<f64>> {
        self.loc.get_dist()
    }
}