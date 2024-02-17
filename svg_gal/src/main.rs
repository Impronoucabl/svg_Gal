//use std::collections::VecDeque;
use std::{env, f64::consts::TAU, rc::Rc};

use svg::{node::element::Circle, Document};

mod gall_fn;
mod gall_struct;
mod gall_ord;
mod gall_vowel;
mod gall_stem;
mod gall_tainer;
mod gall_word;
//mod gall_phrase;
mod gall_errors;
mod rendering;

use gall_ord::GallLoc;
//use gall_phrase::GallPhrase;
//use gall_struct::GallWord;

use crate::{gall_ord::PositiveDist, gall_word::GallWord2};

fn main() {
    static WIDTH:f64 = 512.0;
    static HEIGHT:f64 = 512.0;
    const ORIGIN: Rc<(f64,f64)> = Rc::new((0.0,0.0));
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
    let word_angle = TAU/(word_len as f64 + 1.0);
    let word_radius = PositiveDist::new(200.0).unwrap();
    let word_thickness = PositiveDist::new(3.0).unwrap();
    //let (word_radius, word_angle, word_dist) = gall_fn::default_layouts(word_list.len());
    println!("Generating...");
    //let mut sentence = GallPhrase{words:Vec::new(),dash_pairs:Vec::new(),radius:WIDTH/2.0 - 6.0, thickness: 6.0};
    for (num,words) in word_list.into_iter().enumerate() {
        let word_loc = GallLoc::new(
            word_angle * num as f64, 
            0.0, //word_dist, 
            gall_ord::CenterOrd::Exisiting(ORIGIN), 
        );
        //parse letters more?
        let mut word_circle = GallWord2::new(
            word_loc,
            word_radius,
            word_thickness,
        );
        // let word_circle = GallWord::new(
        //     words.to_owned(),
        //     word_loc,
        //     word_radius,
        //     3.0,
        //     Vec::new(),
        // );
        //sentence.words.push(word_circle);
        word_circle.populate(words);
    }
    println!("Deciding layout...");
    
    // for word in &mut sentence.words {
        // word.distribute();
        // word.update_kids();
    //}
    //sentence.dock_words();
    //TODO: Layout stuff here?
    //sentence.dash_pair_loop(); //create dash pair paths here?
    
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
