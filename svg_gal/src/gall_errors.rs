use std::fmt;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct NegativeDistanceErr;
#[derive(Debug, Clone)]
pub struct StemDistTooShort;
#[derive(Debug, Clone)]
pub struct StemDistTooLong;
#[derive(Debug, Clone)]
pub struct StemRadiusTooShort;
#[derive(Debug, Clone)]
pub struct StemRadiusTooLong;
#[derive(Debug, Clone)]
pub struct VowelRadiusTooShort;
#[derive(Debug, Clone)]
pub struct VowelRadiusTooLong;

#[derive(Debug, Clone)]
pub struct InvalidVowelDist;
#[derive(Debug, Clone)]
pub struct TainerMissingStem;
#[derive(Debug, Clone)]
pub struct BadTainerStem;
#[derive(Debug, Clone)]
pub struct BadVowelType;

impl Error for NegativeDistanceErr {}
impl Error for StemDistTooShort {}
impl Error for StemDistTooLong {}
impl Error for StemRadiusTooShort {}
impl Error for StemRadiusTooLong {}
impl Error for VowelRadiusTooShort {}
impl Error for VowelRadiusTooLong {}
impl Error for InvalidVowelDist {}
impl Error for TainerMissingStem {}
impl Error for BadTainerStem {}
impl Error for BadVowelType {}

impl fmt::Display for NegativeDistanceErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Distance is negative")
    }
}
impl fmt::Display for StemDistTooShort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stem distance is too short")
    }
}
impl fmt::Display for StemDistTooLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stem distance is too long")
    }
}
impl fmt::Display for StemRadiusTooShort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stem radius is too short")
    }
}
impl fmt::Display for StemRadiusTooLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stem radius is too long")
    }
}
impl fmt::Display for VowelRadiusTooShort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vowel radius is too short")
    }
}
impl fmt::Display for VowelRadiusTooLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vowel radius is too long")
    }
}
impl fmt::Display for InvalidVowelDist {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vowel radius is invalid")
    }
}
impl fmt::Display for TainerMissingStem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tainer has no stem type")
    }
}
impl fmt::Display for BadTainerStem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stem has wrong stem type for Tainer")
    }
}
impl fmt::Display for BadVowelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vowel has wrong vowel type for Tainer")
    }
}