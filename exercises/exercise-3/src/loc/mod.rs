//! This module defines both the main set of `Locations`, as well its subsets
//! in its submodules like villages and supplies. Helps by iterating over
//! locations:
//!
//! ```
//! # use exercise_3::loc::Location;
//!
//! // iterate over all locations
//! for location in Location::iter() {
//!     // create assertions or variables here...
//! }
//!
//! // or iterate over combinations
//! for (location1, location2) in Location::iter_tuple() {
//!     // create assertions or variables here...
//! }
//! ```

mod consts;
pub mod route;
pub mod supplies;
pub mod village;

pub(super) use route::*;
pub(super) use supplies::*;
pub(super) use village::*;

use self::consts::*;

use ::std::fmt;
use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

/// The locations on the map.
#[derive(Clone, Copy, Hash, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum Location {
    /// Village A.
    A,
    /// Village B.
    B,
    /// Village C.
    C,
    /// The supply.
    S,
}

impl Location {
    /// The distance to another `Location`.
    pub fn distance_to(&self, other: &Self) -> Option<u64> {
        (self, other).distance()
    }

    /// Iterates over all enum options, and their internal enum options.
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }

    /// Whether this location is a Supply.
    pub fn is_supply(&self) -> bool {
        Supply::try_from(*self).is_ok()
    }

    /// Whether this location is a Village.
    pub fn is_village(&self) -> bool {
        Village::try_from(*self).is_ok()
    }

    /// The amount of `Location`s that exist.
    pub fn count() -> usize {
        Self::iter().count()
    }

    /// Iterate over all combinations as a tuple.
    pub fn iter_tuple() -> impl Iterator<Item = (Self, Self)> {
        Self::iter().flat_map(|a| Self::iter().map(move |b| (a, b)))
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Location::*;
        match self {
            A => write!(f, "A"),
            B => write!(f, "B"),
            C => write!(f, "C"),
            S => write!(f, "S"),
        }
    }
}

trait LocationTupleDistance {
    fn distance(&self) -> Option<u64>;
}

impl LocationTupleDistance for (&Location, &Location) {
    fn distance(&self) -> Option<u64> {
        use Location::*;
        match self {
            (A, A) | (B, B) | (C, C) | (S, S) => Some(DISTANCE_SELF),
            (B, S) | (S, B) => None,
            (A, B) => Some(DISTANCE_A_B),
            (B, A) => Some(DISTANCE_B_A),
            (A, C) => Some(DISTANCE_A_C),
            (C, A) => Some(DISTANCE_C_A),
            (B, C) => Some(DISTANCE_B_C),
            (C, B) => Some(DISTANCE_C_B),
            (A, S) => Some(DISTANCE_A_S),
            (S, A) => Some(DISTANCE_S_A),
            (C, S) => Some(DISTANCE_C_S),
            (S, C) => Some(DISTANCE_S_C),
        }
    }
}

impl LocationTupleDistance for (Location, Location) {
    fn distance(&self) -> Option<u64> {
        (&self.0, &self.1).distance()
    }
}

impl LocationTupleDistance for (&mut Location, &mut Location) {
    fn distance(&self) -> Option<u64> {
        (&*self.0, &*self.1).distance()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_tuple() {
        let routes: Vec<_> = Location::iter_tuple().collect();
        for location1 in Location::iter() {
            for location2 in Location::iter() {
                let combination = (location1, location2);
                assert!(routes.contains(&combination), "{:?}", routes);
            }
        }
    }
}
