use std::f64::consts::PI;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

use crate::gall_circle::{ChildCircle, Circle as Cir, HollowCircle};
use crate::gall_config::Config;
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
    fn part_init(&self) -> ((Data, Data),(f64,f64),(f64,f64));
    fn part_render(&self, inner_outer:(Data,Data)) -> (Data,Data);
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
        let stem2 = if Config::STACK {
            skel[0].stem.last().unwrap()
        } else {
            stem1 // stem1 > stem2
        };
        let center_ref = stem1.get_center();
        let w_in_rad = stem1.parent_inner();
        let w_ou_rad = stem1.parent_outer();
        //let w_thick = stem1.parent_thick();
        let mut l_in_rad = stem2.inner_radius();
        let mut l_ou_rad = stem1.outer_radius();
        let mut l_dist1 = stem1.dist();
        let mut l_dist2 = stem2.dist();
        
        let mut b_div_flag = match stem1.stem_type {
            StemType::B => {1},
            StemType::S => {
                //l_dist += stem1.thick(); 
                0
            },//change to word.thick()?
            _ => {panic!()}
        };
        // if stem1.parent_inner() - l_dist - l_ou_rad >= 0.0 {
        //     panic!("Letter not touching outer skeleton");
        // }
        // if stem1.parent_outer() - l_dist - l_in_rad >= 0.0 {
        //     panic!("Letter not touching inner skeleton");
        // }
        let mut thi_inner = gall_fn::thi(
            l_dist1,
            l_ou_rad, 
            stem1.parent_inner(),
        );
        let mut thi_outer = gall_fn::thi(
            l_dist2,
            l_in_rad, 
            stem1.parent_outer(),
        );
        let mut inner_word_end_angle = stem1.ang().unwrap() - thi_inner;
        let mut outer_word_end_angle = stem2.ang().unwrap() - thi_outer;
        let inner_init_angle = 0.0_f64.min(inner_word_end_angle);
        let outer_init_angle = 0.0_f64.min(outer_word_end_angle);
        let mut inner_tracker = GallLoc::new(
            inner_init_angle,
            w_in_rad, 
            center_ref.clone(),
        );
        let mut outer_tracker = GallLoc::new(
            outer_init_angle,
            w_ou_rad,
            center_ref.clone(),
        );
        let inner_continuum = inner_tracker.pos_ref().get();
        let outer_continuum = outer_tracker.pos_ref().get();
        let mut inner_data = Data::new()
        .move_to(inner_continuum);  
        let mut outer_data = Data::new()
        .move_to(outer_continuum);  

        let mut long_inner_skeleton = if inner_word_end_angle > PI {1} else {0};
        let mut long_outer_skeleton = if outer_word_end_angle > PI {1} else {0};
        inner_tracker.mut_ang( inner_word_end_angle);
        outer_tracker.mut_ang( outer_word_end_angle);
        let mut inner_letter_start = inner_tracker.svg_ord();
        let mut outer_letter_start = outer_tracker.svg_ord();
        inner_tracker.mut_ccw(2.0 * thi_inner);
        outer_tracker.mut_ccw(2.0 * thi_outer);
        let mut inner_letter_finish = inner_tracker.svg_ord();
        let mut outer_letter_finish = outer_tracker.svg_ord();

        
        //TODO: Put this in part rend? -------------------------------
        for tainer in skel {
            //drawn = tainer.render(drawn);
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            inner_data = inner_data.elliptical_arc_to((
                w_in_rad,
                w_in_rad, 
                0,
                long_inner_skeleton,
                0,
                inner_letter_start.0,
                inner_letter_start.1
            ))
            .elliptical_arc_to((
                l_in_rad, 
                l_in_rad, 
                0,
                b_div_flag,
                1,
                outer_letter_finish.0,
                outer_letter_finish.1
            ));
        }

        let mut inner_sweep = if inner_tracker.ang().unwrap() - inner_init_angle > PI {0} else {1};
        let mut outer_sweep = if outer_tracker.ang().unwrap() - outer_init_angle > PI {0} else {1};
        let closed_inner_loop = inner_data
            .elliptical_arc_to((w_in_rad,w_in_rad,0,inner_sweep,0,inner_continuum.0,inner_continuum.1))
            .close();
        let closed_outer_loop = outer_data
            .elliptical_arc_to((w_ou_rad,w_ou_rad,0,outer_sweep,0,outer_continuum.0,outer_continuum.1))
            .close();
        let inner_path = Path::new()
            .set("d", closed_inner_loop);
        let outer_path = Path::new()
            .set("d", closed_outer_loop);
        drawn = drawn.add(outer_path
            .set("fill", "green")
            .set("stroke-width", 0.0)
            .set("stroke", "none")
        );  
        drawn = drawn.add(inner_path
            .set("fill", "green")
            .set("stroke-width", 0.0)
            .set("stroke", "none")
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

impl SkelPart for GallTainer {
    fn part_render(&self, inner_outer:(Data,Data)) -> (Data,Data) {
        let stem1 = self.stem.first().unwrap();
        let stem2 = if Config::STACK {
            self.stem.last().unwrap()
        } else {
            stem1 // stem1 > stem2
        };
        let w_in_rad = stem1.parent_inner();
        let w_ou_rad = stem1.parent_outer();
        let l_in_rad = stem2.inner_radius();
        let l_ou_rad = stem1.outer_radius();
        let b_div_flag = match self.stem_type() {
            Some(s_type) => s_type == &StemType::B,
            None => {panic!()},
        };
        let thi_inner = gall_fn::thi(
            stem1.dist(),
            l_ou_rad, 
            stem1.parent_inner(),
        );
        let thi_outer = gall_fn::thi(
            stem2.dist(),
            l_in_rad, 
            stem1.parent_outer(),
        );
        let inner_word_end_angle = stem1.ang().unwrap() - thi_inner;
        let outer_word_end_angle = stem2.ang().unwrap() - thi_outer;
        let long_inner_skeleton = inner_word_end_angle > PI;
        let long_outer_skeleton = outer_word_end_angle > PI;
        
        let mut tracker = GallLoc::new(
            inner_word_end_angle,
            w_in_rad, 
            stem1.get_center(),
        );
        let inner_letter_start = tracker.svg_ord();
        let inner_letter_finish = tracker.compute_loc(2.0 * thi_inner);
        tracker.mut_ang_d(w_ou_rad, outer_word_end_angle);
        let outer_letter_start = tracker.svg_ord();
        let outer_letter_finish = tracker.compute_loc(2.0 * thi_outer);
        // x radius, y radius, rotation, large arc, sweep direction, end x, end y
        let inner_data = inner_outer.0.elliptical_arc_to((
            w_in_rad, w_in_rad, 
            0,
            if long_inner_skeleton {1} else {0},
            0,
            inner_letter_start.0, inner_letter_start.1
        )).elliptical_arc_to((
            l_in_rad, l_in_rad, 
            0,
            if b_div_flag {1} else {0},
            1,
            inner_letter_finish.0, inner_letter_finish.1
        ));
        let outer_data = inner_outer.1.elliptical_arc_to((
            w_ou_rad, w_ou_rad, 
            0,
            if long_outer_skeleton {1} else {0},
            0,
            outer_letter_start.0, outer_letter_start.1
        )).elliptical_arc_to((
            l_ou_rad, l_ou_rad, 
            0,
            if b_div_flag {1} else {0},
            1,
            outer_letter_finish.0, outer_letter_finish.1
        ));
        (inner_data,outer_data)
    }

    fn part_init(&self) -> ((Data, Data),(f64,f64),(f64,f64)) {
        todo!()
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
