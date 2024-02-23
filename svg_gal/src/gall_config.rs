
pub struct Config {}

impl Config {
    pub const WIDTH:f64 = 2048.0;
    pub const HEIGHT:f64 = 2048.0;
    pub const STACK: bool = false;
    pub const SENT_THICK: u16 = 20;
    pub const LETTER_FRAC_OF_WRD: f64 = 0.35;
    pub const LETTER_THICK_FRAC: f64 = 0.6;
    pub const VOWEL_FRAC_OF_WRD : f64 = 0.15;
    pub const VOWEL_THICK_FRAC: f64 = 0.45;
    pub const DOT_RADIUS: f64 = 20.0;
    pub const COLLISION_DIST: f64 = 0.001;
    pub const fn BG_COLOUR() -> &'static str {"yellow"}
    pub const fn BG2_COLOUR() -> &'static str {"green"}
    pub const fn DOT_COLOUR() -> &'static str {"blue"}
    pub const fn JZ_COLOUR() -> &'static str {"black"}
    pub const fn VOW_COLOUR() -> &'static str {"red"}
    pub const fn SKEL_COLOUR() -> &'static str {"black"}
    pub const fn SENT_COLOUR() -> &'static str {"black"}
    
}

