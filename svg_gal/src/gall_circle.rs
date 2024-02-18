use std::rc::Rc;
use std::cell::Cell;
use crate::gall_errors::Error;

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

pub trait ParentCircle: HollowCircle {
    fn get_mut_rad_fn_ptr(&self) -> fn(f64)->Result<(),Error>;
    fn get_mut_thick_fn_ptr(&self) -> fn(f64)->Result<(),Error>;
}
pub trait ChildCircle{
    fn parent_radius(&self) -> f64;
    fn parent_thick(&self) -> f64;
    fn get_parent_radius(&self) -> Rc<Cell<f64>>;
    fn get_parent_thick(&self) -> Rc<Cell<f64>>;
    fn mut_parent_radius(&mut self, new_radius:f64) -> Result<(),Error>;
    fn mut_parent_thick(&mut self, new_thickness:f64) -> Result<(),Error>;
    fn parent_inner(&self) -> f64 {
        self.parent_radius() - self.parent_thick()
    }
    fn parent_outer(&self) -> f64 {
        self.parent_radius() + self.parent_thick()
    }
}