//use std::collections::VecDeque;
use std::env;

use svg::Document;

mod gall_fn;
mod gall_struct;
mod gall_ord;
mod gall_phrase;
mod gall_errors;
mod rendering;
mod gall_letter_type;
use gall_ord::GallLoc;
use gall_phrase::GallPhrase;
use gall_struct::GallWord;

fn main() {
    static WIDTH:f64 = 512.0;
    static HEIGHT:f64 = 512.0;
    //maybe lazy static it in
    let ORIGIN: GallLoc = GallLoc::new(
        None,
        0.0,
        (WIDTH/2.0,HEIGHT/2.0),
    );
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
    let (word_radius, word_angle, word_dist) = gall_fn::default_layouts(word_list.len());
    println!("Generating...");
    let mut sentence = GallPhrase{words:Vec::new(),dash_pairs:Vec::new(),radius:WIDTH/2.0 - 6.0, thickness: 6.0};
    for (num,words) in word_list.into_iter().enumerate() {
        let word_loc = GallLoc::new(
            Some(word_angle * num as f64), 
            word_dist, 
            ORIGIN.get_center(), 
        );
        //parse letters more?
        let word_circle = GallWord::new(
            words.to_owned(),
            word_loc,
            word_radius,
            3.0,
            Vec::new(),
        );
        sentence.words.push(word_circle);
    }
    println!("Deciding layout...");
    
    for word in &mut sentence.words {
        word.distribute();
        word.update_kids();
    }
    sentence.dock_words();
    //TODO: Layout stuff here?
    sentence.dash_pair_loop(); //create dash pair paths here?
    
    println!("Rendering...");
    let mut drawn = Document::new().set("viewBox", (0, 0, WIDTH, HEIGHT));   
    drawn = sentence.render(drawn, ORIGIN);    
    println!("Saving under {}", filename);
    match svg::save(filename + ".svg", &drawn) {
        Ok(_) => println!("Done!"),
        Err(message) => println!("{}", message),
    }
}
