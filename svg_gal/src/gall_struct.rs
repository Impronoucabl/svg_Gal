pub struct GallCircle<'loc> { //Syllable equivalent
    pub character: char,
    pub repeat: bool,
    pub vowel:Option<VowCircle>,
    pub loc: GallOrd<'loc>,
    pub radius: f64,
    pub decorators:Vec<Decor<'loc>>
}

pub struct VowCircle { //for attached vowels only
    pub character: char,
    pub repeat: bool,
    pub radius: f64,
}

pub struct Decor<'loc> {
    pub loc: GallOrd<'loc>,
    pub dot: bool,
}

use std::f64::consts::PI;

pub struct GallOrd <'parent> {
    pub ang: Option<f64>,
    pub dist: f64,
    pub center: (f64,f64), // abs xy
    pub parent: Option<&'parent GallOrd<'parent>>,
}

impl GallCircle<'_> {
    pub fn theta(&self) -> f64 {
        1.1
    }
    pub fn thi(&self) -> f64 {
        0.6
    }
}

impl GallOrd<'_> {
    fn svg_x(&self) -> f64 {
        match self.ang {
            Some(rad) => self.dist*(rad - PI/2.0).cos() + self.center.0,
            None => self.center.0
        }
    }
    fn svg_y(&self) -> f64 {
        match self.ang {
            Some(rad) => self.dist*(rad - PI/2.0).sin() - self.center.1,
            None => self.center.1
        }
    }
    pub fn svg_ord(&self) -> (f64,f64) {
        match self.ang {
            Some(rad) => (
                self.dist*(rad - PI/2.0).cos() + self.center.0,
                self.dist*(rad - PI/2.0).sin() - self.center.1
            ),
            None => self.center
        }
    }
    pub fn set_ang_d(&mut self, co_ord: (f64,f64)) {
        self.ang = Some(co_ord.0);
        self.dist = co_ord.1;
    }
    pub fn clockwise(&mut self, radians:f64) {
        let ang= match self.ang {
            Some(rad) => rad,
            None => 0.0,
        };
        self.ang = Some(ang - radians);
    }
    pub fn c_clockwise(&mut self, radians:f64) {
        let ang= match self.ang {
            Some(rad) => rad,
            None => 0.0,
        };
        self.ang = Some(ang + radians);
    }
    fn set_dist(&mut self, new_dist:f64) {
        self.dist = new_dist
    }
}