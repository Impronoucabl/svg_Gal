use std::error::Error;
use std::f64::consts::{FRAC_PI_2, PI};

use crate::gall_ord::{GallLoc, PositiveDist};
use crate::gall_fn::{self, LetterType};

pub trait Circle {
    fn get_outer_radius(&self) -> PositiveDist;
    fn get_inner_radius(&self) -> PositiveDist;
    fn mut_radius(&mut self, val:f64) -> Result<(), Box<dyn Error>>;
}

#[derive(PartialEq, Default)]
pub struct GallWord {
    pub syllables: Vec<GallCircle>,
    pub letter_count: usize,
    pub loc: GallLoc,
    pub radius: f64,
    pub thickness:f64,
    pub decorators:Vec<Decor>,
    pub inner_radius: f64,
    pub outer_radius: f64,
}

#[derive(PartialEq, Default)]
pub struct GallCircle { //Syllable equivalent
    pub character: char,
    pub stem:LetterType,
    pub repeat: bool,
    pub vowel:Option<VowCircle>,
    pub loc: GallLoc,
    pub radius: f64,
    pub thickness: f64,
    inner_radius:f64,
    outer_radius:f64,
    pub decorators:Vec<Decor>,
    pub index: usize,
    attached_o_flag: Option<usize>,
}
#[derive(PartialEq,Default)]
pub struct VowCircle { //for attached vowels only
    pub character: char,
    pub repeat: bool,
    pub radius: f64,
}
#[derive(PartialEq,Default)]
pub struct Decor {
    //position relative to syllable
    pub loc: GallLoc,
    pub dot: bool,
    pub pair_syllable: Option<(usize,usize,usize)>,
    pub free: bool,
    pub address: (usize,usize),
}

impl Decor {
    pub fn add_syl_pair(&mut self, pair: (usize,usize,usize)) {
        self.pair_syllable = Some(pair)
    }
}

impl GallWord{
    pub fn new(text: String, loc: GallLoc,word_radius: f64,thickness: f64,decorators: Vec<Decor>) -> GallWord {
        let count_guess = text.len(); //len() is byte len, not # of chars
        let mut syllable_list = Vec::with_capacity(count_guess);
        let mut o_attach_list = Vec::new();
        let mut count:usize = 0;
        let letter_sep_ang = std::f64::consts::TAU/(count_guess as f64);
        let mut text_iter = text.chars(); 
        let mut letter = text_iter.next();
        while letter.is_some() {
            let char1 = letter.unwrap();
            let (stem, repeat) = gall_fn::stem_lookup(&char1);
            let (dot, decor_num) = gall_fn::decor_lookup(&char1);
            let mut decor_list = Vec::new();
            let letter_size = gall_fn::stem_size(&stem);
            let letter_loc = GallLoc::new( 
                Some(letter_sep_ang * count as f64), 
                gall_fn::stem_dist(&stem, word_radius), 
                loc.svg_ord(), 
            );
            for num in 0..decor_num {
                let dec_loc = GallLoc::new(
                    Some(letter_sep_ang * num as f64),
                    letter_size,
                    letter_loc.svg_ord(),
                );
                let dec = Decor { 
                    loc: dec_loc,
                    dot: dot.unwrap(),
                    pair_syllable: None,
                    free:!dot.unwrap_or(true),
                    address: (count,num),
                };
                decor_list.push(dec)
            }
            count += 1;
            let mut syllable = GallCircle::new(
                char1,
                stem,
                repeat,
                None, //for attached vowels only
                letter_loc,                    
                letter_size,
                3.0,
                decor_list,
                count,
            );
            letter = text_iter.next();
            if letter.is_none() {
                syllable_list.push(syllable);
                continue;    
            }
            match syllable.stem {
                LetterType::AVowel|LetterType::OVowel|LetterType::StaticVowel => {
                    syllable_list.push(syllable);
                    continue;    
                },
                _ => {},
            }
            let (vowel_stem, repeated) = gall_fn::stem_lookup(&letter.unwrap());
            let vowel_size = gall_fn::stem_size(&vowel_stem);
            if vowel_stem == LetterType::StaticVowel {
                let vowel = VowCircle::new(
                    letter.unwrap(),
                    repeated,
                    vowel_size,
                    &mut syllable
                );
                syllable.vowel = Some(vowel);
                letter = text_iter.next();
            } else if vowel_stem == LetterType::OVowel {
                o_attach_list.push(count);
            }
            syllable_list.push(syllable);    
        }       
        let mut word = GallWord { 
            syllables: syllable_list, 
            letter_count: count, 
            loc, 
            radius: word_radius, 
            thickness, 
            decorators, 
            inner_radius: word_radius - thickness, 
            outer_radius: word_radius + thickness, 
        };
        for o_index in o_attach_list {
            word.attach_o(o_index);
        }
        word
    }
    
    pub fn attach_o(&mut self, o_addr:usize) {
        let target = &self.syllables[o_addr - 1];
        let xy_loc = target.loc.svg_ord();
        let new_dist = match target.stem {
            LetterType::TStem => target.radius - target.thickness,
            _ => target.radius,
        }; 
        let new_ang = match target.loc.ang {
            Some(ang) => ang + PI,
            None => 0.0,
        };
        let o_syl = &mut self.syllables[o_addr];
        o_syl.attached_o_flag =  Some(o_addr - 1);
        o_syl.loc.set_dist(new_dist);
        o_syl.loc.mut_ang(new_ang);        
        o_syl.loc.update_center(xy_loc);
    }
    
    //generates a list of angles between letters, as measured by thi 
    fn angular_distance_list(&self) -> Vec<f64> {
        let mut angle_list = Vec::with_capacity(self.letter_count);
        let mut angle1 = f64::NAN; //dummy value
        let mut first_angle_cache = f64::NAN;
        for letter in &self.syllables {
            if letter.is_attached_o() {
                angle_list.push(0.0);
                continue;
            }
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
        //TODO: Fix attached Os
        for index in 0..self.letter_count {
            if distribution[index] == 0.0 {
                continue
            }
            let prev:usize; 
            if index == 0 {
                prev = self.letter_count - 1;
            } else {
                prev = index - 1;
            }
            let mut left_dist = distribution[prev];
            if left_dist == 0.0 {
                left_dist = match index {
                    1 => distribution[self.letter_count - 1],
                    _ => distribution[prev - 1],
                };
            }
            let right_dist_weight = distribution[index] - left_dist;
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
        self.update_kids();
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
                Some(val_0) => val_0,
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
    //collections return a vector of addresses, instead of the objects themselves.
    pub fn collect_t_stem(&self) -> Vec<usize> {
        let mut list = Vec::new();
        let mut count = 0;
        for syllable in &self.syllables {
            if syllable.stem == LetterType::TStem {
                list.push(count)
            }
            count += 1;
        }
        list
    }
    pub fn collect_dashes(&self) -> Vec<(usize,usize)> {
        let mut list = Vec::new();
        let mut syl_index = 0;
        for syllable in &self.syllables {
            let mut dec_index = 0;
            for dec in &syllable.decorators {
                if dec.free & !dec.dot {
                    list.push((syl_index, dec_index));
                }
                dec_index += 1;
            }
            syl_index += 1;
        }
        list
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
        let mut o_list = Vec::new();
        for circle in &mut self.syllables {
            match circle.attached_o_flag {
                Some(addr) => {
                    o_list.push(addr); 
                    continue;},
                None => {
                    circle.loc.update_center(self.loc.svg_ord());
                    circle.update_kids();
                }
            }
        }
        for o_index in o_list {
            let new_center = self.syllables[o_index].loc.svg_ord();
            let o_syl = &mut self.syllables[o_index + 1];
            o_syl.loc.update_center(new_center);
            o_syl.update_kids();
        }
    }
}

impl VowCircle {
    pub fn new(text:char, repeat: bool, radius: f64, syllable: &mut GallCircle) -> VowCircle {
        let vowel_radius = match repeat {
            true => radius + syllable.thickness,
            false => radius
        };
        let (vowel_dot, _) = gall_fn::decor_lookup(&text);
        if vowel_dot.is_some() {
            let angle = match text {
                'I'|'i' => syllable.loc.ang.unwrap() + FRAC_PI_2,
                _ => syllable.loc.ang.unwrap(),
            };
            let dec_loc = GallLoc::new(
                Some(angle),
                vowel_radius,
                syllable.loc.svg_ord(),
            );
            let dec = Decor { 
                loc: dec_loc,
                dot: false,
                pair_syllable: None,
                free: true,
                address: (syllable.index,syllable.decorators.len())
            };
            syllable.decorators.push(dec)
        }
        VowCircle { character: text, repeat, radius: vowel_radius }
    }
}

impl GallCircle {
    pub fn new<'a>(character: char,stem: LetterType,repeat: bool,vowel: Option<VowCircle>,loc: GallLoc,radius: f64,thickness:f64, decorators: Vec<Decor>, index:usize) -> GallCircle{
        let mut main_radius = radius;
        if repeat {main_radius += 2.0*thickness}
        let (inner_radius, outer_radius) = match repeat {
            true  => (main_radius - 3.0*thickness, main_radius + 3.0*thickness),
            false => (main_radius -     thickness, main_radius +     thickness)
        };
        GallCircle { 
            character, 
            stem, 
            repeat, 
            vowel, 
            loc, 
            radius: main_radius, 
            thickness,
            inner_radius, 
            outer_radius, 
            decorators,
            index,
            attached_o_flag: None,
        }
    }
    pub fn is_attached_o(&self) -> bool {
        self.attached_o_flag.is_some()
    }
    pub fn outer_rad(&self) -> f64 {
        self.outer_radius
    }
    pub fn inner_rad(&self) -> f64 {
        self.inner_radius
    }
    fn update_kids(&mut self) {
        for dec in &mut self.decorators {
            dec.loc.update_center(self.loc.svg_ord());
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