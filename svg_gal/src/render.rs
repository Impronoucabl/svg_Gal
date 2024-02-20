use std::f64::consts::PI;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

use crate::gall_circle::{ChildCircle, Circle as GCircle, HollowCircle};
use crate::gall_fn;
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_stem::{Stem, StemType};
use crate::gall_tainer::GallTainer;
use crate::gall_word::GallWord;

pub trait Renderable {
    fn render(self, drawn:Document) -> Document;
}

trait SkelPart {
    fn part_render(self, start:(f64,f64)) -> (Path,(f64,f64));
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
        let (skel, divot, mark) = self.pre_render();
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
    fn skel_render(skel:Vec<GallTainer>, mut drawn:Document) -> Document {
        let stem1 = skel[0].stem.first().unwrap();
        let center_ref = stem1.get_center();
        let w_rad = stem1.parent_radius();
        //let w_in_rad = stem1.parent_inner();
        //let w_ou_rad = stem1.parent_outer();
        let mut l_rad = stem1.radius();
        //let mut l_in_rad = stem1.inner_radius();
        //let mut l_ou_rad = stem1.outer_radius();
        let mut l_dist = stem1.dist();
        
        let mut b_div_flag = match stem1.stem_type {
            StemType::B => {1},
            StemType::S => {l_dist += stem1.thick(); 0},//change to word.thick()?
            _ => {panic!()}
        };
        // if stem1.parent_inner() - l_dist - l_ou_rad >= 0.0 {
        //     panic!("Letter not touching outer skeleton");
        // }
        // if stem1.parent_outer() - l_dist - l_in_rad >= 0.0 {
        //     panic!("Letter not touching inner skeleton");
        // }
        let mut thi = gall_fn::thi(
            l_dist,
            l_rad, 
            w_rad,
        );
        // let mut thi_inner = gall_fn::thi(
        //     l_dist,
        //     l_ou_rad, 
        //     stem1.parent_inner(),
        // );
        // let mut thi_outer = gall_fn::thi(
        //     l_dist,
        //     l_in_rad, 
        //     stem1.parent_outer(),
        // );
        let mut word_end_angle = stem1.ang().unwrap() - thi;
        //let mut inner_word_end_angle = stem1.ang().unwrap() - thi_inner;
        //let mut outer_word_end_angle = stem1.ang().unwrap() - thi_outer;
        let init_angle = 0.0_f64.min(word_end_angle);
        // let inner_init_angle = 0.0_f64.min(inner_word_end_angle);
        // let outer_init_angle = 0.0_f64.min(outer_word_end_angle);
        let mut tracker = GallLoc::new(
            init_angle,
            w_rad,
            center_ref,
        );
        // let mut inner_tracker = GallLoc::new(
        //     inner_init_angle,
        //     w_in_rad,
        //     center_ref,
        // );
        // let mut outer_tracker = GallLoc::new(
        //     outer_init_angle,
        //     w_out_rad,
        //     center_ref,
        // );
        let continuum = tracker.svg_ord();
        // let inner_continuum = inner_tracker.pos_ref().get();
        // let outer_continuum = outer_tracker.pos_ref().get();
        let mut long_skeleton = if word_end_angle > PI {1} else {0};
        //let mut long_inner_skeleton = if inner_word_end_angle > PI {1} else {0};
        //let mut long_outer_skeleton = if outer_word_end_angle > PI {1} else {0};
        tracker.mut_ang(word_end_angle);
        // inner_tracker.mut_ang( inner_word_end_angle);
        // outer_tracker.mut_ang( outer_word_end_angle);
        let mut letter_start = tracker.svg_ord();
        // let mut inner_letter_start = inner_tracker.svg_ord();
        // let mut outer_letter_start = outer_tracker.svg_ord();
        tracker.mut_ccw(2.0 * thi);
        // inner_tracker.mut_ccw(2.0 * thi_inner);
        // outer_tracker.mut_ccw(2.0 * thi_outer);
        let mut letter_finish = tracker.svg_ord();
        // let mut inner_letter_finish = inner_tracker.svg_ord();
        // let mut outer_letter_finish = outer_tracker.svg_ord();

        //TODO: Put this in part rend? -------------------------------
        let mut data = Data::new()
        .move_to(continuum)
        // x radius, y radius, rotation, large arc, sweep direction, end x, end y
        .elliptical_arc_to((
            w_rad,
            w_rad, 
            0,
            long_skeleton,
            0,
            letter_start.0,
            letter_start.1
        ))
        .elliptical_arc_to((
            l_rad, 
            l_rad, 
            0,
            b_div_flag,
            1,
            letter_finish.0,
            letter_finish.1
        ));

        for tainer in skel {
            //drawn = tainer.render(drawn);
        }

        let sweep = if tracker.ang().unwrap() - init_angle > PI {0} else {1};
        // let mut inner_sweep = if inner_tracker.ang().unwrap() - inner_init_angle > PI {0} else {1};
        // let mut outer_sweep = if outer_tracker.ang().unwrap() - outer_init_angle > PI {0} else {1};
        let closed_loop = data
            .elliptical_arc_to((w_rad,w_rad,0,sweep,0,continuum.0,continuum.1))
            .close();
        let path = Path::new()
            .set("d", closed_loop);
        // let inner_path = Path::new()
        //     .set("d", closed_inner_loop);
        // let outer_path = Path::new()
        //     .set("d", closed_outer_loop);
        drawn = drawn.add(path
            .set("fill", "green")
            .set("stroke-width", 5.0)
            .set("stroke", "blue")
        );  
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
impl SkelPart for Stem {
    fn part_render(self, start:(f64,f64)) -> (Path,(f64,f64)) {
        todo!()
    }
}