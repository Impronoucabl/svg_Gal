use std::cell::Cell;
use std::rc::Rc;

use crate::gall_circle::{Circle, HollowCircle};
use crate::gall_errors::Error;
use crate::gall_fn;
use crate::gall_loc::{GallLoc, LocHolder, Location};
use crate::gall_node::GallNode;
use crate::gall_ord::{GallOrd, OrdHolder, PolarOrdinate};
use crate::gall_word::GallWord;

pub struct GallSentence {
    loc: GallLoc,
    pub words:Vec<GallWord>,
    //pub dash_pairs: Vec<CircleGallLine>,
    radius:Rc<Cell<f64>>,
    thickness: Rc<Cell<f64>>,
}

impl GallSentence {
    pub fn new(loc:GallLoc, radius:f64, thickness: f64) -> GallSentence {
        GallSentence{
            loc,
            words: Vec::new(),
            radius: Rc::new(Cell::new(radius)),
            thickness: Rc::new(Cell::new(thickness)),
        }
    }
    pub fn generate(&mut self, word_list:Vec<(String,usize)>) {
        let word_len = word_list.len();
        for (num,words) in word_list.into_iter().enumerate() {
            let (w_radius, w_thick, word_ang, dist) = gall_fn::default_layouts(word_len, num);
            //create word struct
            let loc = GallLoc::new(
                word_ang,
                dist,
                self.pos_ref(),
            );
            self.words.push(GallWord::new(words.0,words.1, loc, w_radius, w_thick));
        }
        for wrd in &mut self.words {
            wrd.spread();
        }
    }
    pub fn collect_nodes(&mut self) -> Vec<&mut GallNode> {
        let mut nodes = Vec::new();
        for word in &mut self.words {
            let mut node_list = word.collect_nodes();
            nodes.append(&mut node_list);
        }
        nodes
    }
    pub fn basic(&mut self) {
        for word in &mut self.words {
            word.basic()
        }
    }
}

impl HollowCircle for GallSentence {
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
impl Circle for GallSentence {
    fn radius(&self) -> f64 {
        self.radius.get()
    }
    fn get_radius(&self) -> Rc<Cell<f64>> {
        self.radius.clone()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Error> {
        //self.check_radius(new_radius)?;
        self.radius.set(new_radius);
        Ok(())
    }
}
impl LocHolder for GallSentence {
    fn loc(&self) -> &GallLoc {
        &self.loc
    }
    fn mut_loc(&mut self) -> &mut GallLoc {
        &mut self.loc
    }
}
impl OrdHolder for GallSentence {
    fn ord(&self) -> &GallOrd {
        &self.loc.ord
    }
    fn mut_ord(&mut self) -> &mut GallOrd {
        &mut self.loc.ord
    }
}