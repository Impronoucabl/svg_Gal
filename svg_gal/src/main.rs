use std::env;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

mod gall_struct;

fn text_to_gall(text:&String) -> Vec<gall_struct::GallCircle> {
    let mut syllable_list = Vec::new();
    for letter in text.chars() {
        syllable_list.push(
            gall_struct::GallCircle{
                character: letter,
                repeat: false,
                vowel: None,
                loc:
                    gall_struct::GallOrd { 
                        ang: Some(0.0), 
                        dist: 10.0, 
                        center: (30.0,30.0), 
                        parent: None 
                    },
                radius: 100.0,
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
            .set("cx", loc.center.0/2.0)
            .set("cy", loc.center.1/2.0)
            .set("r", 200);
        svg_doc.add(circle)
    } else {
        loc.set_ang_d((0.0,loc.center.0));
        if false { //skeleton_letters[0].character == '_'
            loc.ang = Some(-0.1);
        }
        let skele_data = Data::new()
            .move_to(loc.svg_ord())
            // x radius, y radius, rotation, large arc, sweep direction
            .elliptical_arc_by((0,0, 0,0,0,250,250))
            .close();


        //attach path to svg
        let path = Path::new()
            .set("fill", "green")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", skele_data);
        svg_doc.add(path)
    }
}

fn render_lttr_path(syllables:Vec<gall_struct::GallCircle>, svg_doc:Document) -> SVG {
    if syllables.len() == 0 {
        let skele_data = Data::new()
            .move_to((10,10))
            // x radius, y radius, rotation, large arc, sweep direction
            .elliptical_arc_by((30,30, 0,0,0,250,250))
            .line_by((50, 0))
            .line_by((0, -50))
            .close();
        let path2 = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", skele_data);
        svg_doc.add(path2)
    } else {
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
    println!("Start");
    let all_letters = text_to_gall(raw_text);
    let skele_ltrs = all_letters;//Vec::new();
    let oth_ltrs = Vec::new();

    let mut origin = gall_struct::GallOrd{
        ang: None,
        dist: 0.0,
        center: (width/2.0,height/2.0),
        parent: None,
    };
    let document = Document::new()
        .set("viewBox", (0, 0, width, height));
    let skeleton = render_skele_path(skele_ltrs, document, &mut origin);
    let all_letters = render_lttr_path(oth_ltrs, skeleton);
    println!("Saving");
    svg::save(raw_text.to_owned() + ".svg", &all_letters).unwrap();
}
