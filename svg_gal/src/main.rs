use std::env;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

mod gall_struct;

fn render_skele_path(skeleton_letters:Vec<gall_struct::GallCircle>, svg_doc:Document, &WIDTH: &i32, &HEIGHT: &i32) -> SVG {
    if skeleton_letters.len() == 0 {
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("cx", WIDTH/2)
            .set("cy", HEIGHT/2)
            .set("r", 200);
        svg_doc.add(circle)
    } else {
        let skele_data = Data::new()
            .move_to((10,10))
            // x radius, y radius, rotation, large arc, sweep direction
            .elliptical_arc_by((30,30, 0,0,0,250,250))
            .line_by((50, 0))
            .line_by((0, -50))
            .close();
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", skele_data);
        svg_doc.add(path)
    }
}

fn render_lttr_path(syllables:Vec<gall_struct::GallCircle>, svg_doc:Document, &WIDTH: &i32, &HEIGHT: &i32) -> SVG {
    if syllables.len() == 0 {
        let skele_data = Data::new()
            .move_to((10,10))
            // x radius, y radius, rotation, large arc, sweep direction
            .elliptical_arc_by((30,30, 0,0,0,250,250))
            .line_by((50, 0))
            .line_by((0, -50))
            .close();
        let path2 = Path::new()
            .set("fill", "blue")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", skele_data);
        svg_doc.add(path2)
    } else {
        print!("something");
        svg_doc
    }
}

fn main() {
    let WIDTH = 512;
    let HEIGHT = 512;
    let args: Vec<String> = env::args().collect();
    let raw_text = &args[1];
    let seed_text = &args[2];
    let _seed = seed_text.to_owned().into_bytes();
    println!("Start");

    let skele_ltrs = Vec::new();
    let oth_ltrs = Vec::new();

    let skele_data = Data::new()
        // x radius, y radius, rotation, large arc, sweep direction
        .elliptical_arc_by((10,10, 0,0,0,250,250))
        .line_by((150, 0))
        .line_by((0, -50))
        .close();
    let path3 = Path::new()
        .set("fill", "blue")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", skele_data);

    let document = Document::new()
        .set("viewBox", (0, 0, WIDTH, HEIGHT));
    let skeleton = render_skele_path(skele_ltrs, document,&WIDTH, &HEIGHT);
    let all_letters = render_lttr_path(oth_ltrs, skeleton, &WIDTH, &HEIGHT);
    println!("Saving");
    svg::save(raw_text.to_owned() + ".svg", &all_letters).unwrap();
}
