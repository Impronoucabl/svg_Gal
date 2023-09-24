use std::env;

use svg::Document;
use svg::node::element::{Path, Circle, SVG};
use svg::node::element::path::Data;

mod gall_struct;
use gall_struct::{GallCircle, GallOrd, LetterType};

fn text_to_gall<'a>(text: String, origin: (f64,f64)) -> Vec<GallCircle<'a>> {
    let mut syllable_list = Vec::new();
    let letter_sep_ang = 2.0 * std::f64::consts::PI/(text.len() as f64);
    for (n, letter) in text.chars().enumerate() {
        let stem = match letter {
            //char => letter type, decorator type, # decor, 
            'A'|'O'|'a'|'o'                                 => LetterType::FloatingVowel,
            'E'|'I'|'U'|'e'|'i'|'u'                         => LetterType::StaticVowel,
            '█'|'B'|'D'|'F'|'G'|'H'|'b'|'d'|'f'|'g'|'h'     => LetterType::BStem, // Also CH & ND
            'C'|'J'|'K'|'L'|'N'|'P'|'c'|'j'|'k'|'l'|'n'|'p' => LetterType::JStem, // Also PH
            'R'|'S'|'T'|'V'|'W'|'r'|'s'|'t'|'v'|'w'         => LetterType::TStem, // Also WH, SH, NT
            'Q'|'X'|'Y'|'Z'|'q'|'x'|'y'|'z'                 => LetterType::ZStem, // Also GH, NG, QU, TH
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'         => LetterType::Digit, // TODO
            _ => LetterType::Punctuation, //TODO
        };
        let dot = match letter {
            'C'|'D'|'K'|'L'|'Q'|'R'|'Y'|'Z'|'c'|'d'|'k'|'l'|'q'|'r'|'y'|'z' => Some(true),
            'E'|'F'|'G'|'H'|'I'|'M'|'N'|'P'|'S'|'V'|'W'|'X'|'e'|'f'|'g'|'h'|'i'|'m'|'n'|'p'|'s'|'v'|'w'|'x' => Some(false),
            'B'|'J'|'T'|'b'|'j'|'t' => None,
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => None, //TODO
            _ => None
        };
        let mut decor_num = 0;
        if dot.is_some()  {
            decor_num = match letter {
                'E'|'G'|'I'|'N'|'U'|'V'|'e'|'g'|'i'|'n'|'u'|'v'  => 1,
                'H'|'K'|'P'|'W'|'X'|'Y'|'h'|'k'|'p'|'w'|'x'|'y' => 2,
                'D'|'F'|'L'|'M'|'R'|'S'|'Z'|'d'|'f'|'l'|'m'|'r'|'s'|'z' => 3,
                'C'|'Q'|'c'|'q' => 4,
                _ => 0
            }
        }
        let letter_loc = GallOrd { 
            ang: Some(letter_sep_ang * n as f64), 
            dist: 180.0, 
            center: origin, 
            parent: None
        };
        let mut decor_list = Vec::new();
        while decor_num > 0 {
            let decor_type = match dot {
                Some(boolean) => boolean,
                None => false, //TODO fix
            };
            let dec = gall_struct::Decor{
                loc: GallOrd{
                    ang:Some(0.2 * decor_num as f64),
                    dist:30.0,
                    center: origin,
                    parent: None //TODO fix
                },
                dot: decor_type,
            };
            decor_list.push(dec);
            decor_num -= 1;
        } 
        syllable_list.push(
            GallCircle{
                character: letter,
                stem:stem,
                repeat: false,
                vowel: None, //for attached vowels only
                loc:letter_loc,                    
                radius: 30.0,
                decorators: decor_list,
            }
        )
    }
    syllable_list
}

fn render_skele_path(skeleton_letters:Vec<gall_struct::GallCircle>, svg_doc:Document, loc: &mut gall_struct::GallOrd) -> SVG {
    if skeleton_letters.len() == 0 {
        let circle = Circle::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("cx", loc.center.0)
            .set("cy", loc.center.1)
            .set("r", 200);
        svg_doc.add(circle)
    } else {
        let mut init_angle = 0.0;
        let mut thi_letter = gall_struct::thi(skeleton_letters[0].loc.dist,skeleton_letters[0].radius,200.0);
        if thi_letter > 0.0 { //skeleton_letters[0].character == '_'
            init_angle -= thi_letter;
        }
        loc.set_ang_d((init_angle,200.0));
        let continuum_pt = loc.svg_ord();
        let mut first = true;
        let mut b_divot_flag = 0;
        
        let mut angle = match skeleton_letters[0].loc.ang {
            Some(ang) => ang,
            None => 0.0
        };
        if skeleton_letters[0].stem == gall_struct::LetterType::BStem {
            b_divot_flag = 1;
        }
        loc.set_ang( angle - thi_letter);
        let mut letter_arc_start = loc.svg_ord();
        loc.c_clockwise(2.0 * thi_letter);
        let mut letter_arc_finish = loc.svg_ord();
        
        //actually fill in data
        let mut skele_data = Data::new()
            .move_to(continuum_pt)
            // x radius, y radius, rotation, large arc, sweep direction, end x, end y
            .elliptical_arc_to((200,200, 0,0,0,letter_arc_start.0,letter_arc_start.1))
            .elliptical_arc_to((skeleton_letters[0].radius, skeleton_letters[0].radius,0,b_divot_flag,1,letter_arc_finish.0,letter_arc_finish.1));

        for letter in skeleton_letters {
            if first {
                first = false;
                continue;
            }
            if letter.stem == gall_struct::LetterType::BStem {
                b_divot_flag = 1
            } else {
                b_divot_flag = 0
            }
            thi_letter = gall_struct::thi(letter.loc.dist,letter.radius,200.0);
            angle = match letter.loc.ang {
                Some(ang) => ang,
                None => 0.2
            };
            loc.set_ang( angle - thi_letter);
            letter_arc_start = loc.svg_ord();
            loc.c_clockwise(2.0 * thi_letter);
            letter_arc_finish = loc.svg_ord();
            skele_data = skele_data
                .elliptical_arc_to((200,200, 0,0,0,letter_arc_start.0,letter_arc_start.1))
                .elliptical_arc_to((letter.radius, letter.radius,0,b_divot_flag,1,letter_arc_finish.0,letter_arc_finish.1));
        }

        let mut final_sweep = 1;
        if -angle - thi_letter + init_angle < -std::f64::consts::FRAC_PI_2 {
            final_sweep = 0
        }
        let closed_loop = skele_data
            .elliptical_arc_to((200.0,200.0,0,final_sweep,0,continuum_pt.0,continuum_pt.1))
            .close();

        //attach path to svg
        let path = Path::new()
            .set("fill", "green")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", closed_loop);
        svg_doc.add(path)
    }
}

fn render_lttr_path(syllables:Vec<gall_struct::GallCircle>, svg_doc:Document) -> SVG {
    if false {//syllables.len() == 0 {
        svg_doc
    } else {
        let mut drawn = svg_doc;
        for letter in syllables {
            let circle = Circle::new()
                .set("fill", "blue")
                .set("stroke", "black")
                .set("stroke-width", 3)
                .set("cx", letter.loc.svg_x())
                .set("cy", letter.loc.svg_y())
                .set("r", 30);
            drawn = drawn.add(circle);
        }
        drawn
    }
}

fn main() {
    let width = 512.0;
    let height = 512.0;
    let args: Vec<String> = env::args().collect();
    let raw_text = &args[1];
    let seed_text = &args[2];
    let _seed = seed_text.to_owned().into_bytes();
    let mut origin = GallOrd{
        ang: None,
        dist: 0.0,
        center: (width/2.0,height/2.0),
        parent: None,
    };
    println!("Start");
    let all_letters = text_to_gall(raw_text.to_owned(), origin.center);
    let skele_ltrs = all_letters;//Vec::new();
    let oth_ltrs = Vec::new();

    let document = Document::new()
        .set("viewBox", (0, 0, width, height));
    let skeleton = render_skele_path(skele_ltrs, document, &mut origin);
    let all_letters = render_lttr_path(oth_ltrs, skeleton);
    println!("Saving");
    svg::save(raw_text.to_owned() + ".svg", &all_letters).unwrap();
}
