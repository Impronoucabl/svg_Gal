use std::f64::consts::FRAC_PI_2;
use std::f64::consts::FRAC_PI_8;

use crate::gall_fn;
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
    //position relative to syllable
    pub loc: GallOrd<'loc>,
    pub dot: bool,
    pub pair_syllable: Option<&'loc GallOrd<'loc>>,
    pub free: bool,
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
    pub fn new<'a>(text: String, loc: GallOrd<'a>,word_radius: f64,thickness: f64,decorators: Vec<Decor<'a>>) -> GallWord<'a> {
        let count_guess = text.len(); //len() is byte len, not # of chars
        let mut syllable_list = Vec::with_capacity(count_guess);
        let mut count:usize = 0;
        let letter_sep_ang = std::f64::consts::TAU/(count_guess as f64);
        let mut text_iter = text.chars(); 
        let mut letter = text_iter.next();
        while letter.is_some() {
            count += 1;
            let char1 = letter.unwrap();
            let stem = gall_fn::stem_lookup(&char1);
            let (dot, decor_num) = gall_fn::decor_lookup(&char1);
            let mut decor_list = Vec::new();
            let letter_size = gall_fn::stem_size(&stem);
            let letter_loc = GallOrd::new( 
                Some(letter_sep_ang * count as f64), 
                gall_fn::stem_dist(&stem, word_radius), 
                loc.svg_ord(), 
                None
            );
            for num in 0..decor_num {
                let dec_loc = GallOrd::new(
                    Some(letter_sep_ang * num as f64),
                    letter_size,
                    letter_loc.svg_ord(),
                    None,
                );
                let dec = Decor { 
                    loc: dec_loc,
                    dot: dot.unwrap(),
                    pair_syllable: None,
                    free:std::ops::Not::not(dot.unwrap_or(true)),
                };
                decor_list.push(dec)
            }
            let mut syllable = GallCircle::new(
                char1,
                stem,
                false,
                None, //for attached vowels only
                letter_loc,                    
                letter_size,
                3.0,
                decor_list,
            );
            letter = text_iter.next();
            if letter.is_some() && gall_fn::stem_lookup(&letter.unwrap()) == LetterType::StaticVowel {
                let vowel = VowCircle::new(
                    letter.unwrap(),
                    false,
                    letter_size/2.0,
                    &mut syllable
                );
                syllable.vowel = Some(vowel);
                letter = text_iter.next();
            }
            syllable_list.push(syllable);
        }       
        return GallWord { 
            syllables: syllable_list, 
            letter_count: count, 
            loc, 
            radius: word_radius, 
            thickness, 
            decorators, 
            inner_radius: word_radius - thickness, 
            outer_radius: word_radius + thickness, 
        }
    }
    
    //generates a list of angles between letters, as measured by thi 
    fn angular_distance_list(&self) -> Vec<f64> {
        let mut angle_list = Vec::with_capacity(self.letter_count);
        let mut angle1 = f64::NAN; //dummy value
        let mut first_angle_cache = f64::NAN;
        for letter in &self.syllables {
            let angle2 = letter.loc.ang.unwrap() - self.inner_thi(letter);
            if angle1.is_nan() {
                first_angle_cache = angle2;
                angle1 = angle2 + 2.0 * self.inner_thi(letter);
                continue;
            }
            angle_list.push(angle2 - angle1);
            angle1 = angle2 + 2.0 * self.inner_thi(letter);
        }
        
        angle_list.push(std::f64::consts::TAU + first_angle_cache - angle1);
        angle_list
    }
    fn distribute_step(&mut self) -> Option<f64> {
        let distribution = self.angular_distance_list();
        let mut success = None;
        let mut max = 0.0;
        for index in 0..self.letter_count {
            let prev:usize; 
            if index == 0 {
                prev = self.letter_count - 1;
            } else {
                prev = index - 1;
            }
            let right_dist_weight = distribution[index] - distribution[prev];
            if right_dist_weight.abs() > std::f64::consts::FRAC_PI_8/10.0{
                if right_dist_weight.abs() > 0.1 {
                    success = self.syllables[index].loc.c_clockwise(right_dist_weight/3.0, false);
                } else {
                    success = match right_dist_weight.is_sign_positive() {
                        true => self.syllables[index].loc.ccw_step(),
                        false => self.syllables[index].loc.cw_step(),
                    }
                }
                max = f64::max(max, right_dist_weight.abs());
            };
        };
        match success {
            Some(_) => Some(max),
            None => None,
        }
    }
    pub fn distribute(&mut self) {
        let mut count = 0;
        let mut max = match self.distribute_step() {
            Some(high) => high,
            None => return,
        };
        loop {
            count += 1;
            let val = match self.distribute_step() {
                Some(val0) => val0,
                None => return
            };
            if val >= max {
                return;
            }
            max = val;
            if count > 200 {
                println!("Error! Distribute timeout");
                return;
            }
        }
    }

    pub fn collect_points<'a>(&'a self, point_vec: &mut Vec<&GallOrd<'a>>) {
        for syllable in &self.syllables {
            point_vec.push(&syllable.loc)
        }
        point_vec.push(&self.loc)
    }
    pub fn collect_dashes<'a>(&'a self, dash_vec: &mut Vec<&Decor<'a>>) {
        for syllable in &self.syllables {
            for dec in &syllable.decorators {
                if dec.free {
                    dash_vec.push(&dec)
                }
            }
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

impl VowCircle {
    pub fn new(text:char, repeat: bool, radius: f64, syllable: &mut GallCircle) -> VowCircle {
        let (vowel_dot, _) = gall_fn::decor_lookup(&text);
            if vowel_dot.is_some() {
                let angle = match text {
                    'I'|'i' => syllable.loc.ang.unwrap() + FRAC_PI_2,
                    _ => syllable.loc.ang.unwrap(),
                };
                let dec_loc = GallOrd::new(
                    Some(angle),
                    radius,
                    syllable.loc.svg_ord(),
                    None,
                );
                let dec = Decor { 
                    loc: dec_loc,
                    dot: false,
                    pair_syllable: None,
                    free: true,
                };
                syllable.decorators.push(dec)
            }
        VowCircle { character: text, repeat, radius }
    }
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