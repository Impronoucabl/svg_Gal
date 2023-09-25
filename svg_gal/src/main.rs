use core::panic;
use std::env;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

mod gall_fn;
mod gall_struct;
use gall_struct::{GallCircle, GallOrd};


fn text_to_gall<'a>(text: String, word_radius:f64, origin: &(f64,f64)) -> Vec<GallCircle<'a>> {
    let mut syllable_list = Vec::new();
    let letter_sep_ang = 2.0 * std::f64::consts::PI/(text.len() as f64);
    for (n, letter) in text.chars().enumerate() {
        let stem = gall_fn::stem_lookup(letter);
        let letter_loc = GallOrd { 
            ang: Some(letter_sep_ang * n as f64), 
            dist: gall_fn::stem_dist(&stem, word_radius), 
            center: origin.to_owned(), 
            parent: None
        };
        let letter_size = gall_fn::stem_size(&stem);
        //make mut later when doing dots & dashes
        let decor_list = Vec::new();
        let syllable = GallCircle{
            character: letter,
            stem:stem,
            repeat: false,
            vowel: None, //for attached vowels only
            loc:letter_loc,                    
            radius: letter_size,
            decorators: decor_list,
        };
        syllable_list.push(syllable);
    }
    syllable_list
}

//Holding code until I actually do dots & dashes
fn _decor_gen(letter:char, origin: (f64,f64)) {
    let (dot, mut decor_num) = gall_fn::decor_lookup(letter);
    let letter_size = 30.0;//remove later
    while decor_num > 0 {
        let dec_loc = GallOrd{
            ang:Some(0.2 * decor_num as f64),
            dist:letter_size,
            center: origin,
            parent: None //TODO fix
        };
        let decor_type = match dot {
            Some(boolean) => boolean,
            None => panic!("Decorator has no type.")
        };
        let _dec = gall_struct::Decor{
            loc: dec_loc,
            dot: decor_type,
        };
        //decor_list.push(dec);
        decor_num -= 1;
    }
}

impl gall_struct::GallWord<'_> {
    fn skele_syl_split(&self) -> (Vec<&GallCircle>,Vec<&GallCircle>) {
        let mut skele_ltrs = Vec::new();
        let mut oth_ltrs = Vec::new();
        for letter in &self.syllables {
            match letter.stem {
                gall_struct::LetterType::BStem => skele_ltrs.push(letter),
                gall_struct::LetterType::TStem => skele_ltrs.push(letter),
                _ => oth_ltrs.push(letter),
            } 
        }
        (skele_ltrs,oth_ltrs)
    }

    fn render_skele_path(&self, skeleton_letters:Vec<&GallCircle>) -> Path {
        let mut thi_letter = gall_fn::thi(skeleton_letters[0].loc.dist,skeleton_letters[0].radius,self.radius);
        let init_angle = 0.0_f64.min(skeleton_letters[0].loc.ang.unwrap() - thi_letter);
        let mut tracker_loc = GallOrd{
            ang: Some(init_angle),
            dist: self.radius,
            center: self.loc.svg_ord(),
            parent: None,
        };
        let continuum_pt = tracker_loc.svg_ord();
        let mut first = true;
        let mut b_divot_flag = 0;
        let mut long_skeleton = 0;
        if skeleton_letters[0].loc.ang.unwrap() - thi_letter > std::f64::consts::PI {
            long_skeleton = 1;
        }
        let mut word_start_angle = skeleton_letters[0].loc.ang.unwrap() - thi_letter;
        if skeleton_letters[0].stem == gall_struct::LetterType::BStem {
            b_divot_flag = 1;
        }
        tracker_loc.set_ang( word_start_angle);
        let mut letter_arc_start = tracker_loc.svg_ord();
        tracker_loc.c_clockwise(2.0 * thi_letter);
        let mut letter_arc_finish = tracker_loc.svg_ord();
        
        //actually fill in data
        let mut skele_data = Data::new()
            .move_to(continuum_pt)
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            .elliptical_arc_to((self.radius,self.radius, 0,long_skeleton,0,letter_arc_start.0,letter_arc_start.1))
            .elliptical_arc_to((skeleton_letters[0].radius, skeleton_letters[0].radius,0,b_divot_flag,1,letter_arc_finish.0,letter_arc_finish.1));

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
            thi_letter = gall_fn::thi(letter.loc.dist,letter.radius,self.radius);
            word_start_angle = letter.loc.ang.unwrap() - thi_letter;
            if word_start_angle - tracker_loc.ang.unwrap() > std::f64::consts::PI {
                long_skeleton = 1
            } else {
                long_skeleton = 0
            }
            tracker_loc.set_ang( word_start_angle);
            letter_arc_start = tracker_loc.svg_ord();
            tracker_loc.c_clockwise(2.0 * thi_letter);
            letter_arc_finish = tracker_loc.svg_ord();
            skele_data = skele_data
                .elliptical_arc_to((self.radius,self.radius, 0,long_skeleton,0,letter_arc_start.0,letter_arc_start.1))
                .elliptical_arc_to((letter.radius, letter.radius,0,b_divot_flag,1,letter_arc_finish.0,letter_arc_finish.1));
        }

        let mut final_sweep = 1;
        if tracker_loc.ang.unwrap() - init_angle > std::f64::consts::PI {
            final_sweep = 0
        }
        let closed_loop = skele_data
            .elliptical_arc_to((self.radius,self.radius,0,final_sweep,0,continuum_pt.0,continuum_pt.1))
            .close();

        Path::new()
            .set("fill", "green")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", closed_loop)
    }

    pub fn render(&self, mut svg_doc:Document) -> SVG {
        let (skeleton_letters,other_letters) = self.skele_syl_split();
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
            let path = self.render_skele_path(skeleton_letters);
            svg_doc = svg_doc.add(path)
        }
        if other_letters.len() != 0 {
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
        };
        svg_doc
    }
}

fn main() {
    static WIDTH:f64 = 512.0;
    static HEIGHT:f64 = 512.0;
    static ORIGIN:GallOrd = GallOrd{
        ang: None,
        dist: 0.0,
        center: (WIDTH/2.0,HEIGHT/2.0),
        parent: None,
    };
    println!("Initialising...");
    let mut args= env::args();
    args.next();
    let word_list: Vec<String> = args.collect();
    let (word_radius, word_angle, word_dist) = match word_list.len() {
        0|1 => (200.0,0.0,0.0),
        2 => (80.0,std::f64::consts::PI,120.0),
        phrase_len => (
            50.0,
            2.0 * std::f64::consts::PI/(phrase_len as f64),
            150.0,
        ),
    };
    println!("Generating...");
    let mut phrase = Vec::new();
    let mut filename:String = "SVGs\\".to_string();
    for (num,words) in word_list.into_iter().enumerate() {
        let word_loc = GallOrd {
            ang: Some(word_angle * num as f64), 
            dist:word_dist, 
            center: ORIGIN.center, 
            parent: None 
        };
        let all_letters = text_to_gall(words.to_owned(),word_radius, &word_loc.svg_ord());
        let word_circle = gall_struct::GallWord {
            syllables: all_letters,
            loc: word_loc,
            radius: word_radius,
            decorators: Vec::new(),
        };
        phrase.push(word_circle);
        filename += &words;
    }
    //Do fancy stuff here?
    
    println!("Rendering...");
    let document = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));   
    let mut drawn = document;
    for word in phrase {
        drawn = word.render(drawn);
    }
    println!("Saving...");
    print!("{}",filename);
    svg::save(filename + ".svg", &drawn).unwrap();
}
