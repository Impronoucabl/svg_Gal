use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    pub error_type: GallError,
}
impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub enum GallError {
    AngleUndefined,
    NegativeDistanceErr, 
    TooThick,
    NotThickEnough,
    DistTooShort ,
    DistTooLong ,
    RadiusTooShort, 
    RadiusTooLong ,
    VowelRadiusTooShort ,
    VowelRadiusTooLong ,
    InvalidVowelDist ,
    DoNotMutTainer ,
    NoStepSpace ,
    NoStemInTainer ,
    LetterNotTouchingSkel,
    TainerNotInit,
}

impl Error {
    pub fn new(err:GallError) -> Error {
        Error {
            error_type: err,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match &self.error_type {
            GallError::AngleUndefined => "Angle is undefined.",
            GallError::NegativeDistanceErr => "Distance is negative",
            GallError::TooThick => "Too much of dat THICCC",
            GallError::NotThickEnough => "Not enough THICCC",
            GallError::DistTooShort => "Stem distance is too short",
            GallError::DistTooLong => "Stem distance is too long",
            GallError::RadiusTooShort => "Stem radius is too short",
            GallError::RadiusTooLong => "Stem radius is too long",
            GallError::VowelRadiusTooShort => "Vowel radius is too short",
            GallError::VowelRadiusTooLong => "Vowel radius is too long",
            GallError::InvalidVowelDist => "Vowel radius is invalid",
            GallError::DoNotMutTainer =>"Do Not Mut Tainer",
            GallError::NoStepSpace =>"Stepping further will cross 6 o'clock",
            GallError::NoStemInTainer =>"Tainer stemtype is None",
            GallError::LetterNotTouchingSkel=>"Letter is not touching skeleton",
            GallError::TainerNotInit=>"Tainer has not been initialised yet",
            _ => "Unspecified Error",
        };
        write!(f, "{}",message)
    }
}
