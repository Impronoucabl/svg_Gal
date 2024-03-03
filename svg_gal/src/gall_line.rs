use crate::gall_word::GallWord;

pub struct GallLine<'a> {
    pub words: Vec<GallWord<'a>>
}