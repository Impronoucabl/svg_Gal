
pub struct Config {}

impl Config {
    pub const NODE_VISIBILITY: bool = true;
    pub const ENABLE_CANVAS: bool = true;
    pub const WIDTH:f64 = 2048.0;
    pub const HEIGHT:f64 = 2048.0;
    pub const STACK: bool = true;
    pub const SENT_RADIUS: f64 = 1020.0;
    pub const SENT_THICK: f64 = 20.0;
    pub const LETTER_FRAC_OF_WRD: f64 = 0.35;
    pub const LETTER_THICK_FRAC: f64 = 0.6;
    pub const STACK_SEP_DIST: f64 = 25.0;
    pub const CONSEC_LETT_GROWTH: f64 = 15.0;
    pub const VOWEL_FRAC_OF_WRD : f64 = 0.15;
    pub const VOWEL_THICK_FRAC: f64 = 0.45;
    pub const DOT_RADIUS: f64 = 25.0;
    pub const DEF_DOT_SPREAD: f64 = 0.6;
    pub const DEF_PAIR_THICK: i16 = 4;
    pub const COLLISION_DIST: f64 = 0.0001;
    pub const STEP_DIST: f64 = 2.0 * Config::COLLISION_DIST;
    pub const fn DEBUG_COLOUR() -> &'static str {"purple"}
    pub const fn CANVAS_COLOUR() -> &'static str {"yellow"}
    pub const fn SENT_COLOUR() -> &'static str {"orange"}
    pub const fn WRD_COLOUR() -> &'static str {"green"}
    pub const fn DOT_COLOUR() -> &'static str {"blue"}
    pub const fn JZ_COLOUR() -> &'static str {"black"}
    pub const fn VOW_COLOUR() -> &'static str {"red"}
    pub const fn SKEL_COLOUR() -> &'static str {"black"}
    pub const fn SENT_SKEL_COLOUR() -> &'static str {"black"}
}

