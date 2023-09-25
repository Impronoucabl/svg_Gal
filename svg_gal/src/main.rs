use core::panic;
use std::env;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

mod gall_fn;
mod gall_struct;
use gall_struct::{GallCircle, GallOrd};


fn text_to_gall<'a>(text: String, word_radius:f64, origin: (f64,f64)) -> Vec<GallCircle<'a>> {
    let mut syllable_list = Vec::new();
    let letter_sep_ang = 2.0 * std::f64::consts::PI/(text.len() as f64);
    for (n, letter) in text.chars().enumerate() {
        let stem = gall_fn::stem_lookup(letter);
        let letter_loc = GallOrd { 
            ang: Some(letter_sep_ang * n as f64), 
            dist: gall_fn::stem_dist(&stem, word_radius), 
            center: origin, 
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
        let mut init_angle = 0.0;
        let mut thi_letter = gall_fn::thi(skeleton_letters[0].loc.dist,skeleton_letters[0].radius,self.radius);
        if thi_letter > 0.0 { //skeleton_letters[0].character == '_'
            init_angle -= thi_letter;
        }
        let mut tracker_loc = GallOrd{
            ang: Some(init_angle),
            dist: self.radius,
            center: self.loc.center,
            parent: None,
        };
        let continuum_pt = tracker_loc.svg_ord();
        let mut first = true;
        let mut b_divot_flag = 0;
        
        let mut angle = match skeleton_letters[0].loc.ang {
            Some(ang) => ang,
            None => 0.0
        };
        if skeleton_letters[0].stem == gall_struct::LetterType::BStem {
            b_divot_flag = 1;
        }
        tracker_loc.set_ang( angle - thi_letter);
        let mut letter_arc_start = tracker_loc.svg_ord();
        tracker_loc.c_clockwise(2.0 * thi_letter);
        let mut letter_arc_finish = tracker_loc.svg_ord();
        
        //actually fill in data
        let mut skele_data = Data::new()
            .move_to(continuum_pt)
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            .elliptical_arc_to((self.radius,self.radius, 0,0,0,letter_arc_start.0,letter_arc_start.1))
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
            angle = match letter.loc.ang {
                Some(ang) => ang,
                None => 0.2
            };
            tracker_loc.set_ang( angle - thi_letter);
            letter_arc_start = tracker_loc.svg_ord();
            tracker_loc.c_clockwise(2.0 * thi_letter);
            letter_arc_finish = tracker_loc.svg_ord();
            skele_data = skele_data
                .elliptical_arc_to((self.radius,self.radius, 0,0,0,letter_arc_start.0,letter_arc_start.1))
                .elliptical_arc_to((letter.radius, letter.radius,0,b_divot_flag,1,letter_arc_finish.0,letter_arc_finish.1));
        }

        let mut final_sweep = 1;
        if -angle - thi_letter + init_angle < -std::f64::consts::FRAC_PI_2 {
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
    let width = 512.0;
    let height = 512.0;
    let args: Vec<String> = env::args().collect();
    let raw_text = &args[1];
    let seed_text = &args[2];
    let _seed = seed_text.to_owned().into_bytes();
    let origin = GallOrd{
        ang: None,
        dist: 0.0,
        center: (width/2.0,height/2.0),
        parent: None,
    };
    println!("Generating...");
    let word_radius = 200.0;
    let all_letters = text_to_gall(raw_text.to_owned(),word_radius, origin.center);
    let word_circle = gall_struct::GallWord {
        syllables: all_letters,
        loc: origin,
        radius: word_radius,
        decorators: Vec::new(),
    };
    //Do fancy stuff here?

    
    println!("Rendering...");
    let document = Document::new().set("viewBox", (0, 0, width, height));   
    let rendered = word_circle.render(document);
    println!("Saving...");
    svg::save(raw_text.to_owned() + ".svg", &rendered).unwrap();
    println!("Done!");
}
