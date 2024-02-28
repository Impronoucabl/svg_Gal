use std::cell::OnceCell;
use std::f64::consts::{FRAC_PI_2, TAU};
use std::rc::Rc;

use crate::gall_ord::{GallOrd, PolarOrdinate};
use crate::gall_errors::{Error, GallError};

#[derive(PartialEq,Default,Clone)]
pub struct GallLoc {
    ord: GallOrd,
    center_ref: Rc<OnceCell<(f64,f64)>>, // abs xy
    abs_svg: Rc<OnceCell<(f64,f64)>>,
}

pub struct GallOffLoc {
    ord: GallOrd,
    ang_offset: f64,
    dist_offset: f64,
    center_ref: Rc<OnceCell<(f64,f64)>>, // abs xy
    abs_svg: Rc<OnceCell<(f64,f64)>>,
}

pub trait Location:PolarOrdinate {
    fn mut_center(&mut self, movement:(f64,f64));
    fn set_center(&mut self, new_center:Rc<OnceCell<(f64,f64)>>);
    fn get_center(&self) -> Rc<OnceCell<(f64,f64)>>;
    fn center_ords(&self) -> (f64,f64);
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn pos_ref(&self) -> Rc<OnceCell<(f64,f64)>>;
    fn svg_ord(&self) -> Result<(f64,f64), Error> {
        match self.pos_ref().get() {
            Some(xy) => Ok(*xy),
            None => Err(Error::new(GallError::ValueNotSet)),
        }
    }
}

impl GallLoc {
    pub fn new(angle:f64, distance: f64, center_ref:Rc<OnceCell<(f64,f64)>>) -> Option<GallLoc> {
        let (rel_y,rel_x) = (FRAC_PI_2 - angle).sin_cos();
        let (center_x,center_y) = center_ref.get()?;
        let pos = (distance*rel_x + center_x, distance*rel_y + center_y);
        let mut abs = OnceCell::new();
        abs.set(pos);
        Some(GallLoc { 
            ord: GallOrd::new(angle, distance),
            center_ref,  
            abs_svg: Rc::new(abs),
        })
    }
    fn update_xy(&mut self) {
        if let Some(dist) = self.dist() {
            let (rel_y,rel_x) = match self.ang() {
                Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
                None => (0.0,0.0)
            };
            let (center_x,center_y) = self.center_ref.get().unwrap();
            self.abs_svg.set((dist*rel_x + center_x, dist*rel_y + center_y));
        } else {
            panic!()
        }
    }
    pub fn compute_loc(&mut self, ang:f64) -> Result<(f64,f64), Error> {
        self.mut_ccw(ang)?;
        self.svg_ord()
    }
    pub fn mut_ang_d(&mut self, ang:f64,dist:f64) {
        self.ord.mut_ang(ang);
        self.ord.mut_dist(dist);
        self.update_xy();
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

impl GallOffLoc {
    pub fn new(ord:GallOrd, ang_offset:f64, dist_offset:f64, center_ref: Rc<OnceCell<(f64,f64)>>) -> Option<GallOffLoc> {
        let (rel_y,rel_x) = (FRAC_PI_2 - ord.ang()?).sin_cos();
        let dist = ord.dist()?;
        let (center_x,center_y) = center_ref.get()?;
        let pos = (dist*rel_x + center_x, dist*rel_y + center_y);
        let abs = OnceCell::new();
        abs.set(pos);
        Some(GallOffLoc {
            ord,
            ang_offset,
            dist_offset,
            center_ref,
            abs_svg:Rc::new(abs),
        })
    }
    fn update_xy(&mut self) {
        if let Some(dist) = self.dist() {
            let (rel_y,rel_x) = match self.ang() {
                Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
                None => (0.0,0.0)
            };
            let (center_x,center_y) = self.center_ords();
            self.abs_svg.set((dist*rel_x + center_x, dist*rel_y + center_y));
        } else {
            panic!()
        }
    }
    pub fn set_ang(&mut self, new_ang_ref: Rc<OnceCell<f64>>) {
        self.ord.set_ang(new_ang_ref)
    }
}

impl Location for GallLoc {
    fn mut_center(&mut self, movement:(f64,f64)) {
        let (center_x,center_y) = self.center_ords();
        self.center_ref.set((center_x + movement.0, center_y + movement.1));
    }
    fn set_center(&mut self, new_center: Rc<OnceCell<(f64,f64)>>) {
        self.center_ref = new_center;
        self.update_xy();
    }
    fn get_center(&self) -> Rc<OnceCell<(f64, f64)>> {
        self.center_ref.clone()
    }
    fn x(&self) -> f64 {
        self.abs_svg.get().unwrap().0
    }
    fn y(&self) -> f64 {
        self.abs_svg.get().unwrap().1
    }
    fn pos_ref(&self) -> Rc<OnceCell<(f64, f64)>> {
        self.abs_svg.clone()
    }
    fn center_ords(&self) -> (f64,f64) {
        *self.center_ref.get().unwrap()
    }
}
impl Location for GallOffLoc {
    fn mut_center(&mut self, movement:(f64,f64)) {
        let (center_x,center_y) = self.center_ref.get().unwrap();
        self.center_ref.set((center_x + movement.0, center_y + movement.1));
    }
    fn set_center(&mut self, new_center: Rc<OnceCell<(f64,f64)>>) {
        self.center_ref = new_center;
        self.update_xy();
    }
    fn get_center(&self) -> Rc<OnceCell<(f64,f64)>> {
        self.center_ref.clone()
    }
    fn x(&self) -> f64 {
        self.abs_svg.get().unwrap().0
    }
    fn y(&self) -> f64 {
        self.abs_svg.get().unwrap().1
    }
    fn pos_ref(&self) -> Rc<OnceCell<(f64,f64)>> {
        self.abs_svg.clone()
    }
    fn center_ords(&self) -> (f64,f64) {
        *self.center_ref.get().unwrap()
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
    fn dist(&self) -> Option<f64> {
        self.ord.dist()
    }
    fn get_ang(&self) -> Rc<OnceCell<f64>> {
        self.ord.get_ang()
    }
    fn get_dist(&self) -> Rc<OnceCell<f64>> {
        self.ord.get_dist()
    }
}

impl PolarOrdinate for GallOffLoc {
    fn mut_ang(&mut self, mut new_ang:f64) {
        if let Some(ang) = self.ord.ang() {
            while new_ang.is_sign_negative() {
                new_ang += TAU;
            }
            while new_ang > TAU {
                new_ang -= TAU;
            } 
            self.ang_offset = new_ang - ang;
        }
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error> {
        if new_dist.is_sign_negative() {
            return Err(Error::new(GallError::NegativeDistanceErr))
        }
        if let Some(dist) = self.ord.dist() {
            self.dist_offset = new_dist - dist;
            Ok(())
        } else {
            Err(Error::new(GallError::ValueNotSet))
        }
    }
    fn ang(&self) -> Option<f64> {
        if let Some(ang) = self.ord.ang() {
            Some(self.ang_offset + ang)
        } else {
            None
        }
    }
    fn dist(&self) -> Option<f64> {
        if let Some(dist) = self.ord.dist() {
            Some(dist + self.dist_offset)
        } else {None} 
    }
    fn get_ang(&self) -> Rc<OnceCell<f64>> {
        todo!()
        //self.ord.get_ang()
        //Err(Error::new(GallError::GallOffAng))
    }
    fn get_dist(&self) -> Rc<OnceCell<f64>> {
        todo!()
        //self.ord.get_dist()
        //Err(Error::new(GallError::GallOffDist))
    }
}