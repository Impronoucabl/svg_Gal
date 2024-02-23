use std::f64::consts::FRAC_PI_2;
use std::rc::Rc;
use std::cell::Cell;
use crate::gall_ang::GallAng;
use crate::gall_errors::Error;
use crate::gall_loc::{GallLoc, Location};
use crate::gall_ord::PolarOrdinate;

pub struct Dot {
    angle: GallAng,
    dist_offset: f64,
    distance: Rc<Cell<f64>>,
    radius: Rc<Cell<f64>>,
    center_ref: Rc<Cell<(f64,f64)>>, // abs xy
    abs_svg: Rc<Cell<(f64,f64)>>,
} 

pub trait Circle {
    fn radius(&self) -> f64;
    fn mut_radius(&mut self, new_radius:f64) -> Result<(),Error>;
    fn get_radius(&self) -> Rc<Cell<f64>>;
}

pub trait HollowCircle: Circle {
    fn thick(&self) -> f64;
    fn get_thickness(&self) -> Rc<Cell<f64>>;
    fn mut_thickness(&mut self, new_thick: f64) -> Result<(),Error>;
    fn outer_radius(&self) -> f64 {
        self.radius() + self.thick()
    }
    fn inner_radius(&self) -> f64 {
        self.radius() - self.thick()
    }
}

pub trait ChildCircle{
    fn parent_radius(&self) -> f64;
    fn parent_thick(&self) -> f64;
    fn get_parent_radius(&self) -> Rc<Cell<f64>>;
    fn get_parent_thick(&self) -> Rc<Cell<f64>>;
    //fn mut_parent_radius(&mut self, new_radius:f64) -> Result<(),Error>;
    //fn mut_parent_thick(&mut self, new_thickness:f64) -> Result<(),Error>;
    fn parent_inner(&self) -> f64 {
        self.parent_radius() - self.parent_thick()
    }
    fn parent_outer(&self) -> f64 {
        self.parent_radius() + self.parent_thick()
    }
}
impl Dot {
    pub fn new(radius:f64, angle:f64, distance:Rc<Cell<f64>>,center_ref: Rc<Cell<(f64,f64)>>) -> Dot{
        let (rel_y,rel_x) = (FRAC_PI_2 - angle).sin_cos();
        let dist = distance.get();
        let (center_x,center_y) = center_ref.get();
        let pos = (dist*rel_x + center_x, dist*rel_y + center_y);
        Dot {
            angle: GallAng::new(Some(angle)),
            dist_offset: 0.0,
            distance,
            radius: Rc::new(Cell::new(radius)),
            center_ref,
            abs_svg:Rc::new(Cell::new(pos)),
        }
    }
    pub fn set_dist(&mut self, dist_ref:Rc<Cell<f64>>) {
        self.distance = dist_ref;
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
}
impl Circle for Dot {
    fn radius(&self) -> f64 {
        self.radius.get()
    }
    fn mut_radius(&mut self, new_radius:f64) -> Result<(),Error> {
        self.radius.set(new_radius);
        Ok(())
    }
    fn get_radius(&self) -> Rc<Cell<f64>> {
        self.radius.clone()
    }
}
impl PolarOrdinate for Dot {
    fn mut_ang(&mut self, new_ang:f64) {
        self.angle.mut_ang(Some(new_ang))
    }
    fn mut_dist(&mut self, new_dist: f64) -> Result<(), Error> {
        let diff = new_dist - self.dist();
        self.dist_offset += diff;
        Ok(())
    }
    fn ang(&self) -> Option<f64> {
        self.angle.ang()
    }
    fn dist(&self) -> f64 {
        self.dist_offset + self.distance.get()
    }
}
impl Location for Dot {
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