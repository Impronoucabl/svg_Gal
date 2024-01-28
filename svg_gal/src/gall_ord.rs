
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
struct GallOrd {
    ang: GallAng,
    dist: f64,
} 

#[derive(PartialEq,Default,Clone, Copy)]
pub struct GallLoc {
    //ang is undefined if dist == 0.0
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
        self.angle = Self::val_check(0.0, TAU, new_angle)?;
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
        self.ang = Self::val_check(0.0, self.dist, new_angle)?;
        Ok(())
    }
}

impl GallAng {
    fn new(angle: Option<f64>) -> GallAng {
        GallAng {angle: Self::val_check(0.0, TAU, angle)?}
    }
    fn rotate(&mut self, angle:f64) {
        GallAng::mut_val(self, Some(angle));
    }    
}

//Technically should use Option<f64> for the angle, but lazy.
impl GallOrd {
    fn new(angle:f64, distance:f64) -> GallOrd {
        GallOrd { ang: GallAng { angle: Some(angle) }, dist: distance }
    } 

    fn rotate(&self, angle:f64) {
        if self.dist == 0.0 {
            return;
        }
        self.ang.rotate(angle);
    }

    fn mut_dist(&mut self, new_dist:f64) {
        self.dist = new_dist;
    }

}

impl GallLoc {
    pub fn new(angle:f64, distance: f64, svg_center:(f64,f64)) -> Galloc {
        let g_ang = GallAng::new(angle);
        let (rel_y,rel_x) = match angle {
            Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
            None => (0.0,0.0)
        };
        GallLoc { 
            ang: g_ang,
            dist,
            center,  
            rel_svg_x: dist*rel_x,
            rel_svg_y: dist*rel_y,
        }
    }
}

// impl GallLoc {
//     pub fn new(angle: Option<f64>,dist: f64,center: (f64, f64)) -> GallLoc {
//         let g_ang = GallAng::new(angle);
//         let (rel_y,rel_x) = match angle {
//             Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
//             None => (0.0,0.0)
//         };
//         GallLoc { 
//             ang: g_ang,
//             dist,
//             center,  
//             rel_svg_x: dist*rel_x,
//             rel_svg_y: dist*rel_y,
//         }
//     }
//     pub fn svg_x(&self) -> f64 {
//         self.rel_svg_x + self.center.0
//     }
//     pub fn svg_y(&self) -> f64 {
//         self.rel_svg_y + self.center.1
//     }
//     //SVG is stupid, and positive angles are clockwise
//     pub fn svg_ord(&self) -> (f64,f64) {
//         match self.ang.angle {
//             //can I use float.sin_cos()?
//             Some(_) => (self.svg_x(),self.svg_y()),
//             None => self.center
//         }
//     }
//     pub fn get_center(&self) -> (f64,f64) {
//         self.center
//     }
//     pub fn flip_ang(&mut self) {
//         let mut angle = match self.ang {
//             Some(ang) => ang + PI,
//             None => return,
//         };
//         if angle > TAU {
//             angle -= TAU;
//         }
//         self.mut_ang(angle);
//     }

//     pub fn mut_ang(&mut self, mut new_ang:f64) {
//         while new_ang > TAU {
//             new_ang -= TAU
//         };
//         while new_ang < 0.0 {
//             new_ang += TAU
//         };
//         self.ang = match self.ang {
//             Some(_) => Some(new_ang),
//             None => None,
//         };
//         self.update_xy();
//     }
//     pub fn c_clockwise(&mut self, radians:f64, force:bool) -> Option<()> {
//         let mut new_angle = (self.ang? + radians).max(0.0);
//         if force {
//             while new_angle > TAU {
//                 new_angle -= TAU;
//             }
//             while new_angle < 0.0 {
//                 new_angle += TAU;
//             }
//             self.ang = Some(new_angle);
//         } else {
//             static READABILITY_ANGLE:f64 = std::f64::consts::TAU - 0.35;
//             if new_angle == READABILITY_ANGLE {
//                 return None
//             }
//             self.ang = Some(new_angle.min(READABILITY_ANGLE));
//         }
//         self.update_xy();
//         Some(())
//     }
//     pub fn cw_step(&mut self) -> Option<()> {
//         self.c_clockwise(-self.ang?.min(FRAC_PI_8/8.0), false)
//     }
//     pub fn ccw_step(&mut self) -> Option<()>{
//         self.c_clockwise(FRAC_PI_8/8.0, false)
//     }
//     pub fn set_dist(&mut self, new_dist:f64) {
//         self.dist = new_dist;
//         if new_dist == 0.0 {
//             self.ang = None;
//         }
//         self.update_xy();
//     }
//     pub fn update_center(&mut self, new_center: (f64,f64)) {
//         self.center = new_center;
//     }
//     fn update_xy(&mut self) {
//         let (rel_y,rel_x) = match self.ang {
//             Some(angle) => (FRAC_PI_2 - angle).sin_cos(),
//             None => (0.0,0.0)
//         };
//         self.rel_svg_x = self.dist*rel_x;
//         self.rel_svg_y = self.dist*rel_y;
//     }
// }