pub struct GallCircle<'loc> { //Syllable equivalent
    character: char,
    repeat: bool,
    vowel:Option<VowCircle>,
    loc: GallOrd<'loc>,
    radius: f64,
    decorators:Vec<Decor<'loc>>
}

struct VowCircle { //for attached vowels only
    character: char,
    repeat: bool,
    radius: f64,
}

struct Decor<'loc> {
    loc: GallOrd<'loc>,
    dot: bool,
}

use std::f64::consts::PI;

pub struct GallOrd <'parent> {
    pub ang: Option<f64>,
    pub dist: f64,
    pub center: (f64,f64), // abs xy
    pub parent: Option<&'parent GallOrd<'parent>>,
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
            Some(rad) => self.dist*(rad - PI/2.0).sin() + self.center.1,
            None => self.center.1
        }
    }
    fn set_ang_d(&mut self, co_ord: (f64,f64)) {
        self.ang = Some(co_ord.0);
        self.dist = co_ord.1;
    }
    fn clockwise(&mut self, radians:f64) {
        let ang= match self.ang {
            Some(rad) => rad,
            None => 0.0,
        };
        self.ang = Some(ang - radians);
    }
    fn c_clockwise(&mut self, radians:f64) {
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