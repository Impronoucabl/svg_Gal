use std::f64::consts::{FRAC_PI_2, PI};
use std::error::Error;

use crate::gall_errors;
use crate::gall_ord::{BoundedValue, GallLoc, PositiveDist};
use crate::gall_struct::Circle;

trait StemTrait {
    fn get_parent_radius(&self) -> f64;
    fn mut_stored_parent_radius(&mut self, new_radius: f64) -> Result<(),Box<dyn Error>>;
}

pub enum Stem {JStem,BStem,TStem,ZStem}

struct JStem {loc: GallLoc,radius: PositiveDist,thickness: PositiveDist,parent_radius: PositiveDist}
struct BStem {loc: GallLoc,radius: PositiveDist,thickness: PositiveDist,parent_radius: PositiveDist}
struct TStem {loc: GallLoc,radius: PositiveDist,thickness: PositiveDist,parent_radius: PositiveDist}
struct ZStem {loc: GallLoc,radius: PositiveDist,thickness: PositiveDist,parent_radius: PositiveDist}

impl StemTrait for JStem {
    fn get_parent_radius(&self) -> f64 {
        self.parent_radius.dist()
    }
    fn mut_stored_parent_radius(&mut self, new_radius:f64) -> Result<(),Box<dyn Error>> {
        self.parent_radius.mut_val(new_radius)
    }
    
}


impl BoundedValue<f64,PositiveDist> for JStem {
    fn val_check(lower_bound:f64, upper_bound:f64, val: PositiveDist) -> Result<PositiveDist, Box<dyn Error>> {
        let dist = val.dist();
        if dist >= lower_bound {
            if dist < upper_bound {
                Ok(val)
            } else {
                Err(Box::new(gall_errors::StemDistTooLong))
            }
        } else {
            Err(Box::new(gall_errors::StemDistTooShort))
        }
    }
    fn mut_val(&mut self, new_radius: PositiveDist) -> Result<(),Box<dyn Error>> {
        let radius = JStem::val_check(0.0, self.get_parent_radius()*0.9, new_radius)?.dist();
        self.radius.mut_val(radius)
    }
}

impl Circle for JStem {
    fn get_outer_radius(&self) -> PositiveDist {
        self.radius + self.thickness
    }
    fn get_inner_radius(&self) -> PositiveDist {
        self.radius - self.thickness
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(), Box<dyn Error>> {
        self.radius.mut_val(new_radius)
    }
}