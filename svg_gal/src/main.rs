use std::{cell::Cell, env, rc::Rc};

use svg::{node::element::Circle, Document};

use crate::gall_loc::GallLoc;
use crate::gall_word::GallWord;
use crate::render::renderable;

mod gall_fn;
mod gall_errors;
mod gall_ang;
mod gall_ord;
mod gall_loc;
mod gall_stem;
mod gall_circle;
mod gall_tainer;
mod gall_word;
mod render;

fn main() {
    const WIDTH:f64 = 2048.0;
    const HEIGHT:f64 = 2048.0;
    let ORIGIN = Rc::new(Cell::new((WIDTH/2.0,HEIGHT/2.0)));
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
    //TODO: Generate word ang
    let word_ang = 10.2;    
    let mut word_vec = Vec::with_capacity(word_len);
    println!("Generating...");
    for (num,words) in word_list.into_iter().enumerate() {
        //create word struct
        let loc = GallLoc::new(
            (num as f64) * word_ang,
            match word_len{
                1 => 0.0,
                _ => 600.0,
            },
            ORIGIN.clone()
        );
        word_vec.push(GallWord::new(words.0,words.1, loc));
    }
    println!("Rendering...");
    let mut drawn = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));   
    // do actual rendering TODO
    for word in word_vec.into_iter() {
        drawn = word.render(drawn);
    }
    let circle = Circle::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 20)
                .set("cx", WIDTH/2.0)
                .set("cy", HEIGHT/2.0)
                .set("r", 1020);
    drawn = drawn.add(circle);
    //drawn = sentence.render(drawn, ORIGIN);    
    println!("Saving under {}", filename);
    match svg::save(filename + ".svg", &drawn) {
        Ok(_) => println!("Done!"),
        Err(message) => println!("{}", message),
    }
}