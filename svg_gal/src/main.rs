use std::{cell::Cell, env, rc::Rc};

use svg::node::element::{Circle,Rectangle, SVG};
use svg::Document;

use crate::gall_config::Config;
use crate::gall_fn::default_layouts;
use crate::gall_loc::{GallLoc, Location};
use crate::gall_sentence::GallSentence;
use crate::gall_word::GallWord;
use crate::render::Renderable;

mod gall_config;
mod gall_fn;
mod gall_errors;
mod gall_ang;
mod gall_ord;
mod gall_loc;
mod gall_node;
mod gall_circle;
mod gall_stem;
mod gall_vowel;
mod gall_tainer;
mod gall_word;
mod gall_sentence;
mod render;

fn create_svg() -> SVG {
    let drawn = Document::new().set("viewBox", (0, 0, Config::WIDTH, Config::HEIGHT));   
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", Config::WIDTH)
        .set("height", Config::HEIGHT)
        .set("fill", Config::BG_COLOUR())
        .set("stroke", "none");
    drawn.add(background)
}

fn main() {
    let ORIGIN = Rc::new(Cell::new((Config::WIDTH/2.0,Config::HEIGHT/2.0)));
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
    let mut sent = GallSentence::new(
        GallLoc::new(
            0.0,
            0.0,
            ORIGIN.clone(),
        ),
        Config::SENT_RADIUS,
        Config::SENT_THICK,
    );
    println!("Generating...");
    sent.generate(word_list);
    println!("Rendering...");
    let mut drawn = create_svg();
    drawn = sent.render(drawn);  
    println!("Saving under {}", filename);
    match svg::save(filename + ".svg", &drawn) {
        Ok(_) => println!("Done!"),
        Err(message) => println!("{}", message),
    }
}