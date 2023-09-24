use std::f64::consts::FRAC_PI_2;

//below is python
//math.acos((Wrd.inner_rad**2 + dist**2 - self.outer_rad**2)/(2*dist*Wrd.inner_rad))
pub fn thi(letter_distance:f64, letter_radius:f64,big_radius:f64) -> f64 {
    let thi = ((big_radius.powf(2.0) + letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*big_radius)).acos();
    if thi == std::f64::NAN {
        0.0 //could do math error?
    } else {
        thi
    }
}

//below is python
//self.theta  = math.acos((Wrd.inner_rad**2 - dist**2 - self.outer_rad**2)/(2*dist*self.outer_rad))
pub fn theta(letter_distance:f64, letter_radius:f64,big_radius:f64) -> f64 {
    let theta = ((big_radius.powf(2.0) - letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*letter_radius)).acos();
    if theta == std::f64::NAN {
        0.0 //could do math error?
    } else {
        theta
    }
}

#[derive(PartialEq,Default)]
pub enum  LetterType {
    Digit,
    StaticVowel,
    BStem,
    JStem,
    TStem,
    ZStem,
    FloatingVowel,
    #[default]
    Punctuation, //more for error case than anything
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
    pub fn set_ang_d(&mut self, co_ord: (f64,f64)) {
        self.ang = Some(co_ord.0);
        self.dist = co_ord.1;
    }
    pub fn set_ang(&mut self, new_ang:f64) {
        self.ang = Some(new_ang);
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
    pub fn set_dist(&mut self, new_dist:f64) {
        self.dist = new_dist
    }
    /*pub fn add_parent<'a>(&mut self, new_parent: &'a mut GallOrd<'a>) {
        self.parent = Some(new_parent)
    }*/
}