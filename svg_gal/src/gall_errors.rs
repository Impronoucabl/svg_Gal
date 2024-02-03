use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    pub error_type: str,
}
impl std::error::Error for Error {}
impl Error {
    pub fn new(err:GallError) -> Error {
        Error {
            error_type: err,
        }
    }
}
pub enum GallError {
    NegativeDistanceErr, 
    StemDistTooShort ,
    StemDistTooLong ,
    StemRadiusTooShort, 
    StemRadiusTooLong ,
    VowelRadiusTooShort ,
    VowelRadiusTooLong ,
    InvalidVowelDist ,
    DoNotMutTainer ,
    TainerMissingStem ,
    BadTainerStem ,
    StemAlreadySet,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self.error_type {
            NegativeDistanceErr => "Distance is negative",
            StemDistTooShort => "Stem distance is too short",
            StemDistTooLong => "Stem distance is too long",
            StemRadiusTooShort => "Stem radius is too short",
            StemRadiusTooLong => "Stem radius is too long",
            VowelRadiusTooShort => "Vowel radius is too short",
            VowelRadiusTooLong => "Vowel radius is too long",
            InvalidVowelDist => "Vowel radius is invalid",
            DoNotMutTainer =>"Do Not Mut Tainer",
            TainerMissingStem =>"Tainer's stem type is none",
            BadTainerStem =>"Stem has wrong stem type for Tainer",
            StemAlreadySet=>"Tainer already contains a vowel or Stem",
            _ => "Unspecified Error",
        };
        write!(f, "{}",message)
    }
}
