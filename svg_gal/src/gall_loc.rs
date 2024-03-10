use std::cell::Cell;
use std::f64::consts::{FRAC_PI_2, PI};
use std::rc::Rc;

use crate::gall_ang::{self, GallAng};
use crate::gall_ord::{GallOrd, PolarOrdinate};
use crate::gall_errors::{Error, GallError};

trait LocHolder {
    fn loc(&mut self) -> GallLoc;
}
trait RelHolder {
    fn loc(&mut self) -> GallRelLoc;
}

#[derive(PartialEq,Default,Clone)]
pub struct GallLoc {
    ord: GallOrd,
    center_ref: Rc<Cell<(f64,f64)>>, // abs xy
    abs_svg: Rc<Cell<(f64,f64)>>,
}

pub struct GallRelLoc {
    angle: Rc<Cell<GallAng>>,
    ang_offset: f64,
    dist_offset: f64,
    letter_radius: Rc<Cell<f64>>,
    center_ref: Rc<Cell<(f64,f64)>>, // abs xy
    abs_svg: Rc<Cell<(f64,f64)>>,
}

pub trait Location:PolarOrdinate {
    fn mut_center(&mut self, movement:(f64,f64));
    fn set_center(&mut self, new_center:Rc<Cell<(f64,f64)>>);
    fn get_center(&self) -> Rc<Cell<(f64,f64)>>;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn update(&mut self);
    fn pos_ref(&self) -> Rc<Cell<(f64,f64)>>;
    fn svg_ord(&self) -> (f64,f64) {
        self.pos_ref().get()
    }
    fn sq_dist_2_loc<T:Location>(&self, loc2:&T) -> f64 {
        let (x1,y1) = self.svg_ord();
        let (x2,y2) = loc2.svg_ord();
        (x2 - x1).powi(2) + (y2 - y1).powi(2)
    }
    fn dist2loc<T:Location>(&self, loc2:&T) -> f64 {
        self.sq_dist_2_loc(loc2).sqrt()
    }
    fn ang2loc<T:Location>(&self, loc2:&T) -> f64 {
        let (x1,y1) = self.svg_ord();
        let (x2,y2) = loc2.svg_ord();
        gall_ang::constrain((y2-y1).atan2(x2-x1)+PI/2.0)
    }
    fn cent_ang2cent_ang<T:Location>(&self, loc2:&T) -> f64 {
        let (x1,y1) = self.get_center().get();
        let (x2,y2) = loc2.get_center().get();
        gall_ang::constrain((y2-y1).atan2(x2-x1)+PI/2.0)
    }
}

fn calc_xy(dist:f64, ang:Option<f64>,center:(f64,f64)) -> (f64,f64) {
    let (rel_y,rel_x) = match ang {
        Some(angle) => (FRAC_PI_2 - angle).sin_cos(),
        None => (0.0,0.0)
    };
    let (center_x,center_y) = center;
    (dist*rel_x + center_x, dist*rel_y + center_y)
}

impl GallLoc {
    pub fn new(angle:f64, distance: f64, center_ref:Rc<Cell<(f64,f64)>>) -> GallLoc {
        let pos = calc_xy(distance, Some(angle), center_ref.get());
        GallLoc { 
            ord: GallOrd::new(angle, distance),
            center_ref,  
            abs_svg: Rc::new(Cell::new(pos)),
        }
    }
    fn update_xy(&mut self) {
        self.abs_svg.set(calc_xy(self.dist(), self.ang(), self.center_ref.get()));
    }
    pub fn compute_loc(&mut self, ang:f64) -> (f64,f64) {
        _ = self.mut_ccw(ang);
        self.svg_ord()
    }
    // pub fn rotate_ccw(&mut self, angle: f64) -> Option<()> {
    //     self.mut_ang(self.ang() + angle);
    //     Some(())
    // }
    // pub fn rotate_cw(&mut self, angle: GallAng) -> Option<()> {
    //     self.rotate_ccw(GallAng{angle:Some(0.0)} - angle)
    // }
    // pub fn flip_ang(&mut self) {
    //     self.rotate_ccw(GallAng{angle:Some(PI)});
    // }
    // pub fn lengthen(&mut self, extra_dist:f64) {
    //     self.shorten(-extra_dist)
    // }
    // pub fn shorten(&mut self, extra_dist:f64) {
    //     let new_dist = self.get_dist() - extra_dist;
    //     if new_dist < 0.0 {
    //         self.mut_dist(-new_dist);
    //         self.flip_ang();
    //     } else {
    //         self.mut_dist(new_dist);
    //     }
    // } 
}

impl GallRelLoc {
    pub fn new(angle_ref:Rc<Cell<GallAng>>, ang_offset:f64, letter_radius:Rc<Cell<f64>>, dist_offset:f64, center_ref: Rc<Cell<(f64,f64)>>) -> GallRelLoc {
        let angle = match angle_ref.get().ang() {
            Some(ang) =>  Some(ang + ang_offset),
            None => None,
        };
        let pos = calc_xy(letter_radius.get(), angle, center_ref.get());
        GallRelLoc {
            angle: angle_ref,
            ang_offset,
            dist_offset,
            letter_radius,
            center_ref,
            abs_svg:Rc::new(Cell::new(pos)),
        }
    }
    fn update_xy(&mut self) {
        self.abs_svg.set(calc_xy(self.dist(), self.ang(), self.center_ref.get()));
    }
    fn base_ang(&self) -> Option<f64> {
        self.angle.get().ang()
    }
    pub fn set_dist(&mut self, dist_ref:Rc<Cell<f64>>) {
        self.letter_radius = dist_ref;
    }
    pub fn set_ang(&mut self, ang_ref: Rc<Cell<GallAng>>) {
        self.angle = ang_ref
    }
    
}

impl Location for GallLoc {
    fn mut_center(&mut self, movement:(f64,f64)) {
        let (center_x,center_y) = self.center_ref.get();
        self.center_ref.set((center_x + movement.0, center_y + movement.1));
        self.update_xy();
    }
    fn set_center(&mut self, new_center: Rc<Cell<(f64,f64)>>) {
        self.center_ref = new_center;
        self.update_xy();
    }
    fn get_center(&self) -> Rc<Cell<(f64,f64)>> {
        self.center_ref.clone()
    }
    fn x(&self) -> f64 {
        self.abs_svg.get().0
    }
    fn y(&self) -> f64 {
        self.abs_svg.get().1
    }
    fn pos_ref(&self) -> Rc<Cell<(f64,f64)>> {
        self.abs_svg.clone()
    }
    fn update(&mut self) {
        self.update_xy()
    }
}
impl Location for GallRelLoc {
    fn mut_center(&mut self, movement:(f64,f64)) {
        let (center_x,center_y) = self.center_ref.get();
        self.center_ref.set((center_x + movement.0, center_y + movement.1));
        self.update_xy();
    }
    fn set_center(&mut self, new_center: Rc<Cell<(f64,f64)>>) {
        self.center_ref = new_center;
        self.update_xy();
    }
    fn get_center(&self) -> Rc<Cell<(f64,f64)>> {
        self.center_ref.clone()
    }
    fn x(&self) -> f64 {
        self.abs_svg.get().0
    }
    fn y(&self) -> f64 {
        self.abs_svg.get().1
    }
    fn pos_ref(&self) -> Rc<Cell<(f64,f64)>> {
        self.abs_svg.clone()
    }
    fn update(&mut self) {
        self.update_xy()
    }
}

impl PolarOrdinate for GallLoc {
    fn mut_ang(&mut self, new_angle:f64) {
        self.ord.mut_ang(new_angle);
        self.update_xy();
    }
    fn mut_dist(&mut self, new_dist:f64)-> Result<(), Error> {
        match self.ord.mut_dist(new_dist) {
            Ok(_) => Ok(self.update_xy()),
            Err(e) => Err(e),
        }
    }
    fn ang(&self) -> Option<f64> {
        self.ord.ang()
    }
    fn dist(&self) -> f64 {
        self.ord.dist()
    }
    fn get_dist(&self) -> Rc<Cell<f64>> {
        self.ord.get_dist()
    }
}

impl PolarOrdinate for GallRelLoc {
    fn mut_ang(&mut self, ang:f64) {
        if let (Some(old_ang), Some(new_ang)) = (
            self.base_ang(), gall_ang::constrain_opt(Some(ang))) {
            self.ang_offset = new_ang - old_ang;
            self.update_xy();
        } else {}//Don't panic if base ang is None
        //self.update_xy();
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error> {
        if new_dist.is_sign_negative() {
            return Err(Error::new(GallError::NegativeDistanceErr))
        }
        self.dist_offset = new_dist - self.letter_radius.get();
        Ok(self.update_xy())
    }
    fn ang(&self) -> Option<f64> {
        if let Some(ang) = self.base_ang() {
            Some(self.ang_offset + ang)
        } else {
            None
        }
    }
    fn dist(&self) -> f64 {
        self.dist_offset + self.letter_radius.get()
    }
    fn get_dist(&self) -> Rc<Cell<f64>> {
        todo!()
        //self.letter_distance.clone()
    }
}