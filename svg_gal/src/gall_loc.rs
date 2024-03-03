use std::cell::OnceCell;
use std::f64::consts::{FRAC_PI_2, TAU};
use std::rc::Rc;

use crate::gall_ang::GallAng;
use crate::gall_ord::{GallOrd, PolarOrdinate};
use crate::gall_errors::{Error, GallError};

#[derive(PartialEq,Default,Clone)]
pub struct GallLoc {
    ord: GallOrd,
    center_ref: OnceCell<(f64,f64)>, // abs xy
    abs_svg: OnceCell<(f64,f64)>,
}

pub struct GallOffLoc {
    base_ord: GallOrd,
    offset_ang: GallAng,
    offset_dist: OnceCell<f64>,
    center_ref: OnceCell<(f64,f64)>, // abs xy
    abs_svg: OnceCell<(f64,f64)>,
}

pub trait Location:PolarOrdinate {
    fn mut_center(&mut self, movement:(f64,f64));
    fn set_center(&mut self, new_center:OnceCell<(f64,f64)>);
    fn get_center(&self) -> OnceCell<(f64,f64)>;
    fn center_ords(&self) -> (f64,f64);
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn pos_ref(&self) -> OnceCell<(f64,f64)>;
    fn svg_ord(&self) -> Result<(f64,f64), Error> {
        match self.pos_ref().get() {
            Some(xy) => Ok(*xy),
            None => Err(Error::new(GallError::ValueNotSet)),
        }
    }
}

fn compute_svg(distance:f64, angle:Option<f64>, center_ref:OnceCell<(f64,f64)>) -> (f64,f64) {
    let (rel_y,rel_x) = match angle {
        Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
        None => (0.0,0.0)
    };
    let (center_x,center_y) = center_ref.get().unwrap();
    (distance*rel_x + center_x, distance*rel_y + center_y)
}

impl GallLoc {
    pub fn new(angle:f64, distance: f64, center_ref:OnceCell<(f64,f64)>) -> Option<GallLoc> {
        let pos = self::compute_svg(distance, Some(angle), center_ref.clone());
        let abs = OnceCell::new();
        abs.set(pos);
        Some(GallLoc { 
            ord: GallOrd::new(angle, distance),
            center_ref,  
            abs_svg: abs,
        })
    }
    fn update_xy(&mut self) -> Result<(),Error> {
        if let Some(&distance) = self.dist() {
            let svg_ord = self::compute_svg(distance, self.ang().copied(), self.get_center());
            _ = self.abs_svg.take();
            _ = self.abs_svg.set(svg_ord);
            Ok(())
        } else {
            Err(Error::new(GallError::ValueNotSet))
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
    pub fn take_ang(&mut self) -> f64 {
        self.ord.take_ang()
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
    pub fn new(base_ord:GallOrd, ang_offset:f64, dist_offset:f64, center_ref: OnceCell<(f64,f64)>) -> Option<GallOffLoc> {
        if let Some(&distance) = base_ord.dist() {
            let angle = match base_ord.ang() {
                Some(&ang) => ang + ang_offset,
                None => return None,
            };
            let offset_dist = OnceCell::new();
            offset_dist.set(dist_offset);
            let pos = self::compute_svg(distance + dist_offset, Some(angle), center_ref.clone());
            let abs = OnceCell::new();
            abs.set(pos);
            Some(GallOffLoc {
                base_ord,
                offset_ang: GallAng::new(Some(ang_offset)),
                offset_dist,
                center_ref,
                abs_svg:abs,
            })
        } else {
            None
        }
    }
    fn update_xy(&mut self) -> Result<(),Error> {
        if let (Some(&b_dist), Some(& o_dist), Some(&b_ang), Some(&o_ang)) = (
                self.base_ord.dist(), self.dist(), self.base_ord.ang(), self.ang()) {
            let svg_ord = self::compute_svg(b_dist+o_dist, Some(b_ang + o_ang), self.get_center());
            _ = self.abs_svg.take();
            _ = self.abs_svg.set(svg_ord);
            Ok(())
        } else {
            Err(Error::new(GallError::ValueNotSet))
        }
    }
    // pub fn set_ang(&mut self, new_ang_ref: OnceCell<f64>) {
    //     self.ord.set_ang(new_ang_ref)
    // }
}

impl Location for GallLoc {
    fn mut_center(&mut self, movement:(f64,f64)) {
        let (center_x,center_y) = self.center_ords();
        self.center_ref.set((center_x + movement.0, center_y + movement.1));
    }
    fn set_center(&mut self, new_center: OnceCell<(f64,f64)>) {
        self.center_ref = new_center;
        self.update_xy();
    }
    fn get_center(&self) -> OnceCell<(f64, f64)> {
        self.center_ref.clone()
    }
    fn x(&self) -> f64 {
        self.abs_svg.get().unwrap().0
    }
    fn y(&self) -> f64 {
        self.abs_svg.get().unwrap().1
    }
    fn pos_ref(&self) -> OnceCell<(f64, f64)> {
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
    fn set_center(&mut self, new_center: OnceCell<(f64,f64)>) {
        self.center_ref = new_center;
        self.update_xy();
    }
    fn get_center(&self) -> OnceCell<(f64,f64)> {
        self.center_ref.clone()
    }
    fn x(&self) -> f64 {
        self.abs_svg.get().unwrap().0
    }
    fn y(&self) -> f64 {
        self.abs_svg.get().unwrap().1
    }
    fn pos_ref(&self) -> OnceCell<(f64,f64)> {
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
        self.ord.mut_dist(new_dist)
            .and_then(|_| self.update_xy())
    }
    fn ang(&self) -> Option<&f64> {
        self.ord.ang()
    }
    fn dist(&self) -> Option<&f64> {
        self.ord.dist()
    }
    fn get_ang(&self) -> OnceCell<f64> {
        self.ord.get_ang()
    }
    fn get_dist(&self) -> OnceCell<f64> {
        self.ord.get_dist()
    }
}

impl PolarOrdinate for GallOffLoc {
    fn mut_ang(&mut self, new_ang:f64) {
        self.offset_ang.mut_ang(Some(new_ang));
        self.update_xy();
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error> {
        _ = self.offset_dist.take();
        _ = self.offset_dist.set(new_dist);
        self.update_xy()
    }
    fn ang(&self) -> Option<&f64> {
        self.offset_ang.ang()
    }
    fn dist(&self) -> Option<&f64> {
        self.offset_dist.get()
    }
    fn get_ang(&self) -> OnceCell<f64> {
        self.offset_ang.get_ang()
    }
    fn get_dist(&self) -> OnceCell<f64> {
        self.offset_dist.clone()
    }    
}