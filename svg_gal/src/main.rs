use std::env;
use svg::Document;
use svg::node::element::{Path, Circle};
use svg::node::element::path::Data;

fn skele_path(skele_str: &str, WIDTH: &i32) -> Path {
    print!("{}", skele_str);
    let data = Data::new()
        .move_to((WIDTH/2, 10))
        // x radius, y radius, rotation, large arc, sweep direction
        .elliptical_arc_by((30,30, 0,0,0,250,250))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);
    path
}

fn main() {
    let WIDTH = 512;
    let HEIGHT = 512;
    let args: Vec<String> = env::args().collect();
    let raw_text = &args[1];
    let seed_text = &args[2];
    let _seed = seed_text.to_owned().into_bytes();
    println!("Start");
    
    let skele_str = "zoo";
    let skele = skele_path(skele_str, &WIDTH);

    let circle = Circle::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("cx", &WIDTH/2)
        .set("cy", &HEIGHT/2)
        .set("r", 40);

    let document = Document::new()
        .set("viewBox", (0, 0, WIDTH, HEIGHT))
        .add(skele)
        .add(circle);
    println!("Saving");
    svg::save(raw_text.to_owned() + ".svg", &document).unwrap();
}
