use crate::gall_struct::LetterType;

pub fn stem_dist(stem:&LetterType, dist:f64) -> f64 {
    match stem {
        LetterType::BStem => dist - 25.0,
        LetterType::JStem => dist - 35.0,
        LetterType::TStem => dist,
        LetterType::ZStem => dist,
        LetterType::StaticVowel => dist,
        LetterType::OVowel => dist - 20.0,
        LetterType::AVowel => dist + 20.0,
        LetterType::Digit => dist - 35.0,
        _ => 0.0
    }
}

pub fn stem_size(stem:&LetterType) -> f64 {
    match stem {
        LetterType::AVowel => 15.0,
        LetterType::StaticVowel => 15.0,
        LetterType::OVowel => 15.0,
        _ => 30.0
    }
}

pub fn default_layouts(word_length:usize) -> (f64,f64,f64) {
    match word_length {
        //word_radius, word_angle, word_dist
        0|1 => (200.0,0.0,0.0),
        2 => (80.0,std::f64::consts::PI,120.0),
        phrase_len => (
            50.0,
            std::f64::consts::TAU/(phrase_len as f64),
            150.0,
        ),
    }
}

pub fn string_parse(raw_word:String) -> String {
    let mut word = raw_word.to_lowercase();
    word = replace_double_char(word);
    //add more fancy parsing bits here
    word
}

fn replace_double_char(lowercase_str:String) -> String {
    // \u{f8ff} is last available unicode in private use space.
    let mut word = lowercase_str;
    if word.contains("ch") {
        word = word.replace("ch", &'\u{e100}'.to_string());
    }
    if word.contains("nd") {
        word = word.replace("nd", &'\u{e200}'.to_string());
    }
    if word.contains("ph") {
        word = word.replace("ph", &'\u{e300}'.to_string());
    }
    if word.contains("wh") {
        word = word.replace("wh", &'\u{e400}'.to_string());
    }
    if word.contains("sh") {
        word = word.replace("sh", &'\u{e500}'.to_string());
    }
    if word.contains("nt") {
        word = word.replace("nt", &'\u{e600}'.to_string());
    }
    if word.contains("gh") {
        word = word.replace("gh", &'\u{e700}'.to_string());
    }
    if word.contains("ng") {
        word = word.replace("ng", &'\u{e800}'.to_string());
    }
    if word.contains("qu") {
        word = word.replace("qu", &'\u{e900}'.to_string());
    }
    if word.contains("th") {
        word = word.replace("th", &'\u{e000}'.to_string());
    }
    word
}

pub fn stem_lookup(letter:&char) -> LetterType {
    match letter {
        'A'|'a'                                                 => LetterType::AVowel,
        'E'|'I'|'U'|'e'|'i'|'u'                                 => LetterType::StaticVowel,
        'O'|'o'                                                 => LetterType::OVowel,
        '█'|'B'|'D'|'F'|'G'|'H'|'b'|'d'|'f'|'g'|'h'             => LetterType::BStem,
        'C'|'J'|'K'|'L'|'M'|'N'|'P'|'c'|'j'|'k'|'l'|'m'|'n'|'p' => LetterType::JStem,
        'R'|'S'|'T'|'V'|'W'|'r'|'s'|'t'|'v'|'w'                 => LetterType::TStem,
        'Q'|'X'|'Y'|'Z'|'q'|'x'|'y'|'z'                         => LetterType::ZStem, 
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'                 => LetterType::Digit, // TODO
        '\u{e100}'..='\u{e2ff}'                                 => LetterType::BStem, // CH & ND,
        '\u{e300}'..='\u{e3ff}'                                 => LetterType::JStem, // PH,
        '\u{e400}'..='\u{e6ff}'                                 => LetterType::TStem, // WH, SH, NT
        '\u{e700}'..='\u{e9ff}'|'\u{e000}'..='\u{e0ff}'         => LetterType::ZStem, // GH, NG, QU, TH
        _ => LetterType::Punctuation, //TODO
    }
}

pub fn decor_lookup(letter:&char) -> (Option<bool>,i8) {
    let dot = match letter {
        'C'|'D'|'K'|'L'|'Q'|'R'|'Y'|'Z'|'c'|'d'|'k'|'l'|'q'|'r'|'y'|'z' => Some(true),
        'F'|'G'|'H'|'I'|'M'|'N'|'P'|'S'|'V'|'W'|'X'|'f'|'g'|'h'|'i'|'m'|'n'|'p'|'s'|'v'|'w'|'x' => Some(false),
        'A'|'B'|'E'|'J'|'O'|'T'|'a'|'b'|'e'|'j'|'o'|'t' => None,
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => None, //TODO
        '\u{e000}'..='\u{e0ff}' => None, // TH & variants
        '\u{e800}'..='\u{e9ff}' => Some(false), // QU, NG & variants
        '\u{e100}'..='\u{e7ff}' => Some(true), // Other double letter variants
        _ => None
    };
    let mut decor_num = 0;
    if dot.is_some()  {
        decor_num = match letter {
            'E'|'G'|'I'|'N'|'U'|'V'|'e'|'g'|'i'|'n'|'u'|'v'  => 1,
            'H'|'K'|'P'|'W'|'X'|'Y'|'h'|'k'|'p'|'w'|'x'|'y' => 2,
            'D'|'F'|'L'|'M'|'R'|'S'|'Z'|'d'|'f'|'l'|'m'|'r'|'s'|'z' => 3,
            'C'|'Q'|'c'|'q' => 4,
            '\u{e300}'|'\u{e400}'|'\u{e700}'|'\u{e900}'=> 1, //PH, WH, GH, QU
            '\u{e100}'|'\u{e500}'=> 2, //CH, SH
            '\u{e200}'|'\u{e600}'=> 4, //ND, NT
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