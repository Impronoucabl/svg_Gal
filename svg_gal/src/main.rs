use std::env;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_text = &args[1];
    let seed_text = &args[2];
    let _seed = seed_text.to_owned().into_bytes();
    println!("Start");
    let data = Data::new()
        .move_to((-10, -10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (0, 0, 512, 512))
        .add(path);
    println!("Saving");
    svg::save(raw_text.to_owned() + ".svg", &document).unwrap();
}
