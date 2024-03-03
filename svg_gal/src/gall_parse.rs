
use crate::gall_char::GallChar;
use crate::gall_config::Config;
use crate::gall_fn::LetterMark;
use crate::gall_stem::StemType;

pub struct PreWord {
    pub pre_vec: Vec<PreSyl>,
    len: usize,
}

pub struct PreSyl {
    pub stem_type:Option<StemType>,
    pub char_vec: Vec<GallChar>,
}

impl PreSyl {
    pub fn new() -> PreSyl {
        PreSyl {
            stem_type: None, 
            char_vec: Vec::with_capacity(1), 
        }
    }
    fn is_empty(&self) -> bool {
        self.char_vec.is_empty()
    }
    pub fn push(&mut self, letter: GallChar) -> Result<(), GallChar> {
        if self.is_empty() {
            self.stem_type = letter.stem;
            self.char_vec.push(letter);
            Ok(())
        } else if let LetterMark::GallVowel(_) = letter.letter {
            if Config::STACK || (self.char_vec.len() < 2 && self.stem_type != None) {
                self.char_vec.push(letter);
                Ok(())
            } else {
                Err(letter)
            }
        } else if Config::STACK {
            if letter.stem == self.stem_type {
                self.char_vec.push(letter);
                Ok(())
            } else {
                Err(letter)
            }
        } else {
            Err(letter)
        }
    }
}

impl PreWord {
    pub fn new(text:String, len_guess:usize) -> PreWord {
        let mut pre_vec = Vec::with_capacity(len_guess);
        let mut con_vec = PreSyl::new();
        let mut con_count :usize = 1; 
        for cha in text.chars() {
            let letter = GallChar::new(cha);
            match con_vec.push(letter) {
                Ok(_) => {},
                Err(lett) => {
                    pre_vec.push(con_vec);
                    con_vec = PreSyl::new();
                    con_count += 1;
                    _ = con_vec.push(lett);
                }
            };
        }
        pre_vec.push(con_vec);
        PreWord{
            pre_vec,
            len:con_count
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
}