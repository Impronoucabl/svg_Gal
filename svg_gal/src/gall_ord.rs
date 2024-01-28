
use std::f64::consts::{PI, FRAC_PI_2, FRAC_PI_8, TAU};
use std::error::Error;

trait BoundedValue<BoundType,ValueType> {
    fn val_check(lower_bound:BoundType, upper_bound:BoundType, val: ValueType) -> Result<ValueType, Box<dyn Error>>;
    fn mut_val(&mut self, val: ValueType) -> Result<(),Box<dyn Error>>;
}

//GallAng is a simple wrapper around Option<f64> to enforce
// the allowed range of 0 < angle < TAU
#[derive(PartialEq,Default,Clone, Copy)]
struct GallAng {
    angle: Option<f64>,
}
#[derive(PartialEq,Default,Clone, Copy)]
struct PositiveDist {
    distance: f64,
}

#[derive(PartialEq,Default,Clone, Copy)]
struct GallOrd {
    ang: GallAng,
    dist: PositiveDist,
} 

#[derive(PartialEq,Default,Clone, Copy)]
pub struct GallLoc {
    pub ord: GallOrd,
    center: (f64,f64), // abs xy
    rel_svg_x:f64,
    rel_svg_y:f64,
}

impl BoundedValue<f64, Option<f64>> for GallAng {
    fn val_check(lower_bound: f64, upper_bound: f64, angle:Option<f64>) -> Result<Option<f64>, Box<dyn Error>> {
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
    fn mut_val(&mut self, new_angle: Option<f64>) -> Result<(), Box<dyn Error>>{
        self.angle = GallAng::val_check(0.0, TAU, new_angle)?;
        Ok(())
    }
}
//Upper bound not used
impl BoundedValue<f64, f64> for PositiveDist {
    fn val_check(lower_bound: f64, upper_bound: f64, dist:f64) -> Result<f64, Box<dyn Error>> {
        if dist >= lower_bound {
            Ok(dist)
        } else {
            Ok(0.0)
        }
    }
    fn mut_val(&mut self, new_dist: f64) -> Result<(), Box<dyn Error>>{
        self.distance = PositiveDist::val_check(0.0, 0.0, new_dist)?;
        Ok(())
    }
}

impl BoundedValue<f64, GallAng> for GallOrd {
    fn val_check(zero_dist:f64, dist:f64, ord:GallAng) -> Result<GallAng, Box<dyn Error>> {
        let new_ang = match dist {
            lower_bound => GallAng { angle: None },
            Other => ord,
        };
        Ok(new_ang)
    }
    fn mut_val(&mut self, new_angle: GallAng) -> Result<(), Box<dyn Error>>{
        self.ang = GallOrd::val_check(0.0, self.dist.distance, new_angle)?;
        Ok(())
    }
}

impl GallAng {
    fn new(angle: Option<f64>) -> Result<GallAng, Box<dyn Error>> {
        let new = GallAng {
            angle: GallAng::val_check(0.0, TAU, angle)?
        };
        Ok(new)
    }
    fn rotate(&mut self, angle:f64) {
        GallAng::mut_val(self, Some(angle));
    }    
}

impl PositiveDist {
    fn new(dist: f64) -> Result<PositiveDist, Box<dyn Error>> {
        //TODO if dist < 0.0 {return Err();}
        let new = PositiveDist {
            distance: PositiveDist::val_check(0.0, 0.0, dist)?
        };
        Ok(new)
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
    fn mut_ang(&mut self, angle:Option<f64>) {
        self.ang.mut_val(angle);
    }
    fn mut_dist(&mut self, new_dist:f64) {
        self.dist.mut_val(new_dist);
    }
}

impl GallLoc {
    pub fn new(angle:f64, distance: f64, svg_center:(f64,f64)) -> GallLoc {
        let ord = GallOrd::new(angle, distance);
        let (rel_y,rel_x) = (FRAC_PI_2 - angle).sin_cos();
        GallLoc { 
            ord: ord,
            center: svg_center,  
            rel_svg_x: distance*rel_x,
            rel_svg_y: distance*rel_y,
        }
    }
    fn update_xy(&mut self) {
        let dist = self.get_dist();
        let (rel_y,rel_x) = match self.get_ang() {
            Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
            None => (0.0,0.0)
        };
        self.rel_svg_x = dist*rel_x;
        self.rel_svg_y = dist*rel_y;
    }
    fn mut_ang(&mut self, angle:Option<f64>) {
        self.ord.mut_ang(angle);
        self.update_xy();
    }
    pub fn mut_dist(&mut self, new_dist:f64) {
        self.ord.mut_dist(new_dist);
        self.update_xy();
    }
    pub fn mut_center(&mut self, new_center: (f64,f64)) {
        self.center = new_center;
        self.update_xy();
    }
    pub fn get_ang(&self) -> Option<f64> {
        self.ord.ang.angle
    }
    pub fn get_dist(&self) -> f64 {
        self.ord.dist.distance
    }
    pub fn get_center(&self) -> (f64,f64) {
        self.center
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
    pub fn rotate_ccw(&mut self, angle: f64) -> Option<()> {
        self.mut_ang(Some(self.get_ang()? + angle));
        Some(())
    }
    pub fn rotate_cw(&mut self, angle: f64) -> Option<()> {
        self.rotate_ccw(-angle)
    }
    pub fn step_ccw(&mut self) -> Option<()> {
        let new_angle = self.get_ang()? + FRAC_PI_8/8.0;
        if new_angle < TAU {
            self.mut_ang(Some(new_angle));
            Some(())
        } else {
            None
        }
    }
    pub fn step_cw(&mut self) -> Option<()> {
        let new_angle = self.get_ang()? - FRAC_PI_8/8.0;
        if new_angle >= 0.0 {
            self.mut_ang(Some(new_angle));
            Some(())
        } else {
            None
        }
    }
    pub fn flip_ang(&mut self) {
        self.rotate_ccw(PI);
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