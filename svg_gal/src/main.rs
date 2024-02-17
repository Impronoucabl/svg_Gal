use std::env;

use svg::{node::element::Circle, Document};

mod gall_fn;

fn main() {
    static WIDTH:f64 = 512.0;
    static HEIGHT:f64 = 512.0;
    //const ORIGIN: Rc<(f64,f64)> = Rc::new((0.0,0.0));
    println!("Initialising...");
    let args = env::args();
    let mut word_list = Vec::new();
    let mut filename:String = "".to_string();
    let mut word_len: usize = 0;
    for raw_word in args {
        if filename.len() == 0 {
            filename += "SVGs\\"; //Save to SVGs folder
            continue;//first argument is usually runpath
        }
        filename += &raw_word;
        word_len += 1;
        word_list.push(gall_fn::string_parse(raw_word));
    }
    println!("Generating...");
    println!("Rendering...");
    let mut drawn = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));   
    // do actual rendering TODO
    let circle = Circle::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("cx", 0.0)
                .set("cy", 0.0)
                .set("r", 210.0);
    drawn = drawn.add(circle);
    //drawn = sentence.render(drawn, ORIGIN);    
    println!("Saving under {}", filename);
    match svg::save(filename + ".svg", &drawn) {
        Ok(_) => println!("Done!"),
        Err(message) => println!("{}", message),
    }
}