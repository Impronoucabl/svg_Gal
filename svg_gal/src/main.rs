use std::collections::VecDeque;
use std::env;

use svg::Document;
use svg::node::element::{Path, Circle, SVG, Line};
use svg::node::element::path::Data;

mod gall_fn;
mod gall_struct;
use gall_struct::{GallCircle, GallOrd, GallWord, Decor};

use crate::gall_struct::DecPair;

impl GallWord {
    fn render_syl_split(&self) -> (Vec<Circle>,Vec<&GallCircle>,Vec<&GallCircle>,Vec<Path>) {
        let mut skele_ltrs = Vec::new();
        let mut oth_ltrs = Vec::new();
        let mut floating_circles = Vec::new();
        let mut decor_dash = Vec::new();
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
                    //TODO move node rendering to sentence?
                    let destination = match decor.pair_syllable {
                        Some(addr) =>(256,256),
                        None => (300,300),
                    };
                    let line_path = Data::new()
                        .move_to(decor.loc.svg_ord())
                        .line_to(destination);
                    let dash = Path::new()
                        .set("fill", "none")
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .set("d", line_path);
                    decor_dash.push(dash);
                }
            }
        }
        (floating_circles,skele_ltrs,oth_ltrs, decor_dash)
    }
    fn render_skele_path(&self, skeleton_letters:Vec<&GallCircle>) -> (Path, Path) {
        let mut first = true;
        let mut b_divot_flag = 0;
        let mut letter_dist = skeleton_letters[0].loc.dist;
        if skeleton_letters[0].stem == gall_struct::LetterType::BStem {
            b_divot_flag = 1;
        } else {
            letter_dist += skeleton_letters[0].thickness
        }
        if self.inner_radius - letter_dist - skeleton_letters[0].outer_rad() >= 0.0 {
            panic!("Letter not touching outer skeleton");
        }
        if self.outer_radius - letter_dist - skeleton_letters[0].inner_rad() >= 0.0 {
            panic!("Letter not touching inner skeleton");
        }
        let mut thi_inner = gall_fn::thi(
            letter_dist,
            skeleton_letters[0].outer_rad(), 
            self.inner_radius
        );
        let mut thi_outer = gall_fn::thi(
            letter_dist,
            skeleton_letters[0].inner_rad(), 
            self.outer_radius
        );
        let inner_init_angle = 0.0_f64.min(skeleton_letters[0].loc.ang.unwrap() - thi_inner);
        let outer_init_angle = 0.0_f64.min(skeleton_letters[0].loc.ang.unwrap() - thi_outer);
        let mut inner_tracker = GallOrd::new(
            Some(inner_init_angle),
            self.inner_radius,
            self.loc.svg_ord(),
        );
        let mut outer_tracker = GallOrd::new(
            Some(outer_init_angle),
            self.outer_radius,
            self.loc.svg_ord(),
        );
        let inner_continuum = inner_tracker.svg_ord();
        let outer_continuum = outer_tracker.svg_ord();

        let mut long_inner_skeleton = 0;
        let mut long_outer_skeleton = 0;
        if skeleton_letters[0].loc.ang.unwrap() - thi_inner > std::f64::consts::PI {
            long_inner_skeleton = 1;
        }
        if skeleton_letters[0].loc.ang.unwrap() - thi_outer > std::f64::consts::PI {
            long_outer_skeleton = 1;
        }
        let mut inner_word_end_angle = skeleton_letters[0].loc.ang.unwrap() - thi_inner;
        let mut outer_word_end_angle = skeleton_letters[0].loc.ang.unwrap() - thi_outer;
        inner_tracker.set_ang( inner_word_end_angle);
        outer_tracker.set_ang( outer_word_end_angle);
        let mut inner_letter_start = inner_tracker.svg_ord();
        let mut outer_letter_start = outer_tracker.svg_ord();
        inner_tracker.c_clockwise(2.0 * thi_inner, true);
        outer_tracker.c_clockwise(2.0 * thi_outer, true);
        let mut inner_letter_finish = inner_tracker.svg_ord();
        let mut outer_letter_finish = outer_tracker.svg_ord();
        
        //actually fill in data
        let mut inner_data = Data::new()
            .move_to(inner_continuum)
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            .elliptical_arc_to((
                self.inner_radius,
                self.inner_radius, 
                0,
                long_inner_skeleton,
                0,
                inner_letter_start.0,
                inner_letter_start.1
            ))
            .elliptical_arc_to((
                skeleton_letters[0].outer_rad(), 
                skeleton_letters[0].outer_rad(), 
                0,
                b_divot_flag,
                1,
                inner_letter_finish.0,
                inner_letter_finish.1
            ))
        ;
        let mut outer_data = Data::new()
            .move_to(outer_continuum)
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            .elliptical_arc_to((
                self.outer_radius,
                self.outer_radius, 
                0,
                long_outer_skeleton,
                0,
                outer_letter_start.0,
                outer_letter_start.1
            ))
            .elliptical_arc_to((
                skeleton_letters[0].inner_rad(),
                skeleton_letters[0].inner_rad(), 
                0,
                b_divot_flag,
                1,
                outer_letter_finish.0,
                outer_letter_finish.1
            ))
        ;

        for letter in skeleton_letters {
            if first {
                first = false;
                continue;
            }
            if letter.stem == gall_struct::LetterType::BStem {
                b_divot_flag = 1;
                letter_dist = letter.loc.dist;
            } else {
                b_divot_flag = 0; //If not Bstem, then Tstem
                letter_dist = letter.loc.dist + letter.thickness;
            }
            //letter_radius = letter_props(letter);
            thi_inner = gall_fn::thi(
                letter_dist, 
                letter.outer_rad(), 
                self.inner_radius
            );
            thi_outer = gall_fn::thi(
                letter_dist, 
                letter.inner_rad(), 
                self.outer_radius
            );
            inner_word_end_angle = letter.loc.ang.unwrap() - thi_inner;
            if inner_word_end_angle - inner_tracker.ang.unwrap() > std::f64::consts::PI {
                long_inner_skeleton = 1
            } else {
                long_inner_skeleton = 0
            }
            outer_word_end_angle = letter.loc.ang.unwrap() - thi_outer;
            if outer_word_end_angle - outer_tracker.ang.unwrap() > std::f64::consts::PI {
                long_outer_skeleton = 1
            } else {
                long_outer_skeleton = 0
            }
            inner_tracker.set_ang( inner_word_end_angle);
            outer_tracker.set_ang( outer_word_end_angle);
            inner_letter_start = inner_tracker.svg_ord();
            outer_letter_start = outer_tracker.svg_ord();
            inner_tracker.c_clockwise(2.0 * thi_inner, true);
            outer_tracker.c_clockwise(2.0 * thi_outer, true);
            inner_letter_finish = inner_tracker.svg_ord();
            outer_letter_finish = outer_tracker.svg_ord();
            inner_data = inner_data
                .elliptical_arc_to((
                    self.inner_radius,
                    self.inner_radius,
                    0,
                    long_inner_skeleton,
                    0,
                    inner_letter_start.0,
                    inner_letter_start.1
                ))
                .elliptical_arc_to((
                    letter.outer_rad(), 
                    letter.outer_rad(),
                    0,
                    b_divot_flag,
                    1,
                    inner_letter_finish.0,
                    inner_letter_finish.1
                ))
            ;
            outer_data = outer_data
                .elliptical_arc_to((
                    self.outer_radius,
                    self.outer_radius,
                    0,
                    long_outer_skeleton,
                    0,
                    outer_letter_start.0,
                    outer_letter_start.1
                ))
                .elliptical_arc_to((
                    letter.inner_rad(), 
                    letter.inner_rad(),
                    0,
                    b_divot_flag,
                    1,
                    outer_letter_finish.0,
                    outer_letter_finish.1
                ))
            ;
        }

        let mut inner_sweep = 1;
        let mut outer_sweep = 1;
        if inner_tracker.ang.unwrap() - inner_init_angle > std::f64::consts::PI {
            inner_sweep = 0
        }
        if outer_tracker.ang.unwrap() - outer_init_angle > std::f64::consts::PI {
            outer_sweep = 0
        }
        let closed_inner_loop = inner_data
            .elliptical_arc_to((self.inner_radius,self.inner_radius,0,inner_sweep,0,inner_continuum.0,inner_continuum.1))
            .close();
        let closed_outer_loop = outer_data
            .elliptical_arc_to((self.outer_radius,self.outer_radius,0,outer_sweep,0,outer_continuum.0,outer_continuum.1))
            .close();
        let inner_path = Path::new()
            .set("d", closed_inner_loop);
        let outer_path = Path::new()
            .set("d", closed_outer_loop);
        (inner_path, outer_path)
    }
    pub fn render(&self, mut svg_doc:Document) -> SVG {
        let (
            attached_letters, 
            skeleton_letters, 
            other_letters,
            decor_dash
        ) = self.render_syl_split();
        if skeleton_letters.is_empty() {
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
            svg_doc = svg_doc.add(outer_path
                .set("fill", "green")
                .set("stroke-width", 0)
                .set("stroke", "none")
            );  
            svg_doc = svg_doc.add(inner_path
                .set("fill", "yellow")
                .set("stroke-width", 0)
                .set("stroke", "none")
            );
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

struct GallPhrase {
    words:Vec<GallWord>
}
impl GallPhrase {
    fn dash_list(&self) -> (usize, Vec<(usize,(usize,usize))>) {
        let mut dashes = Vec::new();
        let mut word_index = 0;
        let mut count:usize = 0;
        for word in &self.words {
            for dash in word.collect_dashes() {
                dashes.push((word_index, dash));
                count += 1;
            }
            word_index += 1;
        }
        (count, dashes)
    }
    fn get_mut_dash(&mut self, address:(usize,usize,usize)) ->  &mut Decor {
        &mut self.words[address.0].syllables[address.1].decorators[address.2]
    }
    fn get_syl(&self, address:(usize,usize)) -> &GallCircle {
        &self.words[address.0].syllables[address.1]
    }
}

fn main() {
    static WIDTH:f64 = 512.0;
    static HEIGHT:f64 = 512.0;
    //maybe lazy static it in
    let ORIGIN: GallOrd = GallOrd::new(
        None,
        0.0,
        (WIDTH/2.0,HEIGHT/2.0),
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
    let mut sentence = GallPhrase{words:Vec::new()};
    for (num,words) in word_list.into_iter().enumerate() {
        let word_loc = GallOrd::new(
            Some(word_angle * num as f64), 
            word_dist, 
            ORIGIN.center, 
        );
        //parse letters more?
        let word_circle = GallWord::new(
            words.to_owned(),
            word_loc,
            word_radius,
            3.0,
            Vec::new(),
        );
        sentence.words.push(word_circle);
    }
    for word in &mut sentence.words {
        word.distribute();
        word.update_kids();
    }
    let dash0 = sentence.get_mut_dash((0,1,2));
    dash0.free = false;
    let (syl_count, list_dash) = sentence.dash_list();
    let mut spare_dash = VecDeque::new(); 
    let mut pair_list = Vec::new();
    let mut dashes = list_dash.into_iter();
    for _ in 0..3 {
        let (word1, addr1) = match dashes.next() {
            Some(dec) => dec,
            None => break,
        };
        let (word2, addr2) = match dashes.next() {
            Some(dec) => dec,
            None => {spare_dash.push_front((word1,addr1));break},
        };
        if word1 == word2 && addr1.0 == addr2.0 {
            //reverse the order
            spare_dash.push_back((word2,addr2));
            spare_dash.push_back((word1,addr1));
            println!("Spare!");
            continue
        }
        pair_list.push(DecPair{
            pair_a:(word1,addr1.0,addr1.1),
            pair_b:(word2,addr2.0,addr2.1)
        });
        pair_list.push(DecPair{
            pair_b:(word1,addr1.0,addr1.1),
            pair_a:(word2,addr2.0,addr2.1)
        });
    }
    pair_list.sort();
    //println!("{}",pair_list[0].pair_a.2);

    let mut pair_iter = pair_list.iter();
    let mut pair = pair_iter.next();
    if pair.is_some() {
        let (mut pair_a, mut pair_b) = pair.unwrap().unpack();
        let mut count = 0;
        for word in &mut sentence.words {
            if pair_a.0 > count {
                continue;
            }
            for syllable in &mut word.syllables {
                if pair_a.1 > syllable.index {
                    continue;
                }
                for decorator in &mut syllable.decorators{
                    println!("{},{}:{},{}", decorator.address.0, decorator.address.1, pair_a.1, pair_a.2);
                    if decorator.address == (pair_a.1,pair_a.2) {
                        decorator.add_syl_pair(pair_b);
                        pair = pair_iter.next();
                        (pair_a,pair_b) = match pair {
                            Some(addr) => (addr.pair_a,addr.pair_b),
                            None => break,
                        };
                    }
                }
            }
            count += 1;
        }
    }

    
    println!("Rendering...");
    let document = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));   
    let mut drawn = document;
    for word in sentence.words {
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
