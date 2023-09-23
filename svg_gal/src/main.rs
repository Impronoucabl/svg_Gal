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

fn render_lttr_path(syllables:Vec<gall_struct::GallCircle>, svg_doc:Document, loc: &gall_struct::GallOrd) -> SVG {
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
    let WIDTH = 512.0;
    let HEIGHT = 512.0;
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
        center: (WIDTH/2.0,HEIGHT/2.0),
        parent: None,
    };
    let document = Document::new()
        .set("viewBox", (0, 0, WIDTH, HEIGHT));
    let skeleton = render_skele_path(skele_ltrs, document, &mut origin);
    let all_letters = render_lttr_path(oth_ltrs, skeleton, &origin);
    println!("Saving");
    svg::save(raw_text.to_owned() + ".svg", &all_letters).unwrap();
}
