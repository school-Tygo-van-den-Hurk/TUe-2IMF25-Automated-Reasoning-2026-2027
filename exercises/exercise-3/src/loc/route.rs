//! A route is a valid combination of 2 locations. The way this is checked is
//! by seeing what combinations of `Location`s return a distance, if it does
//! not return a distance it is not a valid route.
//!
//! The main use case is to loop over all routes:
//!
//! ```
//! # use exercise_3::loc::route::Route;
//! # use exercise_3::loc::Location;
//! for (location1, location2) in Route::iter_tuple() {
//!   println!("location1 = {location1}");
//!   println!("location2 = {location2}");
//! }
//! ```
//!
//! You can also use it to check if a pair of locations is a valid route:
//!
//! ```
//! # use exercise_3::loc::route::Route;
//! # use exercise_3::loc::Location;
//! let location1 = Location::A;
//! let location2 = Location::B;
//! let combination = (location1, location2);
//! let route = Route::try_from(combination);
//! assert!(route.is_ok());
//! ```
//!

use super::Location;
use ::std::fmt;

/// A valid path between 2 location.
#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Route {
    /// The starting location of the route.
    start: Location,
    /// The ending location of the route.
    end: Location,
}

impl Route {
    pub fn start(&self) -> Location {
        self.start
    }

    pub fn end(&self) -> Location {
        self.end
    }

    /// The distance in time to travel of this route.
    pub fn distance(&self) -> u64 {
        self.start.distance_to(&self.end).unwrap()
    }

    /// Iterates over all possible routes.
    pub fn iter() -> impl Iterator<Item = Self> {
        Location::iter_tuple()
            .filter(|(start, end)| start.distance_to(end).is_some())
            .map(|(start, end)| Self { start, end })
    }

    pub fn iter_tuple() -> impl Iterator<Item = (Location, Location)> {
        Self::iter().map(|route| route.into())
    }
}

impl From<Route> for (Location, Location) {
    fn from(route: Route) -> Self {
        (route.start, route.end)
    }
}

impl TryFrom<(Location, Location)> for Route {
    type Error = String;

    fn try_from((start, end): (Location, Location)) -> Result<Self, Self::Error> {
        match start.distance_to(&end) {
            Some(_) => Ok(Self { start, end }),
            None => Err(format!(
                "the route between {start} and {end} does not exist"
            )),
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_ways() {
        use Location::*;
        let routes: Vec<_> = Route::iter().collect();
        let one_way = Route::try_from((A, B)).unwrap();
        let back = Route::try_from((B, A)).unwrap();
        assert!(routes.contains(&one_way), "{:?}", routes);
        assert!(routes.contains(&back), "{:?}", routes);
    }

    #[test]
    fn stay() {
        let routes: Vec<_> = Route::iter().collect();
        for location in Location::iter() {
            let edge = (location, location);
            let staying = Route::try_from(edge).unwrap();
            assert!(routes.contains(&staying), "{:?}", routes);
        }
    }
}
