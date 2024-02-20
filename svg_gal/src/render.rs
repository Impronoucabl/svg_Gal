use std::f64::consts::PI;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

use crate::gall_circle::{ChildCircle, Circle as Cir, HollowCircle};
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_stem::{Stem, StemType};
use crate::gall_tainer::GallTainer;
use crate::gall_word::GallWord;

pub trait Renderable {
    fn render(self, drawn:Document) -> Document;
}

trait SkelPart {
    fn part_init(&self) -> ((Data, Data),(f64,f64),(f64,f64),(f64,f64));
    fn part_render(&self, inner_outer:(Data,Data)) -> ((Data,Data),(f64,f64));
}

impl Renderable for GallWord {
    fn render(self, mut drawn:Document) -> Document {
        let radius = (self.inner_radius(),self.outer_radius());
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", self.thick())
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius());
        let (skel, divot, mark) = self.pre_render();
        drawn = GallWord::skel_render(skel, radius, drawn);
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
    fn skel_render(skel:Vec<GallTainer>, radius:(f64,f64), mut drawn:Document) -> Document {
        let (mut data,inner_join, outer_join, init_angles) = skel[0].part_init();
        let mut fin_ang: (f64,f64) = (0.0,0.0);
        for tainer in skel {
            (data, fin_ang) = tainer.part_render(data)
        };
        let inner_sweep = fin_ang.0 - init_angles.0 <= PI;
        let outer_sweep = fin_ang.1 - init_angles.1 <= PI;
        let closed_inner_loop = data.0.elliptical_arc_to((
            radius.0, radius.0,
            0,
            if inner_sweep {1} else {0},
            0,
            inner_join.0, inner_join.1,
        )).close();
        let closed_outer_loop = data.1.elliptical_arc_to((
            radius.1, radius.1,
            0,
            if outer_sweep {1} else {0},
            0,
            outer_join.0, outer_join.1
        )).close();
        let inner_path = Path::new()
            .set("d", closed_inner_loop);
        let outer_path = Path::new()
            .set("d", closed_outer_loop);
        drawn = drawn.add(outer_path
            .set("fill", "black")
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
    fn part_render(&self, inner_outer:(Data,Data)) -> ((Data,Data),(f64,f64)) {
        let (stem1, stem2) = self.stack_check();
        let (thi_inner,thi_outer) = self.thi_calc();
        let w_in_rad = stem1.parent_inner();
        let w_ou_rad = stem2.parent_outer();
        let l_in_rad = stem2.inner_radius();
        let l_ou_rad = stem1.outer_radius();
        let b_div_flag = match self.stem_type() {
            Some(s_type) => s_type == &StemType::B,
            None => {panic!()},
        };
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
        let final_in_ang = tracker.ang().unwrap();
        tracker.mut_ang_d(w_ou_rad, outer_word_end_angle);
        let outer_letter_start = tracker.svg_ord();
        let outer_letter_finish = tracker.compute_loc(2.0 * thi_outer);
        let final_ou_ang = tracker.ang().unwrap();
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
        (
            (inner_data,outer_data),
            (final_in_ang,final_ou_ang),
        )
    }
    fn part_init(&self) -> ((Data, Data),(f64,f64),(f64,f64), (f64,f64)) {
        let (stem1, stem2) = self.stack_check();
        let (thi_inner,thi_outer) = self.thi_calc();
        let inner_init_angle = stem1.ang().unwrap() - thi_inner;
        let outer_init_angle = stem2.ang().unwrap() - thi_outer;
        let mut tracker = GallLoc::new(
            0.0_f64.min(inner_init_angle),
            stem1.parent_inner(), 
            stem1.get_center(),
        );
        let inner_continuum = tracker.pos_ref().get();
        tracker.mut_ang_d(stem2.parent_outer(), 0.0_f64.min(outer_init_angle));
        let outer_continuum = tracker.pos_ref().get();
        (
            (
                Data::new().move_to(inner_continuum),
                Data::new().move_to(outer_continuum),
            ),
            inner_continuum,
            outer_continuum,
            (
                inner_init_angle,
                outer_init_angle,
            )
        )
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
