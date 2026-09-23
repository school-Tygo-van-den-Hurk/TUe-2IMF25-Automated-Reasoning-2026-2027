//! Module for all supply locations. Allows me to easily add more supply
//! locations if I wanted to. Is mainly there to iterate over all supplies
//! or to check if some `Location` is a supply.
//!
//! ```
//! # use exercise_3::loc::supplies::Supply;
//! # use exercise_3::loc::Location;
//! assert!(Location::S.is_supply);
//!
//! for supply in Supply::iter() {
//!     let location: Location;
//!     location = supply.into();
//!     assert!(location.is_supply());
//! }
//! ```

use super::Location;
use super::consts::*;

use ::std::convert::TryFrom;
use ::std::fmt;
use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

/// The supplies on the map.
#[derive(Clone, Copy, Hash, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Supply {
    S,
}

impl Supply {
    /// Returns the name of the supply.
    pub fn name(&self) -> String {
        format!("{self}")
    }

    /// The amount of supplies that exist.
    pub fn count() -> usize {
        Self::iter().count()
    }

    /// Gets the ID of the supply.
    pub fn id(&self) -> u64 {
        match self {
            Self::S => LOCATION_ID_S,
        }
    }
}

impl fmt::Display for Supply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::S => write!(f, "S"),
        }
    }
}

impl TryFrom<Location> for Supply {
    type Error = String;

    fn try_from(location: Location) -> Result<Self, Self::Error> {
        use Location::*;
        match location {
            S => Ok(Self::S),
            A | B | C => {
                let err = format!("{location} is a village");
                Err(err)
            }
        }
    }
}

impl From<Supply> for Location {
    fn from(supply: Supply) -> Self {
        match supply {
            Supply::S => Location::S,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        use Supply::*;
        assert_eq!(S.name(), "S");
    }

    #[test]
    fn id() {
        assert_eq!(Supply::S.id(), LOCATION_ID_S);
    }
}
