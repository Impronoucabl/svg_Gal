use std::env;

use svg::Document;
use svg::node::element::{Path, Circle, SVG, Line};
use svg::node::element::path::Data;

mod gall_fn;
mod gall_struct;
use gall_struct::{GallCircle, GallOrd, GallWord, Decor};


fn word_to_gall<'a>(text: String, word_radius:f64, origin: &(f64,f64)) -> (usize, Vec<GallCircle<'a>>) {
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
        let mut vowels =None;
        let (dot, decor_num) = gall_fn::decor_lookup(&char1);
        let mut decor_list = Vec::new();
        let letter_size = gall_fn::stem_size(&stem);
        let letter_loc = GallOrd::new( 
            Some(letter_sep_ang * count as f64), 
            gall_fn::stem_dist(&stem, word_radius), 
            origin.to_owned(), 
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
            };
            decor_list.push(dec)
        }
        letter = text_iter.next();
        if letter.is_some() && gall_fn::stem_lookup(&letter.unwrap()) == gall_struct::LetterType::StaticVowel {
            let vowel = gall_struct::VowCircle {
                character: letter.unwrap(),
                repeat: false,
                radius: letter_size/2.0
            };
            let (vowel_dot, _) = gall_fn::decor_lookup(&vowel.character);
            if vowel_dot.is_some() {
                println!("{}",&vowel.character);
                let dec_loc = GallOrd::new(
                    Some(0.0),
                    vowel.radius,
                    letter_loc.svg_ord(),
                    None,
                );
                let dec = Decor { 
                    loc: dec_loc,
                    dot: false,
                };
                decor_list.push(dec)
            }
            vowels = Some(vowel);
            letter = text_iter.next();
        }
        let syllable = GallCircle::new(
            char1,
            stem,
            false,
            vowels, //for attached vowels only
            letter_loc,                    
            letter_size,
            3.0,
            decor_list,
        );
        syllable_list.push(syllable);
    }
    (count, syllable_list)
}

impl GallWord<'_> {
    fn render_syl_split(&self) -> (Vec<Circle>,Vec<&GallCircle>,Vec<&GallCircle>,Vec<&GallCircle>,Vec<Path>) {
        let mut skele_ltrs = Vec::new();
        let mut skele_ltrs2 = Vec::new();
        let mut oth_ltrs = Vec::new();
        let mut floating_circles = Vec::new();
        let mut decor_dash = Vec::new();
        for letter in &self.syllables {
            match letter.stem {
                gall_struct::LetterType::BStem => {skele_ltrs.push(letter);skele_ltrs2.push(letter);},
                gall_struct::LetterType::TStem => {skele_ltrs.push(letter);skele_ltrs2.push(letter);},
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
                floating_circles.push(circle_vowel);
            }
            for decor in &letter.decorators {
                if decor.dot {
                    let circle_dot = Circle::new()
                        .set("fill", "black")
                        .set("stroke", "none")
                        .set("stroke-width", 0)
                        .set("cx", decor.loc.svg_x())
                        .set("cy", decor.loc.svg_y())
                        .set("r", 10);
                    floating_circles.push(circle_dot);
                } else {
                    let line_path = Data::new()
                        .move_to(decor.loc.svg_ord())
                        .line_to((256,256));
                    let dash = Path::new()
                        .set("fill", "none")
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .set("d", line_path);
                    decor_dash.push(dash);
                }
            }
        }
        (floating_circles,skele_ltrs,skele_ltrs2,oth_ltrs, decor_dash)
    }
    fn render_skele_path(&self, skeleton_letters:Vec<&GallCircle>, big_radius:f64, letter_props:fn(&GallCircle)->f64) -> Path {
        let mut letter_radius = letter_props(skeleton_letters[0]);
        if big_radius - skeleton_letters[0].loc.dist - letter_radius >= 0.0 {
            panic!("Letter not touching skeleton");
        }
        let mut thi_letter = gall_fn::thi(skeleton_letters[0].loc.dist, letter_radius, big_radius);
        let init_angle = 0.0_f64.min(skeleton_letters[0].loc.ang.unwrap() - thi_letter);
        let mut tracker_loc = GallOrd::new(
            Some(init_angle),
            big_radius,
            self.loc.svg_ord(),
            None,
        );
        let continuum_pt = tracker_loc.svg_ord();

        let mut first = true;
        let mut b_divot_flag = 0;
        if skeleton_letters[0].stem == gall_struct::LetterType::BStem {
            b_divot_flag = 1;
        }
        let mut long_skeleton = 0;
        if skeleton_letters[0].loc.ang.unwrap() - thi_letter > std::f64::consts::PI {
            long_skeleton = 1;
        }
        let mut word_start_angle = skeleton_letters[0].loc.ang.unwrap() - thi_letter;
        tracker_loc.set_ang( word_start_angle);
        let mut letter_arc_start = tracker_loc.svg_ord();
        tracker_loc.c_clockwise(2.0 * thi_letter, true);
        let mut letter_arc_finish = tracker_loc.svg_ord();
        
        //actually fill in data
        let mut skele_data = Data::new()
            .move_to(continuum_pt)
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            .elliptical_arc_to((big_radius,big_radius, 0,long_skeleton,0,letter_arc_start.0,letter_arc_start.1))
            .elliptical_arc_to((letter_radius, letter_radius,0,b_divot_flag,1,letter_arc_finish.0,letter_arc_finish.1));

        for letter in skeleton_letters {
            if first {
                first = false;
                continue;
            }
            if letter.stem == gall_struct::LetterType::BStem {
                b_divot_flag = 1
            } else {
                b_divot_flag = 0 //If not Bstem, then Tstem
            }
            letter_radius = letter_props(letter);
            thi_letter = gall_fn::thi(letter.loc.dist, letter_radius, big_radius);
            word_start_angle = letter.loc.ang.unwrap() - thi_letter;
            if word_start_angle - tracker_loc.ang.unwrap() > std::f64::consts::PI {
                long_skeleton = 1
            } else {
                long_skeleton = 0
            }
            tracker_loc.set_ang( word_start_angle);
            letter_arc_start = tracker_loc.svg_ord();
            tracker_loc.c_clockwise(2.0 * thi_letter, true);
            letter_arc_finish = tracker_loc.svg_ord();
            skele_data = skele_data
                .elliptical_arc_to((big_radius,big_radius, 0,long_skeleton,0,letter_arc_start.0,letter_arc_start.1))
                .elliptical_arc_to((letter_radius, letter_radius,0,b_divot_flag,1,letter_arc_finish.0,letter_arc_finish.1));
        }

        let mut final_sweep = 1;
        if tracker_loc.ang.unwrap() - init_angle > std::f64::consts::PI {
            final_sweep = 0
        }
        let closed_loop = skele_data
            .elliptical_arc_to((big_radius,big_radius,0,final_sweep,0,continuum_pt.0,continuum_pt.1))
            .close();
        let path = Path::new()
            .set("d", closed_loop);
        path
    }
    pub fn render(&self, mut svg_doc:Document) -> SVG {
        let (
            attached_letters, 
            skeleton_letters1, 
            skeleton_letters2, 
            other_letters,
            decor_dash
        ) = self.render_syl_split();
        if skeleton_letters1.is_empty() {
            let circle = Circle::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("cx", self.loc.svg_x())
                .set("cy", self.loc.svg_y())
                .set("r", self.radius);
            svg_doc = svg_doc.add(circle)
        } else {
            let inner_path = self.render_skele_path(skeleton_letters1,self.inner_radius, gall_struct::outer_rad)
                .set("fill", "yellow")
                .set("stroke-width", 0)
                .set("stroke", "none");
            let outer_path = self.render_skele_path(skeleton_letters2,self.outer_radius, gall_struct::inner_rad)
                .set("fill", "green")
                .set("stroke-width", 0)
                .set("stroke", "none");
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
        }
        for node in decor_dash {
            svg_doc = svg_doc.add(node);
        }

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
        let word_circle = GallWord::new(
            words.to_owned(),
            word_loc,
            word_radius,
            3.0,
            Vec::new(),
        );
        phrase.push(word_circle);
    }
    for word in &mut phrase {
        word.distribute();
        word.update_kids();
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
