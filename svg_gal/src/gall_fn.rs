use std::f64::consts::{PI, TAU};

use crate::gall_stem::StemType;

#[derive(PartialEq)]
pub enum LetterMark {
    Stem(StemType),
    //GallVowel(VowelType),
    GallMark,
    Digit(u32), //TODO: change to i32
}

#[derive(PartialEq,Default)]
pub enum  LetterType {
    Digit,
    EIU,
    BStem,
    JStem,
    SStem,
    ZStem,
    A,
    O1,
    O2,
    #[default]
    Punctuation, //more for error case than anything
}

pub fn default_layouts(phrase_length:usize, num:usize) -> (f64,f64,f64,f64) {
    match phrase_length {
        //word_radius, word_thick, word_angle, word_dist
        0|1 => (650.0,75.0,0.0,0.0),
        2 => (400.0,10.0,PI,200.0),
        len => (
            300.0,
            9.0,
            num as f64*TAU/(len as f64),
            250.0,
        ),//Circular pattern
        //TODO: spiral pattern?
    }
}

pub fn string_parse(raw_word:String) -> (String, usize) {
    let mut word = raw_word.to_lowercase();
    word = replace_two_char(word);
    word = replace_repeat_char(word);
    //add more fancy parsing bits here
    (word, raw_word.len())
}

fn replace_repeat_char(lowercase_str:String) -> String {
    lowercase_str
        .replace("aa", &'\u{ea01}'.to_string())
        .replace("bb", &'\u{ea02}'.to_string())
        .replace("cc", &'\u{ea03}'.to_string())
        .replace("dd", &'\u{ea04}'.to_string())
        .replace("ee", &'\u{ea05}'.to_string())
        .replace("ff", &'\u{ea06}'.to_string())
        .replace("gg", &'\u{ea07}'.to_string())
        .replace("hh", &'\u{ea08}'.to_string())
        .replace("ii", &'\u{ea09}'.to_string())
        .replace("jj", &'\u{ea0a}'.to_string())
        .replace("kk", &'\u{ea0b}'.to_string())
        .replace("ll", &'\u{ea0c}'.to_string())
        .replace("mm", &'\u{ea0d}'.to_string())
        .replace("nn", &'\u{ea0e}'.to_string())
        .replace("oo", &'\u{ea0f}'.to_string())
        .replace("pp", &'\u{ea10}'.to_string())
        .replace("qq", &'\u{ea11}'.to_string())
        .replace("rr", &'\u{ea12}'.to_string())
        .replace("ss", &'\u{ea13}'.to_string())
        .replace("tt", &'\u{ea14}'.to_string())
        .replace("uu", &'\u{ea15}'.to_string())
        .replace("vv", &'\u{ea16}'.to_string())
        .replace("ww", &'\u{ea17}'.to_string())
        .replace("xx", &'\u{ea18}'.to_string())
        .replace("yy", &'\u{ea19}'.to_string())
        .replace("zz", &'\u{ea20}'.to_string())
}

fn replace_two_char(lowercase_str:String) -> String {
    // \u{f8ff} is last available unicode in private use space.
    lowercase_str
        .replace("ch", &'\u{e100}'.to_string())
        .replace("nd", &'\u{e200}'.to_string())
        .replace("ph", &'\u{e300}'.to_string())
        .replace("wh", &'\u{e400}'.to_string())
        .replace("sh", &'\u{e500}'.to_string())
        .replace("nt", &'\u{e600}'.to_string())
        .replace("gh", &'\u{e700}'.to_string())
        .replace("ng", &'\u{e800}'.to_string())
        .replace("qu", &'\u{e900}'.to_string())
        .replace("th", &'\u{e000}'.to_string())
}

pub fn stem_lookup(letter:&char) -> (LetterMark, bool) {
    let stem:LetterMark = match letter {
        // 'A'|'a'|'\u{ea01}'                                      => GallVowel(VowelType::A),
        // 'E'|'e'|'\u{ea05}'                                      => GallVowel(VowelType::E),
        // 'I'|'i'|'\u{ea09}'                                      => GallVowel(VowelType::I),
        // 'O'|'o'|'\u{ea0f}'                                      => GallVowel(VowelType::O2),
        // 'U'|'u'|'\u{ea15}'                                      => GallVowel(VowelType::U),
        '█'|'B'|'D'|'F'|'G'|'H'|'b'|'d'|'f'|'g'|'h'             => LetterMark::Stem(StemType::B),
        'C'|'J'|'K'|'L'|'M'|'N'|'P'|'c'|'j'|'k'|'l'|'m'|'n'|'p' => LetterMark::Stem(StemType::J),
        'R'|'S'|'T'|'V'|'W'|'r'|'s'|'t'|'v'|'w'                 => LetterMark::Stem(StemType::S),
        'Q'|'X'|'Y'|'Z'|'q'|'x'|'y'|'z'                         => LetterMark::Stem(StemType::Z), 
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'                 => LetterMark::Digit(letter.to_digit(10).unwrap()), // TODO
        '\u{e100}'..='\u{e2ff}'                                 => LetterMark::Stem(StemType::B), // CH & ND,
        '\u{e300}'..='\u{e3ff}'                                 => LetterMark::Stem(StemType::J), // PH,
        '\u{e400}'..='\u{e6ff}'                                 => LetterMark::Stem(StemType::S), // WH, SH, NT
        '\u{e700}'..='\u{e9ff}'|'\u{e000}'..='\u{e0ff}'         => LetterMark::Stem(StemType::Z), // GH, NG, QU, TH
        '\u{ea02}'|'\u{ea04}'|'\u{ea06}'|'\u{ea07}'|'\u{ea08}'  => LetterMark::Stem(StemType::B), // repeat BStems
        '\u{ea03}'|'\u{ea0a}'|'\u{ea0b}'|'\u{ea0c}'|'\u{ea0d}'|'\u{ea0e}'|'\u{ea10}' => LetterMark::Stem(StemType::J), // repeat JStems
        '\u{ea12}'|'\u{ea13}'|'\u{ea14}'|'\u{ea16}'|'\u{ea17}'  => LetterMark::Stem(StemType::S), // repeat TStems
        '\u{ea11}'|'\u{ea18}'|'\u{ea19}'|'\u{ea20}'             => LetterMark::Stem(StemType::Z), // repeat ZStems
        _ => LetterMark::GallMark //TODO
    };
    let repeat = match letter {
        '\u{ea01}'..='\u{ea20}' => true,
        _ => false
    };
    (stem,repeat)
}

pub fn dot_lookup(letter:&char) -> (Option<bool>,u8) {
    let dot = match letter {
        'C'|'D'|'K'|'L'|'Q'|'R'|'Y'|'Z'|'c'|'d'|'k'|'l'|'q'|'r'|'y'|'z' => Some(true),
        'F'|'G'|'H'|'I'|'M'|'N'|'P'|'S'|'U'|'V'|'W'|'X'|'f'|'g'|'h'|'i'|'m'|'n'|'p'|'s'|'u'|'v'|'w'|'x' => Some(false),
        'A'|'B'|'E'|'J'|'O'|'T'|'a'|'b'|'e'|'j'|'o'|'t' => None,
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => None, //TODO
        '\u{e000}'..='\u{e0ff}' => None, // TH & variants
        '\u{e800}'..='\u{e9ff}' => Some(false), // QU, NG & variants
        '\u{e100}'..='\u{e7ff}' => Some(true), // Other two letter variants
        '\u{ea03}'|'\u{ea04}'|'\u{ea0b}'|'\u{ea0c}'|'\u{ea11}'|'\u{ea12}'|'\u{ea19}'|'\u{ea20}'=> Some(true), //repeat dots
        '\u{ea06}'|'\u{ea07}'|'\u{ea08}'|'\u{ea09}'|'\u{ea0d}'|'\u{ea0e}'|'\u{ea10}'|'\u{ea13}'|'\u{ea15}'|'\u{ea16}'|'\u{ea17}'|'\u{ea18}'=> Some(false), //repeat dashes
        '\u{ea01}'|'\u{ea02}'|'\u{ea05}'|'\u{ea0f}'|'\u{ea14}' => None, //repeat nones
        _ => None
    };
    let mut decor_num = 0;
    if dot.is_some()  {
        decor_num = match letter {
            'G'|'I'|'N'|'U'|'V'|'g'|'i'|'n'|'u'|'v'  => 1,
            'H'|'K'|'P'|'W'|'X'|'Y'|'h'|'k'|'p'|'w'|'x'|'y' => 2,
            'D'|'F'|'L'|'M'|'R'|'S'|'Z'|'d'|'f'|'l'|'m'|'r'|'s'|'z' => 3,
            'C'|'Q'|'c'|'q' => 4,
            '\u{e300}'|'\u{e400}'|'\u{e700}'|'\u{e900}'=> 1, //PH, WH, GH, QU
            '\u{e100}'|'\u{e500}'=> 2, //CH, SH
            '\u{e200}'|'\u{e600}'|'\u{ea03}'|'\u{ea11}'=> 4, //ND, NT, CC, QQ
            '\u{ea07}'|'\u{ea09}'|'\u{ea0e}'|'\u{ea15}'|'\u{ea16}' => 1,
            '\u{ea08}'|'\u{ea0b}'|'\u{ea10}'|'\u{ea17}'|'\u{ea18}'|'\u{ea19}' => 2,
            '\u{ea04}'|'\u{ea06}'|'\u{ea0c}'|'\u{ea0d}'|'\u{ea12}'|'\u{ea13}'|'\u{ea20}' => 3,
            _ => 0
        }
    };
    (dot, decor_num)
}

pub fn thi(letter_distance:f64, letter_radius:f64,big_radius:f64) -> f64 {
    let thi = ((big_radius.powf(2.0) + letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*big_radius)).acos();
    if thi == std::f64::NAN {
        0.0 //could do math error?
    } else {
        thi
    }
}

pub fn theta(letter_distance:f64, letter_radius:f64,big_radius:f64) -> f64 {
    let theta = ((big_radius.powf(2.0) - letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*letter_radius)).acos();
    if theta == std::f64::NAN {
        0.0 //could do math error?
    } else {
        theta
    }
}