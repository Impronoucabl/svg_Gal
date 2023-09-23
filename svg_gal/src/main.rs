use std::env;

use gall_struct::GallCircle;
use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

mod gall_struct;

fn text_to_gall<'a>(text: String, origin: (f64,f64)) -> Vec<GallCircle<'a>> {
    let mut syllable_list = Vec::new();
    for (n, letter) in text.chars().enumerate() {
        syllable_list.push(
            GallCircle{
                character: letter,
                repeat: false,
                vowel: None,
                loc:
                    gall_struct::GallOrd { 
                        ang: Some(0.9 * n as f64), 
                        dist: 200.0, 
                        center: origin, 
                        parent: None
                    },
                radius: 30.0,
                decorators: Vec::new(),
            }
        )
    }
    syllable_list
}
//below is python
//self.theta  = math.acos((Wrd.inner_rad**2 - dist**2 - self.outer_rad**2)/(2*dist*self.outer_rad))
fn theta(letter_distance:f64, letter_radius:f64,big_radius:f64) -> f64 {
    let theta = ((big_radius.powf(2.0) - letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*letter_radius)).acos();
    if theta == std::f64::NAN {
        0.0 //could do math error?
    } else {
        theta
    }
}
//below is python
//math.acos((Wrd.inner_rad**2 + dist**2 - self.outer_rad**2)/(2*dist*Wrd.inner_rad))
fn thi(letter_distance:f64, letter_radius:f64,big_radius:f64) -> f64 {
    let thi = ((big_radius.powf(2.0) + letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*big_radius)).acos();
    if thi == std::f64::NAN {
        0.0 //could do math error?
    } else {
        thi
    }
}

fn render_skele_path(skeleton_letters:Vec<gall_struct::GallCircle>, svg_doc:Document, loc: &mut gall_struct::GallOrd) -> SVG {
    if skeleton_letters.len() == 0 {
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("cx", loc.center.0)
            .set("cy", loc.center.1)
            .set("r", 200);
        svg_doc.add(circle)
    } else {
        let mut init_angle = 0.0;
        let mut thi_letter = thi(skeleton_letters[0].loc.dist,skeleton_letters[0].radius,200.0);
        if thi_letter > 0.0 { //skeleton_letters[0].character == '_'
            init_angle -= thi_letter;
        }
        loc.set_ang_d((init_angle,200.0));
        let continuum_pt = loc.svg_ord();
        let mut first = true;
        
        let mut angle = match skeleton_letters[0].loc.ang {
            Some(ang) => ang,
            None => 0.2
        };
        loc.set_ang( angle - thi_letter);
        let mut letter_arc_start = loc.svg_ord();
        loc.c_clockwise(2.0 * thi_letter);
        let mut letter_arc_finish = loc.svg_ord();
        
        //actually fill in data
        let mut skele_data = Data::new()
            .move_to(continuum_pt)
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            .elliptical_arc_to((200,200, 0,0,0,letter_arc_start.0,letter_arc_start.1))
            .elliptical_arc_to((skeleton_letters[0].radius, skeleton_letters[0].radius,0,0,1,letter_arc_finish.0,letter_arc_finish.1));

        for letter in skeleton_letters {
            if first {
                first = false;
                continue;
            }
            thi_letter = thi(letter.loc.dist,letter.radius,200.0);
            angle = match letter.loc.ang {
                Some(ang) => ang,
                None => 0.2
            };
            loc.set_ang( angle - thi_letter);
            letter_arc_start = loc.svg_ord();
            loc.c_clockwise(2.0 * thi_letter);
            letter_arc_finish = loc.svg_ord();
            skele_data = skele_data
                .elliptical_arc_to((200,200, 0,0,0,letter_arc_start.0,letter_arc_start.1))
                .elliptical_arc_to((letter.radius, letter.radius,0,0,1,letter_arc_finish.0,letter_arc_finish.1));
        }

        let mut final_sweep = 1;
        if -angle - thi_letter + init_angle < -std::f64::consts::FRAC_PI_2 {
            final_sweep = 0
        }
        let closed_loop = skele_data
            .elliptical_arc_to((200.0,200.0,0,final_sweep,0,continuum_pt.0,continuum_pt.1))
            .close();

        //attach path to svg
        let path = Path::new()
            .set("fill", "green")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", closed_loop);
        svg_doc.add(path)
    }
}

fn render_lttr_path(syllables:Vec<gall_struct::GallCircle>, svg_doc:Document) -> SVG {
    if false {//syllables.len() == 0 {
        svg_doc
    } else {
        let mut drawn = svg_doc;
        for letter in syllables {
            let circle = Circle::new()
                .set("fill", "blue")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("cx", letter.loc.svg_x())
                .set("cy", letter.loc.svg_y())
                .set("r", 30);
            drawn = drawn.add(circle);
        }
        drawn
    }
}

fn main() {
    let width = 512.0;
    let height = 512.0;
    let args: Vec<String> = env::args().collect();
    let raw_text = &args[1];
    let seed_text = &args[2];
    let _seed = seed_text.to_owned().into_bytes();
    let mut origin = gall_struct::GallOrd{
        ang: None,
        dist: 0.0,
        center: (width/2.0,height/2.0),
        parent: None,
    };
    println!("Start");
    let all_letters = text_to_gall(raw_text.to_owned(), origin.center);
    let skele_ltrs = Vec::new();
    let oth_ltrs = all_letters;//Vec::new();

    let document = Document::new()
        .set("viewBox", (0, 0, width, height));
    let skeleton = render_skele_path(skele_ltrs, document, &mut origin);
    let all_letters = render_lttr_path(oth_ltrs, skeleton);
    println!("Saving");
    svg::save(raw_text.to_owned() + ".svg", &all_letters).unwrap();
}
