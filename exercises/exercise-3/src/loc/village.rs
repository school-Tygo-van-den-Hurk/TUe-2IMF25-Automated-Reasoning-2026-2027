//! The module on the villages. This is a subset of `Locations`.
//! This module is mostly here to iterate over Villages, or to
//! check if a location is a village:
//!
//! ```
//! # use exercise_3::loc::supplies::Village;
//! # use exercise_3::loc::Location;
//! assert!(Location::A.is_supply);
//!
//! for village in Village::iter() {
//!     let location: Location;
//!     location = village.into();
//!     assert!(location.is_village());
//! }
//! ```

use super::Location;
use super::consts::*;

use ::std::convert::TryFrom;
use ::std::fmt;
use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

/// The villages on the map.
#[derive(Clone, Copy, Hash, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Village {
    /// Village A.
    A,
    /// Village B.
    B,
    /// Village C.
    C,
}

impl Village {
    /// Gets the starting capacity of a village.
    pub fn starting_capacity(&self) -> u64 {
        VILLAGE_STARTING_CAPACITY
    }

    /// Gets the capacity of a village.
    pub fn capacity(&self) -> u64 {
        use self::Village::*;
        match self {
            A => CAPACITY_A,
            B => CAPACITY_B,
            C => CAPACITY_C,
        }
    }

    /// The distance to another `Village`.
    pub fn distance_to(&self, other: &Self) -> Option<u64> {
        Location::from(*self).distance_to(&Location::from(*other))
    }

    /// Iterates over all enum options.
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }

    /// Iterate over all combinations as a tuple.
    pub fn iter_tuple() -> impl Iterator<Item = (Self, Self)> {
        Self::iter().flat_map(|a| Self::iter().map(move |b| (a, b)))
    }

    /// Returns the name of the village.
    pub fn name(&self) -> String {
        format!("{self}")
    }

    /// The id of this village.
    pub fn id(&self) -> u64 {
        use Village::*;
        match self {
            A => LOCATION_ID_A,
            B => LOCATION_ID_B,
            C => LOCATION_ID_C,
        }
    }

    /// The amount of `Village`s that exist.
    pub fn count() -> usize {
        Self::iter().count()
    }
}

impl fmt::Display for Village {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Village::*;
        match self {
            A => write!(f, "A"),
            B => write!(f, "B"),
            C => write!(f, "C"),
        }
    }
}

impl TryFrom<Location> for Village {
    type Error = String;

    fn try_from(location: Location) -> Result<Self, Self::Error> {
        match location {
            Location::A => Ok(Village::A),
            Location::B => Ok(Village::B),
            Location::C => Ok(Village::C),
            Location::S => {
                let err = format!("{location} is not a village");
                Err(err)
            }
        }
    }
}

impl From<Village> for Location {
    fn from(village: Village) -> Self {
        match village {
            Village::A => Location::A,
            Village::B => Location::B,
            Village::C => Location::C,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name() {
        use Village::*;
        assert_eq!(A.name(), "A");
        assert_eq!(B.name(), "B");
        assert_eq!(C.name(), "C");
    }

    #[test]
    fn id() {
        assert_eq!(Village::A.id(), LOCATION_ID_A);
        assert_eq!(Village::B.id(), LOCATION_ID_B);
        assert_eq!(Village::C.id(), LOCATION_ID_C);
    }

    #[test]
    fn capacity() {
        assert_eq!(Village::A.capacity(), CAPACITY_A);
        assert_eq!(Village::B.capacity(), CAPACITY_B);
        assert_eq!(Village::C.capacity(), CAPACITY_C);
    }

    #[test]
    fn iter_tuple() {
        let all_combinations: Vec<_> = Village::iter_tuple().collect();
        for village1 in Village::iter() {
            for village2 in Village::iter() {
                let combination = (village1, village2);
                assert!(
                    all_combinations.contains(&combination),
                    "{all_combinations:?}.contains(&{combination:?})",
                );
            }
        }
    }

    #[test]
    fn try_from_location() {
        assert!(Location::from(Village::A) == Location::A);
        assert!(Location::from(Village::B) == Location::B);
        assert!(Location::from(Village::C) == Location::C);
    }
}
