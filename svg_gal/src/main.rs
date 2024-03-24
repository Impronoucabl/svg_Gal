use std::{cell::Cell, env, rc::Rc};

use crate::gall_circle::Circle;
use crate::gall_config::Config;
use crate::gall_loc::{GallLoc, Location};
use crate::gall_sentence::GallSentence;

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
mod gall_pair;
mod pairing;
mod render;

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
    filename += ".svg";
    println!("Generating...");
    let mut sent = GallSentence::new(
        GallLoc::new(
            0.0,
            0.0,
            ORIGIN.clone(),
        ),
        Config::SENT_RADIUS,
        Config::SENT_THICK,
    );
    sent.generate(word_list);
    println!("Organizing...");
    sent.basic();
    let (ext_rad, ext_cent) = (sent.get_radius(),sent.get_center());
    let node_vec = sent.collect_nodes();
    let (pairs, spares) = pairing::generate_pairs(node_vec);
    let lines = pairing::extend_spares(spares, ext_rad,ext_cent);
    println!("Rendering...");
    let (mut drawn, post_render) = render::render_init(pairs, lines);
    drawn = render::render_start(sent, drawn);
    drawn = render::render_post(post_render, drawn);
    println!("Saving under {}", filename);
    match svg::save(filename, &drawn) {
        Ok(_) => println!("Done!"),
        Err(message) => println!("{}", message),
    }
}