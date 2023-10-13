//use std::collections::VecDeque;
use std::env;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

mod gall_fn;
mod gall_struct;
mod gall_ord;
mod gall_phrase;
use gall_ord::GallOrd;
use gall_phrase::GallPhrase;
use gall_struct::{GallCircle, GallWord};

impl GallWord {
    fn render_syl_split(&self) -> (Vec<Circle>,Vec<&GallCircle>,Vec<&GallCircle>) {
        let mut floating_circles = Vec::new();
        let mut skele_ltrs = Vec::new();
        let mut oth_ltrs = Vec::new();
        for letter in &self.syllables {
            match letter.stem {
                gall_struct::LetterType::BStem => skele_ltrs.push(letter),
                gall_struct::LetterType::TStem => skele_ltrs.push(letter),
                _ => oth_ltrs.push(letter),
            }
            if letter.vowel.is_some() {
                let vowel = letter.vowel.as_ref().unwrap();
                let circle_vowel = Circle::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", letter.thickness)
                    .set("cx", letter.loc.svg_x())
                    .set("cy", letter.loc.svg_y())
                    .set("r", vowel.radius);
                floating_circles.push(circle_vowel);
                if vowel.repeat {
                    let vowel_repeat = Circle::new()
                        .set("fill", "none")
                        .set("stroke", "black")
                        .set("stroke-width", letter.thickness)
                        .set("cx", letter.loc.svg_x())
                        .set("cy", letter.loc.svg_y())
                        .set("r", vowel.radius - 2.0*letter.thickness);
                    floating_circles.push(vowel_repeat);
                }
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
                } //dashes are rendered by sentence
            }
        }
        (floating_circles,skele_ltrs,oth_ltrs)
    }
    fn render_skele_path(&self, skeleton_letters:Vec<&GallCircle>) -> (Path, Path, Vec<Path>) {
        let mut repeat_circles = Vec::new();
        let mut first = true;
        let mut b_divot_flag = 0;
        let mut letter_dist = skeleton_letters[0].loc.dist;
        let mut letter_smaller_radius = skeleton_letters[0].inner_rad();
        let mut letter_larger_radius = skeleton_letters[0].outer_rad();
        if skeleton_letters[0].repeat {
            letter_larger_radius -= 4.0*skeleton_letters[0].thickness;
        }
        if skeleton_letters[0].stem == gall_struct::LetterType::BStem {
            b_divot_flag = 1;
        } else {
            letter_dist += skeleton_letters[0].thickness
        }
        if self.inner_radius - letter_dist - letter_larger_radius >= 0.0 {
            panic!("Letter not touching outer skeleton");
        }
        if self.outer_radius - letter_dist - letter_smaller_radius >= 0.0 {
            panic!("Letter not touching inner skeleton");
        }
        let mut thi_inner = gall_fn::thi(
            letter_dist,
            letter_larger_radius, 
            self.inner_radius
        );
        let mut thi_outer = gall_fn::thi(
            letter_dist,
            letter_smaller_radius, 
            self.outer_radius
        );
        let mut inner_word_end_angle = skeleton_letters[0].loc.ang.unwrap() - thi_inner;
        let mut outer_word_end_angle = skeleton_letters[0].loc.ang.unwrap() - thi_outer;
        let inner_init_angle = 0.0_f64.min(inner_word_end_angle);
        let outer_init_angle = 0.0_f64.min(outer_word_end_angle);
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

        let shape = skeleton_letters[0].gen_repeat_path(letter_dist, self.inner_radius, self.loc.svg_ord());
        match shape {
            Some(repeat_path) => repeat_circles.push(repeat_path),
            None => {}
        }

        let mut long_inner_skeleton = 0;
        let mut long_outer_skeleton = 0;
        if inner_word_end_angle > std::f64::consts::PI {
            long_inner_skeleton = 1;
        }
        if outer_word_end_angle > std::f64::consts::PI {
            long_outer_skeleton = 1;
        }
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
                letter_larger_radius, 
                letter_larger_radius, 
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
                letter_smaller_radius,
                letter_smaller_radius, 
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
            letter_larger_radius = letter.outer_rad();
            letter_smaller_radius = letter.inner_rad(); 
            if letter.repeat {
                letter_larger_radius -= 4.0*letter.thickness;
            }
            let shape = letter.gen_repeat_path(letter_dist, self.inner_radius, self.loc.svg_ord());
            match shape {
                Some(repeat_path) => repeat_circles.push(repeat_path),
                None => {}
            }
            thi_inner = gall_fn::thi(
                letter_dist, 
                letter_larger_radius, 
                self.inner_radius
            );
            thi_outer = gall_fn::thi(
                letter_dist, 
                letter_smaller_radius, 
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
                    letter_larger_radius, 
                    letter_larger_radius,
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
                    letter_smaller_radius, 
                    letter_smaller_radius,
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
        (inner_path, outer_path, repeat_circles)
    }
    pub fn render(&self, mut svg_doc:Document) -> SVG {
        let (
            attached_letters, 
            skeleton_letters, 
            other_letters
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
            let (inner_path, outer_path, repeats) = self.render_skele_path(skeleton_letters);
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
            for extra_letter in repeats {
                svg_doc = svg_doc.add(extra_letter
                    .set("fill", "red")
                    .set("stroke-width", 0)
                    .set("stroke", "none")
                );
            }
        }
        for letter in other_letters {
            let circle_outline = Circle::new()
                .set("fill", "black")
                .set("stroke", "none")
                .set("stroke-width", 0)
                .set("cx", letter.loc.svg_x())
                .set("cy", letter.loc.svg_y())
                .set("r", letter.outer_rad());
            svg_doc = svg_doc.add(circle_outline);
            let circle = Circle::new()
                .set("fill", "blue")
                .set("stroke", "none")
                .set("stroke-width", 0)
                .set("cx", letter.loc.svg_x())
                .set("cy", letter.loc.svg_y())
                .set("r", letter.inner_rad());
            svg_doc = svg_doc.add(circle);
            if letter.repeat {
                let circle = Circle::new()
                    .set("fill", "none")
                    .set("stroke", "blue")
                    .set("stroke-width", 2.0*letter.thickness)
                    .set("cx", letter.loc.svg_x())
                    .set("cy", letter.loc.svg_y())
                    .set("r", letter.radius);
                svg_doc = svg_doc.add(circle);
            }
        }
        for node in attached_letters {
            svg_doc = svg_doc.add(node);
        }
        svg_doc
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
    let mut sentence = GallPhrase{words:Vec::new(),radius:WIDTH/2.0 - 6.0};
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
    println!("Deciding layout...");
    for word in &mut sentence.words {
        word.distribute();
        word.update_kids();
    }
    sentence.dock_words();
    //TODO: Layout stuff here?
    sentence.dash_pair_loop(); //create dash pair paths here?
    
    println!("Rendering...");
    let mut drawn = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));   
    drawn = sentence.render(drawn, ORIGIN);    
    println!("Saving under {}", filename);
    match svg::save(filename + ".svg", &drawn) {
        Ok(_) => println!("Done!"),
        Err(message) => println!("{}", message),
    }
}
