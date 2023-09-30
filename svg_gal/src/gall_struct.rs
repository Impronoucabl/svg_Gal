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
    pub thickness:f64,
    pub decorators:Vec<Decor<'loc>>,
    pub inner_radius: f64,
    pub outer_radius: f64,
}

#[derive(PartialEq, Default)]
pub struct GallCircle<'loc> { //Syllable equivalent
    pub character: char,
    pub stem:LetterType,
    pub repeat: bool,
    pub vowel:Option<VowCircle>,
    pub loc: GallOrd<'loc>,
    pub radius: f64,
    pub thickness: f64,
    inner_radius:f64,
    outer_radius:f64,
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
    rel_svg_x:f64,
    rel_svg_y:f64,
}

impl GallWord<'_> {
    pub fn new<'a>(gall2text_out: (usize,Vec<GallCircle<'a>>),loc: GallOrd<'a>,radius: f64,thickness: f64,decorators: Vec<Decor<'a>>) -> GallWord<'a> {
        GallWord { 
            syllables: gall2text_out.1, 
            letter_count: gall2text_out.0, 
            loc, 
            radius, 
            thickness, 
            decorators, 
            inner_radius: radius - thickness, 
            outer_radius: radius + thickness, 
        }
    }
    pub fn inner_thi(&self, letter: &GallCircle) -> f64 {
        let thi1 = ((self.inner_radius.powf(2.0) + letter.loc.dist.powf(2.0) - letter.outer_radius.powf(2.0))/(2.0*letter.loc.dist*self.inner_radius)).acos();
        if thi1.is_nan() {
            0.0
        } else {
            thi1
        }
    }
    /*pub fn outer_thi(&self, letter: &GallCircle) -> f64 {
        let thi2 = ((self.outer_radius.powf(2.0) + letter.loc.dist.powf(2.0) - letter.inner_radius.powf(2.0))/(2.0*letter.loc.dist*self.outer_radius)).acos();
        if thi2.is_nan() {
            0.0
        } else {
            thi2
        }
    }*/
    pub fn update_kids(&mut self) {
        for circle in &mut self.syllables {
            circle.loc.center = self.loc.svg_ord();
            circle.update_kids();
        }
    }
}

pub fn outer_rad(letter: &GallCircle) -> f64 {
    letter.outer_radius
}

pub fn inner_rad(letter: &GallCircle) -> f64 {
    letter.inner_radius
}

impl GallCircle<'_> {
    pub fn new<'a>(character: char,stem: LetterType,repeat: bool,vowel: Option<VowCircle>,loc: GallOrd<'a>,radius: f64,thickness:f64, decorators: Vec<Decor<'a>>) -> GallCircle<'a>{
        GallCircle { 
            character, 
            stem, 
            repeat, 
            vowel, 
            loc, 
            radius, 
            thickness,
            inner_radius: radius - thickness, 
            outer_radius: radius + thickness, 
            decorators,
        }
    }

    fn update_kids(&mut self) {
        for dec in &mut self.decorators {
            dec.loc.center = self.loc.svg_ord()
        }
    }
    //below is python
    //self.theta  = math.acos((Wrd.inner_rad**2 - dist**2 - self.outer_rad**2)/(2*dist*self.outer_rad))
    pub fn theta(&self, word:&GallWord) -> f64 {
        let theta = ((word.radius.powf(2.0) - self.loc.dist.powf(2.0) - self.inner_radius.powf(2.0))/(2.0*self.loc.dist*self.inner_radius)).acos();
        if theta.is_nan() {
            0.0 //could do math error?
        } else {
            theta
        }
    }
}

impl GallOrd<'_> {
    pub fn new<'a>(angle: Option<f64>,dist: f64,center: (f64, f64),parent: Option<&'a GallOrd<'a>>) -> GallOrd<'a> {
        let (rel_y,rel_x) = match angle {
            Some(ang) => (FRAC_PI_2 - ang).sin_cos(),
            None => (0.0,0.0)
        };
        GallOrd { 
            ang: angle,
            dist,
            center, 
            parent, 
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
    /*pub fn add_parent<'a>(&mut self, new_parent: &'a mut GallOrd<'a>) {
        self.parent = Some(new_parent)
    }*/
}