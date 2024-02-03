
use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_8, TAU};
use std::ops::{Add,Sub};
use std::rc::Rc;

use crate::gall_errors::{Error, GallError};

pub trait BoundedValue<BoundType,ValueType> {
    fn val_check(lower_bound:BoundType, upper_bound:BoundType, val: ValueType) -> Result<ValueType, Error>;
    fn mut_val(&mut self, val: ValueType) -> Result<(),Error>;
}

pub trait LocMover {
    fn mut_ang(&mut self, angle:GallAng) -> Result<(), Error>;
    fn mut_dist(&mut self, new_dist:f64) -> Result<(), Error>;
    fn mut_center(&mut self, movement:(f64,f64));
    fn set_center(&mut self, new_center:CenterOrd);
    fn get_ang(&self) -> GallAng;
    fn get_dist(&self) -> f64;
    fn get_center(&self) -> Rc<(f64,f64)>;
}

pub enum CenterOrd {
    New((f64,f64)),
    Exisiting(Rc<(f64,f64)>)
}
//GallAng is a simple wrapper around Option<f64> to enforce
// the allowed range of 0 < angle < TAU
#[derive(PartialEq,Default,Clone, Copy)]
pub struct GallAng {
    angle: Option<f64>,
}
#[derive(PartialEq,Default,Clone,Copy)]
pub struct PositiveDist {
    distance: f64,
}

#[derive(PartialEq,Default,Clone,Copy)]
struct GallOrd {
    ang: GallAng,
    dist: PositiveDist,
} 

#[derive(PartialEq,Default,Clone)]
pub struct GallLoc {
    pub ord: GallOrd,
    center: Rc<(f64,f64)>, // abs xy
    rel_svg_x:f64,
    rel_svg_y:f64,
}

impl CenterOrd {
    pub fn get_rc(self) -> Rc<(f64,f64)> {
        match self {
            CenterOrd::New(ord) => Rc::new(ord),
            CenterOrd::Exisiting(ord) => ord.clone(),
        }
    }
}

impl Add for GallAng {
    type Output = Self;
    fn add(self, other: GallAng) -> GallAng {
        GallAng::new(
            match (self.angle, other.angle) {
                (Some(ang1),Some(ang2)) => Some(ang1 + ang2),
                _ => None
            }
        )
    }
}

impl Sub for GallAng {
    type Output = Self;
    fn sub(self, other: GallAng) -> GallAng {
        GallAng::new(
            match (self.angle, other.angle) {
                (Some(ang1),Some(ang2)) => Some(ang1 - ang2),
                _ => None
            }
        )
    }
}

impl Add for PositiveDist {
    type Output = Self;
    fn add(self, other: PositiveDist) -> PositiveDist {
        PositiveDist {
            distance: self.distance + other.distance
        }
    }
}

impl Sub for PositiveDist {
    type Output = Self;
    fn sub(self, rhs: Self) -> PositiveDist {
        PositiveDist::new(
            self.distance - rhs.distance
        ).unwrap()
    }
}

impl BoundedValue<f64, Option<f64>> for GallAng {
    fn val_check(lower_bound: f64, upper_bound: f64, angle:Option<f64>) -> Result<Option<f64>, Error> {
        let val = match angle {
            Some(mut ang) => {
                while ang >= upper_bound {
                    ang -= TAU
                };
                while ang < lower_bound {
                    ang += TAU
                };
                Some(ang)
            }
            None => None
        };
        Ok(val)
    }
    fn mut_val(&mut self, new_angle: Option<f64>) -> Result<(), Error>{
        self.angle = GallAng::val_check(0.0, TAU, new_angle)?;
        Ok(())
    }
}
//Upper bound not used
impl BoundedValue<f64, f64> for PositiveDist {
    fn val_check(lower_bound: f64, upper_bound: f64, dist:f64) -> Result<f64, Error> {
        if dist >= lower_bound {
            Ok(dist)
        } else {
            Ok(0.0)
        }
    }
    fn mut_val(&mut self, new_dist: f64) -> Result<(), Error>{
        self.distance = PositiveDist::val_check(0.0, 0.0, new_dist)?;
        Ok(())
    }
}
//Checks if distance is 0
impl BoundedValue<f64, GallAng> for GallOrd {
    fn val_check(zero_dist:f64, dist:f64, ord:GallAng) -> Result<GallAng, Error> {
        let new_ang = match dist {
            lower_bound => GallAng { angle: None },
            Other => ord,
        };
        Ok(new_ang)
    }
    fn mut_val(&mut self, new_angle: GallAng) -> Result<(), Error>{
        self.ang = GallOrd::val_check(0.0, self.dist.distance, new_angle)?;
        Ok(())
    }
}

impl GallAng {
    fn new(angle: Option<f64>) -> GallAng {
        GallAng {
            angle: GallAng::val_check(0.0, TAU, angle).unwrap()
        }
    }
    fn rotate(&mut self, angle:f64) {
        GallAng::mut_val(self, Some(angle));
    }    
}

impl PositiveDist {
    pub fn new(dist: f64) -> Result<PositiveDist, Error> {
        if dist < 0.0 {return Err(Error::new(GallError::NegativeDistanceErr));}
        let new = PositiveDist {
            distance: PositiveDist::val_check(0.0, 0.0, dist)?
        };
        Ok(new)
    }
    pub fn dist(&self) -> f64 {
        self.distance
    }   
}

//Technically should use Option<f64> for the angle, but lazy.
impl GallOrd {
    fn new(angle:f64, distance:f64) -> GallOrd {
        GallOrd { 
            ang: GallAng { 
                angle: Some(angle) 
            }, 
            dist: PositiveDist {
                distance
            }
        }
    } 
    fn mut_ang(&mut self, angle:GallAng) -> Result<(), Error> {
        self.mut_val(angle)
    }
    fn mut_dist(&mut self, new_dist:f64) -> Result<(), Error> {
        self.dist.mut_val(new_dist)
    }
}

impl LocMover for GallLoc {
    fn mut_ang(&mut self, angle:GallAng) -> Result<(), Error> {
        self.ord.mut_ang(angle)?;
        self.update_xy();
        Ok(())
    }
    fn mut_dist(&mut self, new_dist:f64)-> Result<(), Error> {
        self.ord.mut_dist(new_dist)?;
        self.update_xy();
        Ok(())
    }
    fn mut_center(&mut self, movement:(f64,f64)) {
        self.center.0 += movement.0; 
        self.center.1 += movement.1;
    }
    fn set_center(&mut self, new_center: CenterOrd) {
        self.center = new_center.get_rc();
        self.update_xy();
    }
    fn get_ang(&self) -> GallAng {
        self.ord.ang
    }
    fn get_dist(&self) -> f64 {
        self.ord.dist.distance
    }
    fn get_center(&self) -> Rc<(f64,f64)> {
        self.center.clone()
    }
}

impl GallLoc {
    pub fn new(angle:f64, distance: f64, svg_center:CenterOrd) -> GallLoc {
        let ord = GallOrd::new(angle, distance);
        let (rel_y,rel_x) = (FRAC_PI_2 - angle).sin_cos();
        GallLoc { 
            ord: ord,
            center: svg_center.get_rc(),  
            rel_svg_x: distance*rel_x,
            rel_svg_y: distance*rel_y,
        }
    }
    fn update_xy(&mut self) {
        let dist = self.get_dist();
        let (rel_y,rel_x) = match self.get_ang().angle {
            Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
            None => (0.0,0.0)
        };
        self.rel_svg_x = dist*rel_x;
        self.rel_svg_y = dist*rel_y;
    }
    pub fn svg_x(&self) -> f64 {
        self.rel_svg_x + self.center.0
    }
    pub fn svg_y(&self) -> f64 {
        self.rel_svg_y + self.center.1
    }
    pub fn svg_ord(&self) -> (f64,f64) {
        (self.svg_x(),self.svg_y())
    }
    pub fn rotate_ccw(&mut self, angle: GallAng) -> Option<()> {
        self.mut_ang(self.get_ang() + angle);
        Some(())
    }
    pub fn rotate_cw(&mut self, angle: GallAng) -> Option<()> {
        self.rotate_ccw(GallAng{angle:Some(0.0)} - angle)
    }
    pub fn step_ccw(&mut self) -> Option<()> {
        let new_angle = self.get_ang().angle? + FRAC_PI_8/8.0;
        if new_angle < TAU {
            self.mut_ang(GallAng{angle:Some(new_angle)});
            Some(())
        } else {
            None
        }
    }
    pub fn step_cw(&mut self) -> Option<()> {
        let new_angle = self.get_ang().angle? - FRAC_PI_8/8.0;
        if new_angle >= 0.0 {
            self.mut_ang(GallAng{angle:Some(new_angle)});
            Some(())
        } else {
            None
        }
    }
    pub fn flip_ang(&mut self) {
        self.rotate_ccw(GallAng{angle:Some(PI)});
    }
    pub fn lengthen(&mut self, extra_dist:f64) {
        self.shorten(-extra_dist)
    }
    pub fn shorten(&mut self, extra_dist:f64) {
        let new_dist = self.get_dist() - extra_dist;
        if new_dist < 0.0 {
            self.mut_dist(-new_dist);
            self.flip_ang();
        } else {
            self.mut_dist(new_dist);
        }
    } 
}