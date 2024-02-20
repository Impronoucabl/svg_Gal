use std::cell::Cell;
use std::f64::consts::{FRAC_PI_2, TAU};
use std::rc::Rc;

use crate::gall_ord::{GallOrd, PolarOrdinate};
use crate::gall_errors::{Error, GallError};

#[derive(PartialEq,Default,Clone)]
pub struct GallLoc {
    pub ord: GallOrd,
    center_ref: Rc<Cell<(f64,f64)>>, // abs xy
    abs_svg: Rc<Cell<(f64,f64)>>,
}

pub trait Location:PolarOrdinate {
    fn mut_center(&mut self, movement:(f64,f64));
    fn set_center(&mut self, new_center:Rc<Cell<(f64,f64)>>);
    fn get_center(&self) -> Rc<Cell<(f64,f64)>>;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn pos_ref(&self) -> Rc<Cell<(f64,f64)>>;
    fn svg_ord(&self) -> (f64,f64) {
        self.pos_ref().get()
    }
}

impl GallLoc {
    pub fn new(angle:f64, distance: f64, center_ref:Rc<Cell<(f64,f64)>>) -> GallLoc {
        let (rel_y,rel_x) = (FRAC_PI_2 - angle).sin_cos();
        let (center_x,center_y) = center_ref.get();
        let pos = (distance*rel_x + center_x, distance*rel_y + center_y);
        GallLoc { 
            ord: GallOrd::new(angle, distance),
            center_ref,  
            abs_svg: Rc::new(Cell::new(pos)),
        }
    }
    fn update_xy(&mut self) {
        let dist = self.dist();
        let (rel_y,rel_x) = match self.ang() {
            Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
            None => (0.0,0.0)
        };
        let (center_x,center_y) = self.center_ref.get();
        self.abs_svg.set((dist*rel_x + center_x, dist*rel_y + center_y));
    }
    pub fn compute_loc(&mut self, ang:f64) -> (f64,f64) {
        self.mut_ccw(ang);
        self.svg_ord()
    }
    
    // pub fn rotate_ccw(&mut self, angle: f64) -> Option<()> {
    //     self.mut_ang(self.ang() + angle);
    //     Some(())
    // }
    // pub fn rotate_cw(&mut self, angle: GallAng) -> Option<()> {
    //     self.rotate_ccw(GallAng{angle:Some(0.0)} - angle)
    // }
    // pub fn step_ccw(&mut self) -> Option<()> {
    //     let new_angle = self.get_ang().angle? + FRAC_PI_8/8.0;
    //     if new_angle < TAU {
    //         self.mut_ang(GallAng{angle:Some(new_angle)});
    //         Some(())
    //     } else {
    //         None
    //     }
    // }
    // pub fn step_cw(&mut self) -> Option<()> {
    //     let new_angle = self.get_ang().angle? - FRAC_PI_8/8.0;
    //     if new_angle >= 0.0 {
    //         self.mut_ang(GallAng{angle:Some(new_angle)});
    //         Some(())
    //     } else {
    //         None
    //     }
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

impl Location for GallLoc {
    fn mut_center(&mut self, movement:(f64,f64)) {
        let (center_x,center_y) = self.center_ref.get();
        self.center_ref.set((center_x + movement.0, center_y + movement.1));
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
}

impl PolarOrdinate for GallLoc {
    fn mut_ang(&mut self, new_angle:f64) {
        self.ord.mut_ang(new_angle);
        self.update_xy();
    }
    fn mut_dist(&mut self, new_dist:f64)-> Result<(), Error> {
        match self.ord.mut_dist(new_dist) {
            Ok(_) => Ok(self.update_xy()),
            Err(E) => Err(E),
        }
    }
    fn ang(&self) -> Option<f64> {
        self.ord.ang()
    }
    fn dist(&self) -> f64 {
        self.ord.dist()
    }
}