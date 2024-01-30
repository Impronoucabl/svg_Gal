use std::fmt;
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct NegativeDistanceErr;

impl Error for NegativeDistanceErr {}

impl fmt::Display for NegativeDistanceErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Distance is negative")
    }
}

#[derive(Debug, Clone)]
pub struct StemDistTooShort;

impl Error for StemDistTooShort {}

impl fmt::Display for StemDistTooShort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stem distance is too short")
    }
}

#[derive(Debug, Clone)]
pub struct StemDistTooLong;

impl Error for StemDistTooLong {}

impl fmt::Display for StemDistTooLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stem distance is too long")
    }
}