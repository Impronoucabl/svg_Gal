use std::cell::OnceCell;
use std::env;
use std::f64::consts::TAU;

use gall_ang::GallAng;
use gall_fn::LetterMark;
use gall_line::GallLine;
use gall_loc::Location;
use gall_stem::StemType;
use gall_tainer::GallTainer;
use gall_vowel::GallVowel;
use svg::node::element::Rectangle;
use svg::{node::element::Circle, Document};

use crate::gall_config::Config;
use crate::gall_fn::default_layouts;
use crate::gall_loc::GallLoc;
use crate::gall_parse::PreWord;
use crate::gall_stem::Stem;
use crate::gall_word::GallWord;
//use crate::render::Renderable;

mod gall_config;
mod gall_fn;
mod gall_parse;
mod gall_char;
mod gall_errors;
mod gall_ang;
mod gall_ord;
mod gall_loc;
// mod gall_node;
mod gall_circle;
mod gall_stem;
mod gall_vowel;
mod gall_tainer;
mod gall_word;
mod gall_line;
//mod render;

fn generate_stem_loc(stem:StemType, angle:f64, w_radius:f64, w_thick:f64, center_ref:OnceCell<(f64,f64)>) -> (f64, GallLoc) {
    let (l_dist,l_thick) = match stem {
        StemType::J => (w_radius*(0.7 - Config::LETTER_FRAC_OF_WRD),w_thick),
        StemType::B => (w_radius*(1.2 - Config::LETTER_FRAC_OF_WRD),w_thick),
        StemType::S => (w_radius + w_thick, w_thick),
        StemType::Z => (w_radius, w_thick),
    };
    (
        l_thick,
        GallLoc::new(angle,l_dist,center_ref).unwrap()
    )
    
}

fn convert_parse_to_obj<'a>(phrase:Vec<PreWord>, word_len:usize, origin:(f64,f64)) -> Option<GallLine<'a>> {
    let ORIGIN = OnceCell::new();
        ORIGIN.set(origin);
        let clause_location = GallLoc::new(
            0.0,
            0.0,
            ORIGIN,
        )?;
    let mut word_vec = Vec::with_capacity(word_len);
    for (w_num, pre_word) in phrase.into_iter().enumerate() {
        let w_len = pre_word.len();
        let (w_radius, w_thick, word_ang, dist) = default_layouts(word_len, w_num);
        let (word_radius, word_thickness) = (OnceCell::new(),OnceCell::new());
        _ = word_radius.set(w_radius);
        _ = word_thickness.set(w_thick);
        let word_location = GallLoc::new(
            w_num as f64*word_ang,//TODO: multiple words
            dist,//TODO: multiple words
            clause_location.pos_ref(),
        )?;
        let mut syl_vec = Vec::with_capacity(pre_word.pre_vec.len());
        for (s_num, syl) in pre_word.pre_vec.into_iter().enumerate() {
            let s_angle = (s_num/w_len) as f64*TAU;
            let mut stem_vec = Vec::new();
            let mut vowel_vec = Vec::new();
            let mut mark_vec = Vec::new();
            for cha in syl.char_vec {
                match cha.letter {
                    LetterMark::Stem(stem) => {
                        let (l_thick , letter_location) = generate_stem_loc(stem, s_angle, w_radius, w_thick, word_location.get_center());
                        let new_stem = Stem::new(
                            letter_location, 
                            w_radius*Config::LETTER_FRAC_OF_WRD, 
                            l_thick, 
                            stem, 
                            word_radius.get(), word_thickness.get()
                        )?;
                        stem_vec.push(new_stem)
                    },
                    LetterMark::GallVowel(_) => {
                        // let new_vowel = GallVowel::new()?;
                        // vowel_vec.push(new_vowel)
                    },
                    LetterMark::Digit(_) => {},
                    LetterMark::GallMark => {
                        let new_mark = true;
                        mark_vec.push(new_mark);
                    },
                }
            }
            let stem_t = OnceCell::new();
            if let Some(stm) = syl.stem_type {
                _ = stem_t.set(stm);
            }
            let con = GallTainer{
                ang: GallAng::new(Some(s_angle)),
                stem_type: stem_t,
                stem: stem_vec,
                vowel: vowel_vec,
                //dot: mark_vec,
                buffer: ((OnceCell::new(), OnceCell::new(),OnceCell::new())),
            };
            syl_vec.push(con);
        }
        let word = GallWord {
            loc: word_location,
            tainer_vec: syl_vec,
            radius: word_radius,
            thickness: word_thickness
        };
        word_vec.push(word)
    }
    Some(GallLine{
        words: word_vec
    })
}

fn main() {
    // let ORIGIN = OnceCell::new();
    // ORIGIN.set((Config::WIDTH/2.0,Config::HEIGHT/2.0));
    const ORIGIN: (f64,f64) = (Config::WIDTH/2.0,Config::HEIGHT/2.0);
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
    //let mut word_vec = Vec::with_capacity(word_len);
    let mut parse_vec = Vec::with_capacity(word_len);
    for (text, guess) in word_list {
        let proto_word = PreWord::new(text, guess);
        parse_vec.push(proto_word);
    }
    let gallifreyan = convert_parse_to_obj(parse_vec, word_len,ORIGIN);
    // println!("Generating...");
    // for (num,words) in word_list.into_iter().enumerate() {
    //     let (w_radius, w_thick, word_ang, dist) = default_layouts(word_len, num);
    //     //create word struct
    //     let loc = GallLoc::new(
    //         (num as f64) * word_ang,
    //         dist,
    //         ORIGIN.clone()
    //     );
    //     let new_word = GallWord::new(words.0,words.1, loc.unwrap(), w_radius, w_thick);
    //     word_vec.push(new_word);
    // }
    // println!("Rendering...");
    // let mut drawn = Document::new().set("viewBox", (0, 0, Config::WIDTH, Config::HEIGHT));   
    // let background = Rectangle::new()
    //     .set("x", 0)
    //     .set("y", 0)
    //     .set("width", Config::WIDTH)
    //     .set("height", Config::HEIGHT)
    //     .set("fill", Config::BG_COLOUR())
    //     .set("stroke", "none");
    // drawn = drawn.add(background);
    // for word in word_vec.into_iter() {
    //     drawn = word.render(drawn);
    // }
    // let circle = Circle::new()
    //             .set("fill", "none")
    //             .set("stroke", Config::SENT_COLOUR())
    //             .set("stroke-width", Config::SENT_THICK)
    //             .set("cx", Config::WIDTH/2.0)
    //             .set("cy", Config::HEIGHT/2.0)
    //             .set("r", 1020);
    // drawn = drawn.add(circle);   
    // println!("Saving under {}", filename);
    // match svg::save(filename + ".svg", &drawn) {
    //     Ok(_) => println!("Done!"),
    //     Err(message) => println!("{}", message),
    // }
}