//!

use crate::prelude::*;
use ::std::fmt;

/// A point in 2D space.
#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    /// The x coordinate of the point.
    pub(crate) x: isize,
    /// The y coordinate of the point.
    pub(crate) y: isize,
}

impl Point {
    /// Represents this point as a string.
    pub fn to_string(&self) -> String {
        format!("{self}")
    }

    /// Returns an iterator over all intersections on the map.
    pub fn intersections() -> impl Iterator<Item = &'static Self> {
        Self::INTERSECTIONS.iter()
    }

    /// Calculates the distance between two points.
    pub fn distance(&self, other: &Self) -> usize {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        (dx * dx + dy * dy).sqrt().ceil() as usize
    }

    /// The intersections of the map.
    const INTERSECTIONS: [Point; 64] = [
        Point { x: 0, y: 0 },
        Point { x: 2, y: 0 },
        Point { x: 4, y: 0 },
        Point { x: 6, y: 0 },
        Point { x: 3, y: -1 },
        Point { x: 5, y: -1 },
        Point { x: 1, y: -1 },
        Point { x: 2, y: -1 },
        Point { x: 0, y: -2 },
        Point { x: 1, y: -2 },
        Point { x: 6, y: -2 },
        Point { x: 8, y: -2 },
        Point { x: 1, y: -3 },
        Point { x: 2, y: -3 },
        Point { x: 3, y: -3 },
        Point { x: 4, y: -3 },
        Point { x: 5, y: -3 },
        Point { x: 0, y: -4 },
        Point { x: 2, y: -4 },
        Point { x: 3, y: -4 },
        Point { x: 5, y: -4 },
        Point { x: 6, y: -4 },
        Point { x: 7, y: -4 },
        Point { x: 0, y: -5 },
        Point { x: 1, y: -5 },
        Point { x: 2, y: -5 },
        Point { x: 4, y: -5 },
        Point { x: 0, y: -6 },
        Point { x: 1, y: -6 },
        Point { x: 2, y: -6 },
        Point { x: 4, y: -6 },
        Point { x: 5, y: -6 },
        Point { x: 7, y: -6 },
        Point { x: 8, y: -6 },
        Point { x: 0, y: -7 },
        Point { x: 1, y: -7 },
        Point { x: 2, y: -7 },
        Point { x: 4, y: -7 },
        Point { x: 5, y: -7 },
        Point { x: 6, y: -7 },
        Point { x: 7, y: -7 },
        Point { x: 8, y: -7 },
        Point { x: 1, y: -8 },
        Point { x: 4, y: -8 },
        Point { x: 5, y: -8 },
        Point { x: 6, y: -8 },
        Point { x: 9, y: -8 },
        Point { x: 2, y: -9 },
        Point { x: 5, y: -9 },
        Point { x: 8, y: -9 },
        Point { x: 0, y: -10 },
        Point { x: 3, y: -10 },
        Point { x: 5, y: -10 },
        Point { x: 7, y: -10 },
        Point { x: 9, y: -10 },
        Point { x: 1, y: -11 },
        Point { x: 2, y: -11 },
        Point { x: 3, y: -11 },
        Point { x: 4, y: -11 },
        Point { x: 8, y: -11 },
        Point { x: 2, y: -12 },
        Point { x: 3, y: -12 },
        Point { x: 4, y: -12 },
        Point { x: 7, y: -12 },
    ];

    /// Returns an iterator over all points of interest.
    pub fn points_of_interest() -> impl Iterator<Item = &'static Self> {
        Self::POINTS_OF_INTEREST.iter()
    }

    const POINTS_OF_INTEREST: [Point; 9] = [
        Point { x: 6, y: 0 },
        Point { x: 1, y: -1 },
        Point { x: 5, y: -3 },
        Point { x: 7, y: -4 },
        Point { x: 0, y: -7 },
        Point { x: 2, y: -7 },
        Point { x: 5, y: -10 },
        Point { x: 9, y: -10 },
        Point { x: 3, y: -12 },
    ];
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let p1 = Point { x: -1, y: 0 };
        let p2 = Point { x: 1, y: 0 };
        assert_eq!(p1.distance(&p2), 2);
        let p2 = p1.clone();
        assert_eq!(p1.distance(&p2), 0);
        let p1 = Point { x: 0, y: 0 };
        let p2 = Point { x: 1, y: 1 };
        assert_eq!(p1.distance(&p2), 2);
    }

    #[test]
    fn display() {
        let point = Point { x: 0, y: 0 };
        let displayed = format!("{point}");
        assert_eq!(displayed, "(0, 0)");
        let point = Point { x: 1, y: 2 };
        let displayed = format!("{point}");
        assert_eq!(displayed, "(1, 2)");
    }
}
