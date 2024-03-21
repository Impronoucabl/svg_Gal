use std::f64::consts::{PI, TAU};

use svg::Document;
use svg::node::element::{Circle, Element, Line, Path, Rectangle};
use svg::node::element::path::Data;

use crate::gall_circle::{ChildCircle, Circle as Cir, Dot, HollowCircle};
use crate::gall_config::Config;
use crate::gall_errors::Error;
use crate::gall_loc::{GallLoc, Location};
use crate::gall_node::GallNode;
use crate::gall_ord::PolarOrdinate;
use crate::gall_pair::{GallLine, GallLinePair};
use crate::gall_sentence::GallSentence;
use crate::gall_stem::{Stem, StemType};
use crate::gall_tainer::GallTainer;
use crate::gall_vowel::GallVowel;
use crate::gall_word::GallWord;

pub trait Renderable {
    fn render(self, drawn:Document) -> Document; 
}

trait SkelPart {
    fn part_init(&self) -> ((Data, Data),(f64,f64),(f64,f64),(f64,f64));
    fn part_render(&self, inner_outer:(Data,Data), start_ang:(f64,f64)) -> Result<((Data,Data),(f64,f64)), Error>;
}

trait FreeRender {
    fn post_render(&self, vec:&mut Vec<Element>);
}

trait Basic {
    fn get_shape(&self) -> Element;
}
impl<T:Basic> Renderable for T {
    fn render(self, drawn:Document) -> Document {
        drawn.add(self.get_shape())
    }
}
impl<T:Basic> FreeRender for T {
    fn post_render(&self, vec:&mut Vec<Element>) {
        vec.push(self.get_shape().into())
    }
}

pub fn create_svg() -> Document {
    let drawn = Document::new().set("viewBox", (0, 0, Config::WIDTH, Config::HEIGHT));   
    if Config::ENABLE_CANVAS {
        let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", Config::WIDTH)
        .set("height", Config::HEIGHT)
        .set("fill", Config::CANVAS_COLOUR())
        .set("stroke", "none");
        drawn.add(background)
    } else {
        drawn
    }
    
}

pub fn render_init(pairs:Vec<GallLinePair>, lines: Vec<GallLine>) -> (Document,Vec<Element>) {
    let mut post_render = Vec::new();
    for pair in pairs {
        pair.post_render(&mut post_render)
    }
    for line in lines {
        line.post_render(&mut post_render)
    }
    (create_svg(),post_render)
}

pub fn render_start<T:Renderable>(start_obj:T, drawn:Document) -> Document {
    start_obj.render(drawn)
}
pub fn render_post(post_render:Vec<Element>, mut drawn: Document) -> Document {
    for differed in post_render {
        drawn = drawn.add(differed);
    }  
    drawn
}

impl Renderable for GallSentence {
    fn render(self, mut drawn:Document) -> Document {
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", Config::SENT_SKEL_COLOUR())
            .set("stroke-width", 2.0*self.thick())
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius());
        let filled_circle = Circle::new()
            .set("fill", Config::SENT_COLOUR())
            .set("stroke", "none")
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.inner_radius());
        drawn = drawn.add(filled_circle);
        for word in self.words.into_iter() {
            drawn = word.render(drawn);
        }
        drawn.add(circle)
    }
}

impl Renderable for GallWord {
    fn render(self, mut drawn:Document) -> Document {
        let radius = (self.inner_radius(),self.outer_radius());
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", Config::SKEL_COLOUR())
            .set("stroke-width", self.thick()*2.0)
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius());
        let (skel, divot, mark) = self.pre_render();
        drawn = if skel.len() == 0 {
            drawn.add(circle)
        } else {
            GallWord::skel_render(skel, radius, drawn).unwrap()
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
    fn skel_render(skel:Vec<GallTainer>, radius:(f64,f64), mut drawn:Document) -> Result<Document, Error> {
        let (mut data,inner_join, outer_join, init_angles) = skel[0].part_init();
        let mut fin_ang: (f64,f64) = init_angles;
        let mut post_render = Vec::new();
        for tainer in skel {
            (data, fin_ang) = tainer.part_render(data, fin_ang)?;
            tainer.stack_render(&mut post_render); // render skel letter gaps
            tainer.post_render(&mut post_render); //render non-skel stems
        };
        let (inner_sweep, outer_sweep) = (
            (TAU + init_angles.0 - fin_ang.0) >= PI,
            (TAU + init_angles.1 - fin_ang.1) >= PI
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
            .set("fill", Config::WRD_COLOUR())
            .set("stroke-width", 0.0)
            .set("stroke", "none")
        );
        for differed in post_render {
            drawn = drawn.add(differed);
        }  
        Ok(drawn)
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
        if Config::NODE_VISIBILITY {
            for node in self.node {
                drawn = node.render(drawn);
            }
        }
        drawn
    }
}

impl SkelPart for GallTainer {
    fn part_render(&self, inner_outer:(Data,Data), start_ang:(f64,f64)) -> Result<((Data,Data),(f64,f64)), Error> {
        //TODO: Split the rendering between Bs & Ts - T is fine as is, but B to render only the largest stem
        let (stem1, stem2) = self.stack_check()?;
        let (thi_inner,thi_outer) = self.thi_calc()?;
        let (theta_inner,theta_outer) = self.theta_calc()?;
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
        Ok((
            (inner_data,outer_data),
            (final_in_ang,final_ou_ang),
        ))
    }
    fn part_init(&self) -> ((Data, Data),(f64,f64),(f64,f64), (f64,f64)) {
        let (stem1, stem2) = self.stack_check().expect("Tainer is empty"); 
        let (thi_inner,thi_outer) = (stem1.inner_thi().expect(""),stem2.outer_thi().expect(""));
        let (inner_init_angle, outer_init_angle) = (
            0.0_f64.min(stem1.ang().unwrap() - thi_inner),
            0.0_f64.min(stem2.ang().unwrap() - thi_outer)
        );
        let mut tracker = GallLoc::new(
            inner_init_angle,
            stem1.parent_inner(), 
            stem1.get_center(),
        );
        let inner_continuum = tracker.pos_ref().get();
        tracker.mut_ang_d(stem2.parent_outer(), outer_init_angle);
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
impl FreeRender for GallTainer {
    fn post_render(&self, vec: &mut Vec<Element>) {
        for stem in &self.stem {
            stem.post_render(vec);
        }
        for vow in &self.vowel {
            vow.post_render(vec);
        }
        for dot in &self.dot {
            dot.post_render(vec);
        }
        for node in &self.node {
            node.post_render(vec);
        }
    } 
}

impl GallTainer {
    fn stack_render(&self, vec: &mut Vec<Element>) {
        if self.stem.is_empty() {
            return
        }
        let stem = self.stem.first().expect("There should be more than 1 stem");
        let ang = self.ang();
        let (is_b, dist, thi2, theta2, colour) = if self.stem_type() == Some(&StemType::B) {
            (true, stem.parent_outer(), stem.outer_thi().unwrap(), stem.outer_theta2().unwrap(), Config::SENT_COLOUR())
        } else {
            (false, stem.parent_inner(), stem.inner_thi2().unwrap(), stem.inner_theta2().unwrap(), Config::WRD_COLOUR())
        };
        let mut tracker = GallLoc::new(
            ang,
            dist,
            stem.get_center()
        );
        let mut first = true;
        tracker.mut_ang(ang - thi2);
        let mut pos1 = tracker.pos_ref().get();
        tracker.mut_ang(ang + thi2);
        let pos2 = tracker.pos_ref().get();
        let mut path = Path::new()
            .set("fill", colour)
            .set("stroke-width", 0.0)
            .set("stroke", "none");
        let mut data = Data::new()
            .move_to(pos1)
            .elliptical_arc_to((
                stem.inner_radius(), stem.inner_radius(), 
                0,
                if theta2 * 2.0 > PI {0} else {1},
                1,
                pos2.0, pos2.1,
            ));
        for stem in &self.stem {
            if first {
                first = false;
                continue;
            }
            let (thi, thi2, theta, theta2) = if is_b {
                (
                    stem.outer_thi2().unwrap(), 
                    stem.outer_thi().unwrap(), 
                    stem.outer_theta().unwrap(), 
                    stem.outer_theta2().unwrap()
                )
            } else {
                (
                    stem.inner_thi().unwrap(), 
                    stem.inner_thi2().unwrap(), 
                    stem.inner_theta().unwrap(), 
                    stem.inner_theta2().unwrap())
            };
            tracker.mut_ang(ang + thi);
            let pos3 = tracker.pos_ref().get();
            tracker.mut_ang(ang - thi);
            let pos4 = tracker.pos_ref().get();
            data = data
                //.line_to(pos3)
                .elliptical_arc_to((
                    dist, dist,
                    0,0,1,
                    pos3.0, pos3.1
                ))
                .elliptical_arc_to((
                    stem.outer_radius(),stem.outer_radius(), 
                    0, 
                    if theta* 2.0 > PI {0} else {1},
                    0,
                    pos4.0, pos4.1
                ))
                .elliptical_arc_to((
                    dist, dist,
                    0,0,1,
                    pos1.0, pos1.1
                ))
                //.line_to(pos1)
                .close();
            path = path.set("d", data);
            vec.push(path.into());
            tracker.mut_ang(ang - thi2);
            pos1 = tracker.pos_ref().get();
            tracker.mut_ang(ang + thi2);
            let pos2 = tracker.pos_ref().get();
            path = Path::new()
                .set("fill", colour)
                .set("stroke-width", 0.0)
                .set("stroke", "none");
            data = Data::new().move_to(pos1).elliptical_arc_to((
                stem.inner_radius(), stem.inner_radius(),  
                0, 
                if theta2 * 2.0 > PI {0} else {1},
                1,
                pos2.0, pos2.1,
            ));
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
            vec.push(circle)
        }
    }
}
impl Stem {
    fn get_shape(&self) -> Option<Element> {
        match self.stem_type {
            StemType::J|StemType::Z => {
                let circle = Circle::new()
                    .set("fill", "none")
                    .set("stroke", Config::JZ_COLOUR())
                    .set("stroke-width", (self.thick()*2.0).to_string()+"px")
                    .set("cx", self.x())
                    .set("cy", self.y())
                    .set("r", self.radius())
                    .into();
                Some(circle)
            },
            StemType::B|StemType::S => None,
        }
    }
}

impl Basic for GallVowel {
    fn get_shape(&self) -> Element {
        Circle::new()
            .set("fill", "none")
            .set("stroke", Config::VOW_COLOUR())
            .set("stroke-width", (self.thick()*2.0).to_string()+"px")
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius())
            .into()
    }
}

impl Basic for Dot {
    fn get_shape(&self) -> Element {
        Circle::new()
            .set("fill", Config::DOT_COLOUR())
            .set("stroke", "none")
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", self.radius())
            .into()
    }
}

impl Basic for GallLine<'_> {
    fn get_shape(&self) -> Element {
        let (x,y) = self.get_endpoint();
        Line::new()
            .set("stroke", Config::SKEL_COLOUR())
            .set("stroke-width", self.thickness*2)
            .set("x1", self.node.x())
            .set("y1", self.node.y())
            .set("x2", x)
            .set("y2", y)
            .into()
    }
}

impl Basic for GallLinePair<'_> {
    fn get_shape(&self) -> Element {
        Line::new()
            .set("stroke", Config::SKEL_COLOUR())
            .set("stroke-width", self.thickness*2)
            .set("x1", self.node1.x())
            .set("y1", self.node1.y())
            .set("x2", self.node2.x())
            .set("y2", self.node2.y())
            .into()
    }
}

//------ TOGGLE NODE_VISIBILITY in Config ------

impl Basic for GallNode {
    fn get_shape(&self) -> Element {
        Circle::new()
            .set("fill", Config::DEBUG_COLOUR())
            .set("stroke", "none")
            .set("cx", self.x())
            .set("cy", self.y())
            .set("r", Config::DOT_RADIUS*0.8)
            .into()
    }
}