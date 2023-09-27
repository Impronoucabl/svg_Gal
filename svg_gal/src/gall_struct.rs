use std::f64::consts::FRAC_PI_2;
use std::f64::consts::FRAC_PI_8;
//use std::result::Result::{Err, Ok};

#[derive(PartialEq,Default)]
pub enum  LetterType {
    Digit,
    StaticVowel,
    BStem,
    JStem,
    TStem,
    ZStem,
    AVowel,
    OVowel,
    #[default]
    Punctuation, //more for error case than anything
}
#[derive(PartialEq, Default)]
pub struct GallWord<'loc> {
    pub syllables: Vec<GallCircle<'loc>>,
    pub letter_count: usize,
    pub loc: GallOrd<'loc>,
    pub radius: f64,
    pub decorators:Vec<Decor<'loc>>
}

#[derive(PartialEq, Default)]
pub struct GallCircle<'loc> { //Syllable equivalent
    pub character: char,
    pub stem:LetterType,
    pub repeat: bool,
    pub vowel:Option<VowCircle>,
    pub loc: GallOrd<'loc>,
    pub radius: f64,
    pub decorators:Vec<Decor<'loc>>
}
#[derive(PartialEq,Default)]
pub struct VowCircle { //for attached vowels only
    pub character: char,
    pub repeat: bool,
    pub radius: f64,
}
#[derive(PartialEq,Default)]
pub struct Decor<'loc> {
    pub loc: GallOrd<'loc>,
    pub dot: bool,
}

#[derive(PartialEq,Default)]
pub struct GallOrd <'parent> {
    //ang is undefined if dist == 0.0
    pub ang: Option<f64>,
    pub dist: f64,
    pub center: (f64,f64), // abs xy
    pub parent: Option<&'parent GallOrd<'parent>>,
}

impl GallWord<'_> {
    pub fn thi(&self, letter: &GallCircle) -> f64 {
        //below is python
        //math.acos((Wrd.inner_rad**2 + dist**2 - self.outer_rad**2)/(2*dist*Wrd.inner_rad))
        let thi = ((self.radius.powf(2.0) + letter.loc.dist.powf(2.0) - letter.radius.powf(2.0))/(2.0*letter.loc.dist*self.radius)).acos();
        if thi == std::f64::NAN { //Circles aren't touching
            0.0 //could do math error?
        } else {
            thi
        }
    }
    fn update_kids(&mut self) {
        for circle in &mut self.syllables {
            circle.loc.center = self.loc.svg_ord();
        }
    }
}

impl GallCircle<'_> {
    //below is python
    //self.theta  = math.acos((Wrd.inner_rad**2 - dist**2 - self.outer_rad**2)/(2*dist*self.outer_rad))
    pub fn theta(&self, word:&GallWord) -> f64 {
        let theta = ((word.radius.powf(2.0) - self.loc.dist.powf(2.0) - self.radius.powf(2.0))/(2.0*self.loc.dist*self.radius)).acos();
        if theta == std::f64::NAN {
            0.0 //could do math error?
        } else {
            theta
        }
    }
}

impl GallOrd<'_> {
    pub fn svg_x(&self) -> f64 {
        match self.ang {
            Some(rad) => self.dist*(FRAC_PI_2 - rad).cos() + self.center.0,
            None => self.center.0
        }
    }
    pub fn svg_y(&self) -> f64 {
        match self.ang {
            Some(rad) => self.dist*(FRAC_PI_2 - rad).sin() + self.center.1,
            None => self.center.1
        }
    }
    //SVG is stupid, and positive angles are clockwise
    pub fn svg_ord(&self) -> (f64,f64) {
        match self.ang {
            //can I use float.sin_cos()?
            Some(rad) => (
                self.dist*(FRAC_PI_2 - rad).cos() + self.center.0,
                self.dist*(FRAC_PI_2 - rad).sin() + self.center.1
            ),
            None => self.center
        }
    }
    pub fn set_ang(&mut self, new_ang:f64) {
        self.ang = match self.ang {
            Some(_) => Some(new_ang),
            None => None,
        }
    }
    pub fn c_clockwise(&mut self, radians:f64) -> Option<()> {
        self.ang = Some(self.ang? + radians);
        Some(())
    }
    pub fn cw_step(&mut self) -> Option<()> {
        self.c_clockwise(-self.ang?.min(FRAC_PI_8/10.0))
    }
    pub fn ccw_step(&mut self) -> Option<()>{
        self.c_clockwise(FRAC_PI_8/10.0)
    }
    pub fn set_dist(&mut self, new_dist:f64) {
        self.dist = new_dist;
        if new_dist == 0.0 {
            self.ang = None;
        }
    }
    /*pub fn add_parent<'a>(&mut self, new_parent: &'a mut GallOrd<'a>) {
        self.parent = Some(new_parent)
    }*/
}