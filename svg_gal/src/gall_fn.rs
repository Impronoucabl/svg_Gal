use std::f64::consts::{PI, TAU};

#[derive(PartialEq,Default)]
pub enum  LetterType {
    Digit,
    StaticVowel,
    BStem,
    JStem,
    TStem,
    ZStem,
    AVowel,
    OVowel,
    #[default]
    Punctuation, //more for error case than anything
}

pub fn stem_dist(stem:&LetterType, dist:f64) -> f64 {
    match stem {
        LetterType::BStem => dist - 20.0,
        LetterType::JStem => dist - 35.0,
        LetterType::TStem => dist,
        LetterType::ZStem => dist,
        LetterType::StaticVowel => dist,
        LetterType::OVowel => dist - 25.0,
        LetterType::AVowel => dist + 25.0,
        LetterType::Digit => dist - 35.0,
        _ => dist
    }
}

pub fn stem_size(stem:&LetterType) -> f64 {
    match stem {
        LetterType::AVowel => 15.0,
        LetterType::StaticVowel => 15.0,
        LetterType::OVowel => 15.0,
        LetterType::Punctuation => 0.0,
        _ => 30.0
    }
}

pub fn default_layouts(word_length:usize) -> (f64,f64,f64) {
    match word_length {
        //word_radius, word_angle, word_dist
        0|1 => (200.0,0.0,0.0),
        2 => (80.0,PI,120.0),
        phrase_len => (
            50.0,
            TAU/(phrase_len as f64),
            150.0,
        ),
    }
}

pub fn string_parse(raw_word:String) -> String {
    let mut word = raw_word.to_lowercase();
    word = replace_two_char(word);
    word = replace_repeat_char(word);
    //add more fancy parsing bits here
    word
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

pub fn stem_lookup(letter:&char) -> (LetterType, bool) {
    let stem = match letter {
        'A'|'a'|'\u{ea01}'                                      => LetterType::AVowel,
        'E'|'I'|'U'|'e'|'i'|'u'|'\u{ea05}'|'\u{ea09}'|'\u{ea15}'=> LetterType::StaticVowel,
        'O'|'o'|'\u{ea0f}'                                      => LetterType::OVowel,
        '█'|'B'|'D'|'F'|'G'|'H'|'b'|'d'|'f'|'g'|'h'             => LetterType::BStem,
        'C'|'J'|'K'|'L'|'M'|'N'|'P'|'c'|'j'|'k'|'l'|'m'|'n'|'p' => LetterType::JStem,
        'R'|'S'|'T'|'V'|'W'|'r'|'s'|'t'|'v'|'w'                 => LetterType::TStem,
        'Q'|'X'|'Y'|'Z'|'q'|'x'|'y'|'z'                         => LetterType::ZStem, 
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'                 => LetterType::Digit, // TODO
        '\u{e100}'..='\u{e2ff}'                                 => LetterType::BStem, // CH & ND,
        '\u{e300}'..='\u{e3ff}'                                 => LetterType::JStem, // PH,
        '\u{e400}'..='\u{e6ff}'                                 => LetterType::TStem, // WH, SH, NT
        '\u{e700}'..='\u{e9ff}'|'\u{e000}'..='\u{e0ff}'         => LetterType::ZStem, // GH, NG, QU, TH
        '\u{ea02}'|'\u{ea04}'|'\u{ea06}'|'\u{ea07}'|'\u{ea08}'  => LetterType::BStem, // repeat BStems
        '\u{ea03}'|'\u{ea0a}'|'\u{ea0b}'|'\u{ea0c}'|'\u{ea0d}'|'\u{ea0e}'|'\u{ea10}' => LetterType::JStem, // repeat JStems
        '\u{ea12}'|'\u{ea13}'|'\u{ea14}'|'\u{ea16}'|'\u{ea17}'  => LetterType::TStem, // repeat TStems
        '\u{ea11}'|'\u{ea18}'|'\u{ea19}'|'\u{ea20}'             => LetterType::ZStem, // repeat ZStems
        _ => LetterType::Punctuation, //TODO
    };
    let repeat = match letter {
        '\u{ea01}'..='\u{ea20}' => true,
        _ => false
    };
    (stem,repeat)
}

pub fn decor_lookup(letter:&char) -> (Option<bool>,usize) {
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

pub fn _theta(letter_distance:f64, letter_radius:f64,big_radius:f64) -> f64 {
    let theta = ((big_radius.powf(2.0) - letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*letter_radius)).acos();
    if theta == std::f64::NAN {
        0.0 //could do math error?
    } else {
        theta
    }
}