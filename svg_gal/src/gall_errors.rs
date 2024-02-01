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

impl Error for NegativeDistanceErr {}
impl Error for StemDistTooShort {}
impl Error for StemDistTooLong {}
impl Error for StemRadiusTooShort {}
impl Error for StemRadiusTooLong {}

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