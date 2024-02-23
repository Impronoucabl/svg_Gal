use std::f64::consts::PI;

use svg::{Document, Node};
use svg::node::element::{Circle, Element, Path, SVG};
use svg::node::element::path::Data;

use crate::gall_circle::{ChildCircle, Circle as Cir, Dot, HollowCircle};
use crate::gall_config::Config;
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;
use crate::gall_stem::{Stem, StemType};
use crate::gall_tainer::GallTainer;
use crate::gall_vowel::GallVowel;
use crate::gall_word::GallWord;

pub trait Renderable {
    fn render(self, drawn:Document) -> Document; 
}

trait SkelPart {
    fn part_init(&self) -> ((Data, Data),(f64,f64),(f64,f64),(f64,f64));
    fn part_render(&self, inner_outer:(Data,Data), start_ang:(f64,f64)) -> ((Data,Data),(f64,f64));
}

trait FreeRender {
    fn post_render(&self, vec:&mut Vec<Element>);
}

impl Renderable for GallWord {
    fn render(self, mut drawn:Document) -> Document {
        let radius = (self.inner_radius(),self.outer_radius());
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", Config::SKEL_COLOUR())
            .set("stroke-width", self.thick())
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius());
        let (skel, divot, mark) = self.pre_render();
        drawn = if skel.len() == 0 {
            drawn.add(circle)
        } else {
            GallWord::skel_render(skel, radius, drawn)
        };
        for tainer in divot {
            drawn = tainer.render(drawn);
        }
        for tainer in mark {
            drawn = tainer.render(drawn);
        }
        drawn
    }
}

impl GallWord {
    fn pre_render(self) -> (Vec<GallTainer>,Vec<GallTainer>,Vec<GallTainer>) {
        let mut skel = Vec::new();
        let mut divot = Vec::new();
        let mut mark = Vec::new();
        for tainer in self.tainer_vec {
            match tainer.stem_type() {
                None => {
                    if tainer.vowel.is_empty() {
                        mark.push(tainer)
                    } else {
                        divot.push(tainer)
                    }
                }
                Some(stem_type) => {
                    match stem_type {
                        StemType::J => divot.push(tainer),
                        StemType::B => skel.push(tainer),
                        StemType::S => skel.push(tainer),
                        StemType::Z => divot.push(tainer),
                    }
                },
            }
        }
        (skel,divot,mark)
    }
    fn skel_render(skel:Vec<GallTainer>, radius:(f64,f64), mut drawn:Document) -> Document {
        let (mut data,inner_join, outer_join, init_angles) = skel[0].part_init();
        let mut fin_ang: (f64,f64) = init_angles;
        let mut post_render = Vec::new();
        for tainer in skel {
            (data, fin_ang) = tainer.part_render(data, fin_ang);
            tainer.post_render(&mut post_render); //render non-skel stems
        };
        let (inner_sweep, outer_sweep) = (
            (init_angles.0 - fin_ang.0).abs() <= PI,
            (init_angles.1 - fin_ang.1).abs() <= PI,
        );
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
            .set("fill", Config::SKEL_COLOUR())
            .set("stroke-width", 0.0)
            .set("stroke", "none")
        );  
        drawn = drawn.add(inner_path
            .set("fill", Config::BG2_COLOUR())
            .set("stroke-width", 0.0)
            .set("stroke", "none")
        );
        for differed in post_render {
            drawn = drawn.add(differed);
        }  
        drawn
    }
}

impl Renderable for GallTainer {
    fn render(self, mut drawn:Document) -> Document {
        for stem in self.stem {
            drawn = stem.render(drawn);
        }
        for vow in self.vowel {
            drawn = vow.render(drawn);
        }
        for dot in self.dot {
            drawn = dot.render(drawn);
        }
        drawn
    }
}

impl SkelPart for GallTainer {
    fn part_render(&self, inner_outer:(Data,Data), start_ang:(f64,f64)) -> ((Data,Data),(f64,f64)) {
        let (stem1, stem2) = self.stack_check();
        let (thi_inner,thi_outer) = self.thi_calc();
        let (theta_inner,theta_outer) = self.theta_calc();
        if thi_inner.is_nan() || thi_outer.is_nan() {
            println!("Skeleton letter not touching skeleton");
            panic!();
        };
        let (w_in_rad, w_ou_rad) = (
            stem1.parent_inner(), stem2.parent_outer());
        let (l_in_big_rad, l_ou_smal_rad) = (
            stem1.outer_radius(), stem2.inner_radius());
        let (big_inner_l_arc, big_outer_l_arc) = (
            2.0*theta_inner < PI, 2.0*theta_outer < PI);
        let (inner_word_end_angle, outer_word_end_angle) = (
            stem1.ang().unwrap() - thi_inner, 
            stem2.ang().unwrap() - thi_outer
        );
        let (long_inner_skeleton, long_outer_skeleton) = (
            (inner_word_end_angle - start_ang.0).abs() > PI,
            (outer_word_end_angle - start_ang.1).abs() > PI
        );
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
            l_in_big_rad, l_in_big_rad, 
            0,
            if big_inner_l_arc {1} else {0},
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
            l_ou_smal_rad, l_ou_smal_rad, 
            0,
            if big_outer_l_arc {1} else {0},
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
        let (inner_init_angle, outer_init_angle) = (
            stem1.ang().unwrap() - thi_inner,
            stem2.ang().unwrap() - thi_outer
        );
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
impl GallTainer {
    fn post_render(self, vec: &mut Vec<Element>) {
        for stem in self.stem {
            stem.post_render(vec);
        }
        for vow in self.vowel {
            vow.post_render(vec);
        }
        for dot in self.dot {
            dot.post_render(vec);
        }
    } 
}

impl Renderable for Stem {
    fn render(self, drawn:Document) -> Document {
        match self.get_shape() {
            Some(circle) => drawn.add(circle),
            None => drawn
        }
    }
}

impl FreeRender for Stem {
    fn post_render(&self, vec:&mut Vec<Element>) {
        if let Some(circle) = self.get_shape() {
            vec.push(circle.into())
        }
    }
}
impl Stem {
    fn get_shape(&self) -> Option<Circle> {
        match self.stem_type {
            StemType::J|StemType::Z => {
                let circle = Circle::new()
                    .set("fill", "none")
                    .set("stroke", Config::JZ_COLOUR())
                    .set("stroke-width", (self.thick()*2.0).to_string()+"px")
                    .set("cx", self.x())
                    .set("cy", self.y())
                    .set("r", self.radius());
                Some(circle)
            },
            StemType::B|StemType::S => None,//TODO: Stack gaps
        }
    }
}
impl Renderable for GallVowel {
    fn render(self, drawn:Document) -> Document {
        drawn.add(self.get_shape())
    }
}
impl FreeRender for GallVowel {
    fn post_render(&self, vec:&mut Vec<Element>) {
        vec.push(self.get_shape().into())
    }
}
impl GallVowel {
    fn get_shape(&self) -> Circle {
        Circle::new()
            .set("fill", "none")
            .set("stroke", Config::VOW_COLOUR())
            .set("stroke-width", (self.thick()*2.0).to_string()+"px")
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius())
    }
}
impl Renderable for Dot {
    fn render(self, drawn:Document) -> Document {
        drawn.add(self.get_shape())
    }
}
impl FreeRender for Dot {
    fn post_render(&self, vec:&mut Vec<Element>) {
        vec.push(self.get_shape().into())
    }
}
impl Dot {
    fn get_shape(&self) -> Circle {
        Circle::new()
            .set("fill", Config::DOT_COLOUR())
            .set("stroke", "none")
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius())
    }
}