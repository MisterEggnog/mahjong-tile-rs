use crate::Tile;
use std::fmt;

/// Error returned when casting Tile to str
#[derive(Debug)]
pub struct TileCastingError(Tile);

impl fmt::Display for TileCastingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "this tile variant ({:?}) is not castable", self.0)
    }
}

impl std::error::Error for TileCastingError {}
