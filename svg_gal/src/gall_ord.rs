

use std::f64::consts::FRAC_PI_2;
use std::f64::consts::FRAC_PI_8;

#[derive(PartialEq,Default,Clone, Copy)]
pub struct GallOrd {
    //ang is undefined if dist == 0.0
    pub ang: Option<f64>,
    pub dist: f64,
    pub center: (f64,f64), // abs xy
    rel_svg_x:f64,
    rel_svg_y:f64,
}

impl GallOrd {
    pub fn new(angle: Option<f64>,dist: f64,center: (f64, f64)) -> GallOrd {
        let (rel_y,rel_x) = match angle {
            Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
            None => (0.0,0.0)
        };
        GallOrd { 
            ang: angle,
            dist,
            center,  
            rel_svg_x: dist*rel_x,
            rel_svg_y: dist*rel_y,
        }
    }
    pub fn svg_x(&self) -> f64 {
        self.rel_svg_x + self.center.0
    }
    pub fn svg_y(&self) -> f64 {
        self.rel_svg_y + self.center.1
    }
    //SVG is stupid, and positive angles are clockwise
    pub fn svg_ord(&self) -> (f64,f64) {
        match self.ang {
            //can I use float.sin_cos()?
            Some(_) => (self.svg_x(),self.svg_y()),
            None => self.center
        }
    }
    pub fn set_ang(&mut self, new_ang:f64) {
        self.ang = match self.ang {
            Some(_) => Some(new_ang),
            None => None,
        };
        self.update_xy();
    }
    pub fn c_clockwise(&mut self, radians:f64, force:bool) -> Option<()> {
        let new_angle = (self.ang? + radians).max(0.0);
        if force {
            self.ang = Some(new_angle);
        } else {
            static READABILITY_ANGLE:f64 = std::f64::consts::TAU - 0.35;
            if new_angle == READABILITY_ANGLE {
                return None
            }
            self.ang = Some(new_angle.min(READABILITY_ANGLE));
        }
        self.update_xy();
        Some(())
    }
    pub fn cw_step(&mut self) -> Option<()> {
        self.c_clockwise(-self.ang?.min(FRAC_PI_8/8.0), false)
    }
    pub fn ccw_step(&mut self) -> Option<()>{
        self.c_clockwise(FRAC_PI_8/8.0, false)
    }
    pub fn set_dist(&mut self, new_dist:f64) {
        self.dist = new_dist;
        if new_dist == 0.0 {
            self.ang = None;
        }
        self.update_xy();
    }
    fn update_xy(&mut self) {
        let (rel_y,rel_x) = match self.ang {
            Some(angle) => (FRAC_PI_2 - angle).sin_cos(),
            None => (0.0,0.0)
        };
        self.rel_svg_x = self.dist*rel_x;
        self.rel_svg_y = self.dist*rel_y;
    }
}