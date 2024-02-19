use std::f64::consts::PI;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

use crate::gall_circle::{Circle as GCircle, HollowCircle};
use crate::gall_fn;
use crate::gall_loc::Location;
use crate::gall_stem::{Stem, StemType};
use crate::gall_tainer::GallTainer;
use crate::gall_word::GallWord;

pub trait Renderable {
    fn render(self, drawn:Document) -> Document;
}

impl Renderable for GallWord {
    fn render(self, mut drawn:Document) -> Document {
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", self.thick())
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius());
        let (skel, divot, mark,) = self.pre_render();
        drawn = GallWord::skel_render(skel, drawn);
        for tainer in divot {
            drawn = tainer.render(drawn);
        }
        for tainer in mark {
            drawn = tainer.render(drawn);
        }
        drawn.add(circle)
    }
}

impl GallWord {
    fn pre_render(self) -> (Vec<GallTainer>,Vec<GallTainer>,Vec<GallTainer>) {
        let mut skel = Vec::new();
        let mut divot = Vec::new();
        let mut mark = Vec::new();
        for tainer in self.tainer_vec {
            match tainer.stem_type() {
                None => mark.push(tainer),
                Some(stem_type) => {
                    match stem_type {
                        StemType::J => divot.push(tainer),
                        StemType::B => skel.push(tainer),
                        StemType::S => skel.push(tainer),
                        StemType::Z => divot.push(tainer),
                    }
                }
            }
        }
        (skel,divot,mark)
    }
    fn skel_render(skel:Vec<GallTainer>, drawn:Document) -> Document {
        let clock = 0.0;
        let tain1 = &skel[0];
        //TODO: tain unpacker.
        
        let mut b_div_flag = 0;
        //let mut l_dist
        for tainer in skel {
            //drawn = tainer.render(drawn);
        }
        drawn
    }
}

impl Renderable for GallTainer {
    fn render(self, mut drawn:Document) -> Document {
        for stem in self.stem {
            drawn = stem.render(drawn);
        }
        drawn
    }
}

impl Renderable for Stem {
    fn render(self, drawn:Document) -> Document {
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", self.thick())
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius());
        drawn.add(circle)
    }
}