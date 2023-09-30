use std::env;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

mod gall_fn;
mod gall_struct;
use gall_struct::{GallCircle, GallOrd};


fn text_to_gall<'a>(text: String, word_radius:f64, origin: &(f64,f64)) -> (usize, Vec<GallCircle<'a>>) {
    let count_guess = text.len(); //len() is byte len, not # of chars
    let mut syllable_list = Vec::with_capacity(count_guess);
    let mut count:usize = 0;
    let letter_sep_ang = std::f64::consts::TAU/(count_guess as f64);
    let mut text_iter = text.chars(); 
    let mut letter = text_iter.next();
    loop {
        if letter.is_none() {
            break;
        }
        count += 1;
        let mut vowel =None;
        let char1 = letter.unwrap();
        let stem = gall_fn::stem_lookup(&char1);
        let letter_size = gall_fn::stem_size(&stem);
        letter = text_iter.next();
        if letter.is_some() && gall_fn::stem_lookup(&letter.unwrap()) == gall_struct::LetterType::StaticVowel {
            vowel = Some(gall_struct::VowCircle {
                character: letter.unwrap(),
                repeat: false,
                radius: letter_size/2.0
            });
            letter = text_iter.next();
        }
        let letter_loc = GallOrd::new( 
            Some(letter_sep_ang * count as f64), 
            gall_fn::stem_dist(&stem, word_radius), 
            origin.to_owned(), 
            None
        );
        
        //make mut later when doing dots & dashes
        let decor_list = Vec::new();
        let syllable = GallCircle::new(
            char1,
            stem,
            false,
            vowel, //for attached vowels only
            letter_loc,                    
            letter_size,
            2.0,
            decor_list,
        );
        syllable_list.push(syllable);
        
    }
    (count, syllable_list)
}

impl GallCircle<'_> {
    fn generate_decor(&mut self) {
        let (dot, mut decor_num) = gall_fn::decor_lookup(&self.character);
        while decor_num > 0 {
            let dec_loc = GallOrd::new(
                Some(0.2 * decor_num as f64),
                self.radius,
                self.loc.svg_ord(),
                None //TODO fix
            );
            let dec = gall_struct::Decor{
                loc: dec_loc,
                dot: dot.unwrap(),
            };
            self.decorators.push(dec);
            decor_num -= 1;
        }
    }
}

impl gall_struct::GallWord<'_> {
    //generates a list of angles between letters, as measured by thi 
    fn angular_distance_list(&self) -> Vec<f64> {
        let mut angle_list = Vec::new();
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
                    success = self.syllables[index].loc.c_clockwise(right_dist_weight/3.0);
                } else {
                    success = match right_dist_weight.signum() {
                        1.0 => self.syllables[index].loc.ccw_step(),
                        -1.0 => self.syllables[index].loc.cw_step(),
                        _ => success
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
    fn skele_syl_split(&self) -> (Vec<Circle>,Vec<&GallCircle>,Vec<&GallCircle>) {
        let mut attached_letters = Vec::new();
        let mut skele_ltrs = Vec::new();
        let mut oth_ltrs = Vec::new();
        for letter in &self.syllables {
            match letter.stem {
                gall_struct::LetterType::BStem => skele_ltrs.push(letter),
                gall_struct::LetterType::TStem => skele_ltrs.push(letter),
                _ => oth_ltrs.push(letter),
            }
            if letter.vowel.is_some() {
                let circle_vowel = Circle::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 3)
                    .set("cx", letter.loc.svg_x())
                    .set("cy", letter.loc.svg_y())
                    .set("r", letter.vowel.as_ref().unwrap().radius);
                attached_letters.push(circle_vowel);
            } 
        }
        (attached_letters,skele_ltrs,oth_ltrs)
    }
    fn render_skele_path(&self, skeleton_letters:Vec<&GallCircle>) -> (Path,Path) {
        let mut first = true;
        let mut b_divot_flag = 0;
        if skeleton_letters[0].stem == gall_struct::LetterType::BStem {
            b_divot_flag = 1;
        }
        let mut thi_inner = self.inner_thi(skeleton_letters[0]);
        let mut thi_outer = self.outer_thi(skeleton_letters[0]);
        let (ang, inner_init_angle, outer_init_angle) = match skeleton_letters[0].loc.ang {
            Some(angle) => (
                angle,
                0.0_f64.min(angle - thi_inner), 
                0.0_f64.min(angle - thi_outer),
            ),
            None => ( f64::NAN,0.0,0.0,),
        };
        let mut inner_loc = GallOrd::new(
            Some(inner_init_angle),
            self.inner_radius,
            self.loc.svg_ord(),
            None,
        );
        let mut outer_loc = GallOrd::new(
            Some(outer_init_angle),
            self.outer_radius,
            self.loc.svg_ord(),
            None,
        );
        let inner_continuum = inner_loc.svg_ord();
        let outer_continuum = outer_loc.svg_ord();
        let mut inner_long_skeleton = 0;
        let mut outer_long_skeleton = 0;
        if ang - thi_inner > std::f64::consts::PI {
            inner_long_skeleton = 1;
        }
        if ang - thi_outer > std::f64::consts::PI {
            outer_long_skeleton = 1;
        }
        let mut inner_word_angle = ang - thi_inner;
        let mut outer_word_angle = ang - thi_outer;
        inner_loc.set_ang( inner_word_angle);
        outer_loc.set_ang( outer_word_angle);
        let mut inner_letter_arc_start = inner_loc.svg_ord();
        let mut outer_letter_arc_start = outer_loc.svg_ord();
        inner_loc.c_clockwise(2.0 * thi_inner);
        outer_loc.c_clockwise(2.0 * thi_outer);
        let mut inner_letter_arc_finish = inner_loc.svg_ord();
        let mut outer_letter_arc_finish = outer_loc.svg_ord();
        //actually fill in data
        let mut inner_data = Data::new()
            .move_to(inner_continuum)
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            .elliptical_arc_to((self.inner_radius,self.inner_radius, 0,inner_long_skeleton,0,inner_letter_arc_start.0,inner_letter_arc_start.1))
            .elliptical_arc_to((skeleton_letters[0].inner_radius, skeleton_letters[0].inner_radius,0,b_divot_flag,1,inner_letter_arc_finish.0,inner_letter_arc_finish.1));
        let mut outer_data = Data::new()
            .move_to(outer_continuum)
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            .elliptical_arc_to((self.outer_radius,self.outer_radius, 0,outer_long_skeleton,0,outer_letter_arc_start.0,outer_letter_arc_start.1))
            .elliptical_arc_to((skeleton_letters[0].outer_radius, skeleton_letters[0].outer_radius,0,b_divot_flag,1,outer_letter_arc_finish.0,outer_letter_arc_finish.1));        
        for letter in skeleton_letters {
            if first {
                first = false;
                continue;
            }
            if letter.stem == gall_struct::LetterType::BStem {
                b_divot_flag = 1
            } else {
                b_divot_flag = 0
            }
            thi_inner = self.inner_thi(letter);
            thi_outer = self.outer_thi(letter);
            inner_word_angle = letter.loc.ang.unwrap() - thi_inner;
            outer_word_angle = letter.loc.ang.unwrap() - thi_outer;
            if inner_word_angle - inner_loc.ang.unwrap() > std::f64::consts::PI {
                inner_long_skeleton = 1
            } else {
                inner_long_skeleton = 0
            }
            if outer_word_angle - outer_loc.ang.unwrap() > std::f64::consts::PI {
                outer_long_skeleton = 1
            } else {
                outer_long_skeleton = 0
            }
            inner_loc.set_ang( inner_word_angle);
            outer_loc.set_ang( outer_word_angle);
            inner_letter_arc_start = inner_loc.svg_ord();
            outer_letter_arc_start = outer_loc.svg_ord();
            inner_loc.c_clockwise(2.0 * thi_inner);
            outer_loc.c_clockwise(2.0 * thi_outer);
            inner_letter_arc_finish = inner_loc.svg_ord();
            outer_letter_arc_finish = outer_loc.svg_ord();
            inner_data = inner_data
                .elliptical_arc_to((self.inner_radius,self.inner_radius, 0,inner_long_skeleton,0,inner_letter_arc_start.0,inner_letter_arc_start.1))
                .elliptical_arc_to((letter.outer_radius, letter.outer_radius,0,b_divot_flag,1,inner_letter_arc_finish.0,inner_letter_arc_finish.1));
            outer_data = outer_data
                .elliptical_arc_to((self.outer_radius,self.outer_radius, 0,outer_long_skeleton,0,outer_letter_arc_start.0,outer_letter_arc_start.1))
                .elliptical_arc_to((letter.inner_radius, letter.inner_radius,0,b_divot_flag,1,outer_letter_arc_finish.0,outer_letter_arc_finish.1));
        }

        let mut final_inner_sweep = 1;
        let mut final_outer_sweep = 1;
        if inner_loc.ang.unwrap() - inner_init_angle > std::f64::consts::PI {
            final_inner_sweep = 0
        }
        if outer_loc.ang.unwrap() - outer_init_angle > std::f64::consts::PI {
            final_outer_sweep = 0
        }
        let closed_inner_loop = inner_data
            .elliptical_arc_to((self.inner_radius,self.inner_radius,0,final_inner_sweep,0,inner_continuum.0,inner_continuum.1))
            .close();
        let closed_outer_loop = outer_data
            .elliptical_arc_to((self.outer_radius,self.outer_radius,0,final_outer_sweep,0,outer_continuum.0,outer_continuum.1))
            .close();
        let inner_path = Path::new()
            .set("fill", "yellow")
            .set("stroke", "black")
            .set("stroke-width", 0)
            .set("d", closed_inner_loop);
        let outer_path = Path::new()
            .set("fill", "green")
            .set("stroke", "black")
            .set("stroke-width", 0)
            .set("d", closed_outer_loop);
        (inner_path,outer_path)
    }
    pub fn render(&self, mut svg_doc:Document) -> SVG {
        let (attached_letters, skeleton_letters,other_letters) = self.skele_syl_split();
        if skeleton_letters.len() == 0 {
            let circle = Circle::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("cx", self.loc.svg_x())
                .set("cy", self.loc.svg_y())
                .set("r", self.radius);
            svg_doc = svg_doc.add(circle)
        } else {
            let (inner_path, outer_path) = self.render_skele_path(skeleton_letters);
            svg_doc = svg_doc.add(outer_path);
            svg_doc = svg_doc.add(inner_path);
        }
        for letter in other_letters {
            let circle = Circle::new()
                .set("fill", "blue")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("cx", letter.loc.svg_x())
                .set("cy", letter.loc.svg_y())
                .set("r", gall_fn::stem_size(&letter.stem));
            svg_doc = svg_doc.add(circle);
        }
        for node in attached_letters {
            svg_doc = svg_doc.add(node);
        };
        svg_doc
    }
}

fn main() {
    static WIDTH:f64 = 512.0;
    static HEIGHT:f64 = 512.0;
    //maybe lazy static it in
    let ORIGIN:GallOrd = GallOrd::new(
        None,
        0.0,
        (WIDTH/2.0,HEIGHT/2.0),
        None,
    );
    println!("Initialising...");
    let args = env::args();
    let mut word_list = Vec::new();
    let mut filename:String = "".to_string();
    for raw_word in args {
        if filename.len() == 0 {
            filename += "SVGs\\"; //Save to SVGs folder
            continue;//first argument is usually runpath
        }
        filename += &raw_word;
        word_list.push(gall_fn::string_parse(raw_word));
    }
    let (word_radius, word_angle, word_dist) = gall_fn::default_layouts(word_list.len());
    println!("Generating...");
    let mut phrase = Vec::new();
    for (num,words) in word_list.into_iter().enumerate() {
        let word_loc = GallOrd::new(
            Some(word_angle * num as f64), 
            word_dist, 
            ORIGIN.center, 
            None 
        );
        //parse letters more?
        let word_circle = gall_struct::GallWord::new(
            text_to_gall(words.to_owned(),word_radius, &word_loc.svg_ord()),
            word_loc,
            word_radius,
            9.0,
            Vec::new(),
        );
        phrase.push(word_circle);
    }
    for word in &mut phrase {
        word.distribute();
    }
    //Now generate decorators - not rendered yet
    for word in &mut phrase {
        for syllable in &mut word.syllables {
            syllable.generate_decor();
        }
    }
    
    println!("Rendering...");
    let document = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));   
    let mut drawn = document;
    for word in phrase {
        drawn = word.render(drawn);
    }
    //Draw sentence circle
    let circle = Circle::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 6)
        .set("cx", ORIGIN.svg_x())
        .set("cy", ORIGIN.svg_y())
        .set("r", 250);
    drawn = drawn.add(circle);
    println!("Saving under {}", filename);
    match svg::save(filename + ".svg", &drawn) {
        Ok(_) => println!("Done!"),
        Err(message) => println!("{}", message),
    }
}
