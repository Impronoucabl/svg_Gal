use std::cmp::max_by;
use std::f64::consts::TAU;
use std::iter::zip;
use std::cell::Cell;
use std::rc::Rc;

use crate::gall_circle::{Circle, HollowCircle};
use crate::gall_config::Config;
use crate::gall_errors::{Error, GallError};
use crate::gall_fn::{self, LetterMark, ProcessedWord};
use crate::gall_loc::{GallLoc, LocHolder};
use crate::gall_node::GallNode;
use crate::gall_ord::{GallOrd, OrdHolder};
use crate::gall_stem::StemType;
use crate::gall_tainer::GallTainer;
use crate::gall_vowel::VowelType;

pub struct GallWord {
    loc: GallLoc,
    pub tainer_vec: Vec<GallTainer>,
    radius: Rc<Cell<f64>>,
    thickness: Rc<Cell<f64>>,
}

impl GallWord {
    pub fn new(processed_word:ProcessedWord, loc:GallLoc, radius: f64, thick:f64) -> GallWord {
        let tainer_vec = Vec::with_capacity(processed_word.length);
        let mut word = GallWord{
            loc,
            tainer_vec,
            radius: Rc::new(Cell::new(radius)),
            thickness: Rc::new(Cell::new(thick))
        };
        word.populate(processed_word);
        word
    } 
    fn populate(&mut self, mut processed_word:ProcessedWord) {
        let word = processed_word.word;
        let tainer_ang = TAU/(processed_word.length as f64); 
        let mut con_count:usize = 0;
        let mut con = GallTainer::new(); // create new container
        for cha in word.chars() {
            let (mut l_mark, repeats) = gall_fn::stem_lookup(&cha);
            let d_mark = gall_fn::dot_lookup(&cha);
            //check if we can add to container
            if con.is_stateless() {
                if let LetterMark::Digit(num) = l_mark {
                    if let Some(neg) = processed_word.neg_digit.pop() {
                        l_mark = LetterMark::Digit(if neg {-num}else{num});
                    }
                }
                con_count = con.init(&l_mark,con_count,tainer_ang, self);
            } else {
                match &l_mark {
                    LetterMark::Stem(stem) => {
                        if (!Config::STACK && !con.is_empty()) || 
                        (Some(stem) != con.stem_type()) || (!con.vowel.is_empty()) {
                            self.tainer_vec.push(con);
                            con = GallTainer::new();
                            con_count = con.init(&l_mark,con_count,tainer_ang, self);
                        }
                    },
                    LetterMark::GallVowel(vow) => {
                        if !Config::STACK && !con.vowel.is_empty() {
                            self.tainer_vec.push(con);
                            con = GallTainer::new();
                            con_count = con.init(&l_mark,con_count,tainer_ang, self);
                        } else if !con.stem.is_empty() && vow == &VowelType::O2{
                            //con.populate_o1(repeat, &self);
                            continue;
                        }
                    },
                    LetterMark::Digit(num) => {
                        if con.stem_type() != Some(&StemType::J) || con.mark.is_empty() {
                            con = GallTainer::new();
                            con_count = con.init(&l_mark,con_count,tainer_ang, self);
                            if let Some(neg) = processed_word.neg_digit.pop() {
                                l_mark = LetterMark::Digit(if neg {-num}else{*num});
                            }
                        }
                    },
                    LetterMark::GallMark => {},
                }
            }
            //actually add to the container
            con.populate(l_mark, d_mark, repeats, &self)
        }
        self.tainer_vec.push(con);
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
            let movement = (right - left)/3.0;
            _ = con.bound_ccw_rotate(movement);
            if movement > change {
                change = movement
            }
            left = right;
        }
        let movement = (first - left)/3.0;
        _ = con1.bound_ccw_rotate(movement);
        if movement > change {
            change = movement
        }
        if change > Config::STEP_DIST {
            Some(())
        } else {
            None
        }
    }
    pub fn spread(&mut self) {
        while let Some(_) = self.even_tainer_spread() {};
    }
    pub fn collect_nodes(&mut self) -> Vec<&mut GallNode> {
        let mut nodes = Vec::new();
        for con in &mut self.tainer_vec {
            let mut node_list = con.collect_nodes();
            nodes.append(&mut node_list);
        }
        nodes
    }
    pub fn basic(&mut self) {
        for con in &mut self.tainer_vec {
            con.stem_sort();
        }
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
impl LocHolder for GallWord {
    fn loc(&self) -> &GallLoc {
        &self.loc
    }
    fn mut_loc(&mut self) -> &mut GallLoc {
        &mut self.loc
    }
}
impl OrdHolder for GallWord {
    fn ord(&self) -> &GallOrd {
        &self.loc.ord
    }
    fn mut_ord(&mut self) -> &mut GallOrd {
        &mut self.loc.ord
    }
}