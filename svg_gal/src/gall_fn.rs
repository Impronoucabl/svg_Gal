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

pub fn stem_lookup(letter:char) -> LetterType {
    match letter {
        'A'|'a'                                                 => LetterType::AVowel,
        'E'|'I'|'U'|'e'|'i'|'u'                                 => LetterType::StaticVowel,
        'O'|'o'                                                 => LetterType::OVowel,
        '█'|'B'|'D'|'F'|'G'|'H'|'b'|'d'|'f'|'g'|'h'             => LetterType::BStem, // Also CH & ND
        'C'|'J'|'K'|'L'|'M'|'N'|'P'|'c'|'j'|'k'|'l'|'m'|'n'|'p' => LetterType::JStem, // Also PH
        'R'|'S'|'T'|'V'|'W'|'r'|'s'|'t'|'v'|'w'                 => LetterType::TStem, // Also WH, SH, NT
        'Q'|'X'|'Y'|'Z'|'q'|'x'|'y'|'z'                         => LetterType::ZStem, // Also GH, NG, QU, TH
        '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'                 => LetterType::Digit, // TODO
        _ => LetterType::Punctuation, //TODO
    }
}

pub fn decor_lookup(letter:char) -> (Option<bool>,i8) {
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
    };
    (dot, decor_num)
}

//below is python
//math.acos((Wrd.inner_rad**2 + dist**2 - self.outer_rad**2)/(2*dist*Wrd.inner_rad))
pub fn thi(letter_distance:f64, letter_radius:f64,big_radius:f64) -> f64 {
    let thi = ((big_radius.powf(2.0) + letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*big_radius)).acos();
    if thi == std::f64::NAN {
        0.0 //could do math error?
    } else {
        thi
    }
}

//below is python
//self.theta  = math.acos((Wrd.inner_rad**2 - dist**2 - self.outer_rad**2)/(2*dist*self.outer_rad))
pub fn theta(letter_distance:f64, letter_radius:f64,big_radius:f64) -> f64 {
    let theta = ((big_radius.powf(2.0) - letter_distance.powf(2.0) - letter_radius.powf(2.0))/(2.0*letter_distance*letter_radius)).acos();
    if theta == std::f64::NAN {
        0.0 //could do math error?
    } else {
        theta
    }
}