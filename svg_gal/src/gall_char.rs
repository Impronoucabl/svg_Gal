use crate::gall_fn::{self, LetterMark, DecorType};
use crate::gall_stem::StemType;

pub struct GallChar {
    pub letter: LetterMark,
    pub stem:Option<StemType>,
    decor: Option<DecorType>,
    decor_num: i8,
    repeat: bool,
}

impl GallChar {
    pub fn new(letter: char) -> GallChar {
        let (l_mark, repeat) = gall_fn::stem_lookup(&letter);
        let stem = match l_mark {
            LetterMark::Stem(val) => Some(val),
            LetterMark::GallVowel(val) => None,
            LetterMark::Digit(_) => Some(StemType::J),
            LetterMark::GallMark => None,
        };
        let (decor, decor_num) = gall_fn::dot_lookup(&letter);
        GallChar {
            letter: l_mark,
            stem,
            decor,
            decor_num,
            repeat,
        }
    }
}