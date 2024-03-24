use std::f64::consts::{PI, TAU};

use crate::gall_errors::{Error, GallError};
use crate::gall_stem::StemType; 
use crate::gall_vowel::VowelType;

#[derive(PartialEq)]
pub enum LetterMark {
    Stem(StemType),
    GallVowel(VowelType),
    GallMark,
    Digit(i8), //TODO: change to i32
}
#[derive(PartialEq)]
pub enum Decor {
    Dot,
    Dash,    
}
pub enum Size {
    Small,
    Med,
    Large,
}
pub struct ProcessedWord {
    pub word: String,
    pub length: usize,
    pub vowels: usize,
    pub size: Size,
    pub a_flag: bool,
    pub z_stem: bool,
    pub s_stem: bool,
    pub neg_digit: Vec<bool>,
} 
pub fn basic_angle(word_list:&Vec<ProcessedWord>, small_weight:i32,avg_weight:i32,large_weight:i32) -> f64 {
    let mut pool = 0;
    for word in word_list {
        match word.size {
            Size::Large => pool += large_weight,
            Size::Med => pool += avg_weight,
            Size::Small => pool += small_weight,
        }
        if word.a_flag {
            pool += 2
        }
        if word.z_stem {
            pool += 3
        }
    }
    TAU/f64::from(pool) 
} 

pub fn default_layouts(phrase_length:usize, num:usize) -> (f64,f64,f64,f64) {
    match phrase_length {
        //word_radius, word_thick, word_angle, word_dist
        0|1 => (650.0,55.0,0.0,0.0),
        2 => (350.0,35.0, num as f64 * PI,450.0),
        3 => (350.0,35.0, num as f64 * TAU/3.0,450.0),
        4 => (325.0,32.5, num as f64 * PI/2.0,525.0),
        len => (
            300.0 - 8.5*len as f64,
            20.0 - 0.1*len as f64,
            num as f64*TAU/(len as f64),
            500.0 + 20.0*len as f64,
        ),//Circular pattern
        //TODO: spiral pattern?
    }
}

pub fn string_parse(raw_word:String) -> ProcessedWord {
    let mut word = raw_word.to_lowercase();
    word = replace_two_char(word);
    word = replace_repeat_3_char(word);
    word = replace_repeat_char(word);
    
    let (length, vowels, a_flag, z_stem, s_stem, neg_digit) = letter_count(&word);
    let size = match length {
        0..=4 => {Size::Small},
        5..=9 => {Size::Med},
        _ => {Size::Large}
    };
    ProcessedWord{
        word,
        length,
        vowels,
        size,
        a_flag,
        z_stem,
        s_stem,
        neg_digit,
    }
}

fn letter_count(word:&String) -> (usize, usize, bool, bool, bool, Vec<bool>) {
    let mut vow_count = 0;
    let mut count = 0;
    let mut a_flag = false;
    let mut z_stem = false;
    let mut s_stem = false;
    let mut num_vec = Vec::new();
    let mut negative_digit = None;
    let mut negative_flag = false;
    for letter in word.chars() {
        count += 1;
        match letter {
            'E'|'e'|'I'|'i'|'O'|'o'|'U'|'u' => vow_count += 1,
            '\u{ea05}'|'\u{ea09}'|'\u{ea0f}'|'\u{ea15}' => vow_count += 2,
            '\u{ea25}'|'\u{ea29}'|'\u{ea2f}'|'\u{ea35}' => vow_count += 3,
            'A'|'a' => {vow_count += 1; a_flag = true},
            '\u{ea01}' => {vow_count += 2; a_flag = true},
            '\u{ea21}' => {vow_count += 3; a_flag = true},
            'R'|'r'|'S'|'s'|'T'|'t'|'W'|'w'|'V'|'v' => s_stem = true,
            '\u{ea12}'..='\u{ea17}'|'\u{ea32}'..='\u{ea37}' => s_stem = true, //repeats
            '\u{e400}'|'\u{e500}'|'\u{e600}' => s_stem = true, //WH, SH, NT
            'Q'|'q'|'X'|'x'|'Y'|'y'|'Z'|'z' => z_stem = true,
            '\u{ea11}'|'\u{ea31}'|'\u{ea18}'..='\u{ea1a}'|'\u{ea38}'..='\u{ea3a}' => z_stem = true, //repeats
            '\u{e000}'|'\u{e700}'|'\u{e800}'|'\u{e900}' => z_stem = true, //TH, GH,NG, QU
            _ => {},
        }
        match letter {
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                if negative_flag {
                    negative_digit = Some(true);
                } else {
                    negative_digit = Some(false);
                }
                count -= 1; //the whole number is 1 character
            },
            '-' => negative_flag = true,
            _ => {
                negative_flag = false;
                if let Some(neg) = negative_digit {
                    if !neg {
                        count += 1;
                    }
                    num_vec.push(neg);
                    negative_digit = None;
                }
            }
        }
        if let Some(neg) = negative_digit {
            if !neg {
                count += 1;
            }
            num_vec.push(neg);
        }
    }
    num_vec.reverse(); //reversed so that it gets popped in the right order
    (count, vow_count, a_flag, z_stem, s_stem, num_vec)
}

fn replace_repeat_3_char(lowercase_str:String) -> String {
    //TODO: Add precursor/middle vowels
    lowercase_str
        .replace("aaa", &'\u{ea21}'.to_string())
        .replace("bbb", &'\u{ea22}'.to_string())
        .replace("ccc", &'\u{ea23}'.to_string())
        .replace("ddd", &'\u{ea24}'.to_string())
        .replace("eee", &'\u{ea25}'.to_string())
        .replace("fff", &'\u{ea26}'.to_string())
        .replace("ggg", &'\u{ea27}'.to_string())
        .replace("hhh", &'\u{ea28}'.to_string())
        .replace("iii", &'\u{ea29}'.to_string())
        .replace("jjj", &'\u{ea2a}'.to_string())
        .replace("kkk", &'\u{ea2b}'.to_string())
        .replace("lll", &'\u{ea2c}'.to_string())
        .replace("mmm", &'\u{ea2d}'.to_string())
        .replace("nnn", &'\u{ea2e}'.to_string())
        .replace("ooo", &'\u{ea2f}'.to_string())
        .replace("ppp", &'\u{ea30}'.to_string())
        .replace("qqq", &'\u{ea31}'.to_string())
        .replace("rrr", &'\u{ea32}'.to_string())
        .replace("sss", &'\u{ea33}'.to_string())
        .replace("ttt", &'\u{ea34}'.to_string())
        .replace("uuu", &'\u{ea35}'.to_string())
        .replace("vvv", &'\u{ea36}'.to_string())
        .replace("www", &'\u{ea37}'.to_string())
        .replace("xxx", &'\u{ea38}'.to_string())
        .replace("yyy", &'\u{ea39}'.to_string())
        .replace("zzz", &'\u{ea3a}'.to_string())
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
        .replace("zz", &'\u{ea1a}'.to_string())
}

fn replace_two_char(lowercase_str:String) -> String {
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

pub fn stem_lookup(letter:&char) -> (LetterMark, u8) {
    let stem:LetterMark = match letter {
        'A'|'a'|'\u{ea01}'|'\u{ea21}'                           => LetterMark::GallVowel(VowelType::A),
        'E'|'e'|'\u{ea05}'|'\u{ea25}'                           => LetterMark::GallVowel(VowelType::E),
        'I'|'i'|'\u{ea09}'|'\u{ea29}'                           => LetterMark::GallVowel(VowelType::I),
        'O'|'o'|'\u{ea0f}'|'\u{ea2f}'                           => LetterMark::GallVowel(VowelType::O2),
        'U'|'u'|'\u{ea15}'|'\u{ea35}'                           => LetterMark::GallVowel(VowelType::U),
        '█'|'B'|'D'|'F'|'G'|'H'|'b'|'d'|'f'|'g'|'h'             => LetterMark::Stem(StemType::B),
        'C'|'J'|'K'|'L'|'M'|'N'|'P'|'c'|'j'|'k'|'l'|'m'|'n'|'p' => LetterMark::Stem(StemType::J),
        'R'|'S'|'T'|'V'|'W'|'r'|'s'|'t'|'v'|'w'                 => LetterMark::Stem(StemType::S),
        'Q'|'X'|'Y'|'Z'|'q'|'x'|'y'|'z'                         => LetterMark::Stem(StemType::Z), 
        '0'                                                     => LetterMark::Digit(0),
        '1'                                                     => LetterMark::Digit(1),
        '2'                                                     => LetterMark::Digit(2),
        '3'                                                     => LetterMark::Digit(3),
        '4'                                                     => LetterMark::Digit(4),
        '5'                                                     => LetterMark::Digit(5),
        '6'                                                     => LetterMark::Digit(6),
        '7'                                                     => LetterMark::Digit(7),
        '8'                                                     => LetterMark::Digit(8),
        '9'                                                     => LetterMark::Digit(9),
        '\u{e100}'..='\u{e2ff}'                                 => LetterMark::Stem(StemType::B), // CH & ND,
        '\u{e300}'..='\u{e3ff}'                                 => LetterMark::Stem(StemType::J), // PH,
        '\u{e400}'..='\u{e6ff}'                                 => LetterMark::Stem(StemType::S), // WH, SH, NT
        '\u{e700}'..='\u{e9ff}'|'\u{e000}'..='\u{e0ff}'         => LetterMark::Stem(StemType::Z), // GH, NG, QU, TH
        '\u{ea02}'|'\u{ea04}'|'\u{ea06}'|'\u{ea07}'|'\u{ea08}'  => LetterMark::Stem(StemType::B), // repeat BStems
        '\u{ea03}'|'\u{ea0a}'..='\u{ea0e}'|'\u{ea10}'           => LetterMark::Stem(StemType::J), // repeat JStems
        '\u{ea12}'|'\u{ea13}'|'\u{ea14}'|'\u{ea16}'|'\u{ea17}'  => LetterMark::Stem(StemType::S), // repeat TStems
        '\u{ea11}'|'\u{ea18}'|'\u{ea19}'|'\u{ea1a}'             => LetterMark::Stem(StemType::Z), // repeat ZStems
        '\u{ea22}'|'\u{ea24}'|'\u{ea26}'|'\u{ea27}'|'\u{ea28}'  => LetterMark::Stem(StemType::B), // triple repeat BStems
        '\u{ea23}'|'\u{ea2a}'..='\u{ea2e}'|'\u{ea30}'           => LetterMark::Stem(StemType::J), // triple repeat JStems
        '\u{ea32}'|'\u{ea33}'|'\u{ea34}'|'\u{ea36}'|'\u{ea37}'  => LetterMark::Stem(StemType::S), // triple repeat TStems
        '\u{ea31}'|'\u{ea38}'|'\u{ea39}'|'\u{ea3a}'             => LetterMark::Stem(StemType::Z), // triple repeat ZStems
        _ => LetterMark::GallMark //TODO
    };
    let repeat = match letter {
        '\u{ea01}'..='\u{ea1a}' => 1,
        '\u{ea21}'..='\u{ea3a}' => 2,
        _ => 0,
    };
    (stem,repeat)
}

pub fn dot_lookup(letter:&char) -> (Option<Decor>,i8) {
    let dot = match letter {
        'C'|'D'|'K'|'L'|'Q'|'R'|'Y'|'Z'|'c'|'d'|'k'|'l'|'q'|'r'|'y'|'z' => Some(Decor::Dot),
        'F'|'G'|'H'|'I'|'M'|'N'|'P'|'S'|'U'|'V'|'W'|'X'|'f'|'g'|'h'|'i'|'m'|'n'|'p'|'s'|'u'|'v'|'w'|'x' => Some(Decor::Dash),
        'A'|'B'|'E'|'J'|'O'|'T'|'a'|'b'|'e'|'j'|'o'|'t' => None,
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => None, //TODO
        '\u{e000}'..='\u{e0ff}' => None, // TH & variants
        '\u{e800}'..='\u{e9ff}' => Some(Decor::Dash), // QU, NG & variants
        '\u{e100}'..='\u{e7ff}' => Some(Decor::Dot), // Other two letter variants
        '\u{ea03}'|'\u{ea04}'|'\u{ea0b}'|'\u{ea0c}'|'\u{ea11}'|'\u{ea12}'|'\u{ea19}'|'\u{ea1a}'=> Some(Decor::Dot), //repeat dots
        '\u{ea06}'..='\u{ea09}'|'\u{ea0d}'|'\u{ea0e}'|'\u{ea10}'|'\u{ea13}'|'\u{ea15}'..='\u{ea18}'=> Some(Decor::Dash), //repeat dashes
        '\u{ea01}'|'\u{ea02}'|'\u{ea05}'|'\u{ea0f}'|'\u{ea14}' => None, //repeat nones
        '\u{ea23}'|'\u{ea24}'|'\u{ea2b}'|'\u{ea2c}'|'\u{ea31}'|'\u{ea32}'|'\u{ea39}'|'\u{ea3a}'=> Some(Decor::Dot), //repeat dots
        '\u{ea26}'..='\u{ea29}'|'\u{ea2d}'|'\u{ea2e}'|'\u{ea30}'|'\u{ea33}'|'\u{ea35}'..='\u{ea38}'=> Some(Decor::Dash), //repeat dashes
        '\u{ea21}'|'\u{ea22}'|'\u{ea25}'|'\u{ea2f}'|'\u{ea34}' => None, //repeat nones
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
            '\u{e800}' => 3, //NG, 
            '\u{e200}'|'\u{e600}'|'\u{ea03}'|'\u{ea11}'=> 4, //ND, NT, CC, QQ
            '\u{ea07}'|'\u{ea09}'|'\u{ea0e}'|'\u{ea15}'|'\u{ea16}' => 1, //GG, II, NN, UU, VV
            '\u{ea08}'|'\u{ea0b}'|'\u{ea10}'|'\u{ea17}'|'\u{ea18}'|'\u{ea19}' => 2, //HH, KK, PP, WW, XX, YY,
            '\u{ea04}'|'\u{ea06}'|'\u{ea0c}'|'\u{ea0d}'|'\u{ea12}'|'\u{ea13}'|'\u{ea1a}' => 3, //DD, FF, LL, MM, RR, SS, ZZ
            '\u{ea27}'|'\u{ea29}'|'\u{ea2e}'|'\u{ea35}'|'\u{ea36}' => 1, //GGG, III, NNN, UUU, VVV
            '\u{ea28}'|'\u{ea2b}'|'\u{ea30}'|'\u{ea37}'|'\u{ea38}'|'\u{ea39}' => 2, //HHH, KKK, PPP, WWW, XXX, YYY,
            '\u{ea24}'|'\u{ea26}'|'\u{ea2c}'|'\u{ea2d}'|'\u{ea32}'|'\u{ea33}'|'\u{ea3a}' => 3, //DDD, FFF, LLL, MMM, RRR, SSS, ZZZ
            '\u{ea23}'|'\u{ea31}'=> 4, // CCC, QQQ
            _ => 0
        }
    };
    (dot, decor_num)
}

pub fn thi(letter_distance:f64, letter_radius:f64,big_radius:f64) -> Result<f64, Error> {
    let thi = ((big_radius.powf(2.0) + letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*big_radius)).acos();
    if thi.is_nan() {
        Err(Error::new(GallError::LetterNotTouchingSkel))
    } else {
        Ok(thi)
    }
}

pub fn theta(letter_distance:f64, letter_radius:f64,big_radius:f64) -> Result<f64, Error> {
    let theta = ((big_radius.powf(2.0) - letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*letter_radius)).acos();
    if theta.is_nan() {
        Err(Error::new(GallError::LetterNotTouchingSkel))
    } else {
        Ok(theta)
    }
}