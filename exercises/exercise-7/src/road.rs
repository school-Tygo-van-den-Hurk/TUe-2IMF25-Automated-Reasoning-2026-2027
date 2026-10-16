//!

use crate::point::Point;

use ::std::fmt;

#[derive(Clone, Copy, Hash, Debug, Default, Eq, PartialOrd, Ord)]
pub struct Road {
    /// The start of the road
    pub(crate) start: Point,
    /// The end of the road
    pub(crate) end: Point,
}

impl PartialEq for Road {
    fn eq(&self, other: &Self) -> bool {
        let match_1 = self.start == other.start && self.end == other.end;
        let match_2 = self.start == other.end && self.end == other.start;
        match_1 || match_2
    }
}

impl Road {
    pub fn iter() -> impl Iterator<Item = &'static Self> {
        Self::ROADS.iter()
    }

    /// The roads on the map.
    pub const ROADS: [Road; 65] = [
        // Outer frame
        Road {
            start: Point { x: 0, y: 0 },
            end: Point { x: 6, y: 0 },
        },
        Road {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: -10 },
        },
        Road {
            start: Point { x: 0, y: -10 },
            end: Point { x: 5, y: -10 },
        },
        Road {
            start: Point { x: 6, y: 0 },
            end: Point { x: 6, y: -8 },
        },
        // Top / upper middle
        Road {
            start: Point { x: 4, y: 0 },
            end: Point { x: 4, y: -6 },
        },
        Road {
            start: Point { x: 2, y: 0 },
            end: Point { x: 2, y: -4 },
        },
        Road {
            start: Point { x: 3, y: -1 },
            end: Point { x: 5, y: -1 },
        },
        Road {
            start: Point { x: 5, y: -1 },
            end: Point { x: 6, y: -2 },
        },
        Road {
            start: Point { x: 3, y: -1 },
            end: Point { x: 3, y: -4 },
        },
        Road {
            start: Point { x: 3, y: -1 },
            end: Point { x: 2, y: -3 },
        },
        Road {
            start: Point { x: 3, y: -3 },
            end: Point { x: 5, y: -3 },
        },
        Road {
            start: Point { x: 5, y: -3 },
            end: Point { x: 5, y: -4 },
        },
        Road {
            start: Point { x: 5, y: -4 },
            end: Point { x: 7, y: -4 },
        },
        Road {
            start: Point { x: 3, y: -4 },
            end: Point { x: 0, y: -4 },
        },
        // Upper left
        Road {
            start: Point { x: 1, y: -1 },
            end: Point { x: 2, y: -1 },
        },
        Road {
            start: Point { x: 1, y: -1 },
            end: Point { x: 1, y: -3 },
        },
        Road {
            start: Point { x: 1, y: -3 },
            end: Point { x: 2, y: -3 },
        },
        Road {
            start: Point { x: 0, y: -2 },
            end: Point { x: 1, y: -2 },
        },
        Road {
            start: Point { x: 1, y: -3 },
            end: Point { x: 0, y: -4 },
        },
        // Right side
        Road {
            start: Point { x: 6, y: -2 },
            end: Point { x: 8, y: -2 },
        },
        Road {
            start: Point { x: 8, y: -2 },
            end: Point { x: 8, y: -7 },
        },
        Road {
            start: Point { x: 6, y: -4 },
            end: Point { x: 7, y: -6 },
        },
        Road {
            start: Point { x: 7, y: -4 },
            end: Point { x: 7, y: -7 },
        },
        Road {
            start: Point { x: 7, y: -6 },
            end: Point { x: 8, y: -6 },
        },
        Road {
            start: Point { x: 6, y: -7 },
            end: Point { x: 8, y: -7 },
        },
        Road {
            start: Point { x: 8, y: -7 },
            end: Point { x: 9, y: -8 },
        },
        Road {
            start: Point { x: 9, y: -8 },
            end: Point { x: 9, y: -10 },
        },
        Road {
            start: Point { x: 9, y: -10 },
            end: Point { x: 8, y: -11 },
        },
        Road {
            start: Point { x: 8, y: -11 },
            end: Point { x: 7, y: -12 },
        },
        Road {
            start: Point { x: 7, y: -12 },
            end: Point { x: 4, y: -12 },
        },
        Road {
            start: Point { x: 8, y: -7 },
            end: Point { x: 8, y: -9 },
        },
        Road {
            start: Point { x: 8, y: -9 },
            end: Point { x: 7, y: -10 },
        },
        Road {
            start: Point { x: 7, y: -10 },
            end: Point { x: 8, y: -11 },
        },
        Road {
            start: Point { x: 7, y: -10 },
            end: Point { x: 7, y: -12 },
        },
        Road {
            start: Point { x: 7, y: -10 },
            end: Point { x: 5, y: -9 },
        },
        Road {
            start: Point { x: 5, y: -9 },
            end: Point { x: 2, y: -9 },
        },
        Road {
            start: Point { x: 5, y: -9 },
            end: Point { x: 5, y: -10 },
        },
        // Centre
        Road {
            start: Point { x: 2, y: -5 },
            end: Point { x: 4, y: -5 },
        },
        Road {
            start: Point { x: 2, y: -5 },
            end: Point { x: 2, y: -9 },
        },
        Road {
            start: Point { x: 2, y: -7 },
            end: Point { x: 4, y: -7 },
        },
        Road {
            start: Point { x: 4, y: -7 },
            end: Point { x: 6, y: -7 },
        },
        Road {
            start: Point { x: 4, y: -6 },
            end: Point { x: 5, y: -6 },
        },
        Road {
            start: Point { x: 5, y: -6 },
            end: Point { x: 6, y: -7 },
        },
        Road {
            start: Point { x: 5, y: -6 },
            end: Point { x: 5, y: -8 },
        },
        Road {
            start: Point { x: 4, y: -7 },
            end: Point { x: 4, y: -8 },
        },
        Road {
            start: Point { x: 4, y: -8 },
            end: Point { x: 6, y: -8 },
        },
        Road {
            start: Point { x: 4, y: -7 },
            end: Point { x: 5, y: -8 },
        },
        // Left middle
        Road {
            start: Point { x: 0, y: -5 },
            end: Point { x: 1, y: -5 },
        },
        Road {
            start: Point { x: 1, y: -5 },
            end: Point { x: 1, y: -6 },
        },
        Road {
            start: Point { x: 1, y: -6 },
            end: Point { x: 0, y: -6 },
        },
        Road {
            start: Point { x: 0, y: -6 },
            end: Point { x: 0, y: -5 },
        },
        Road {
            start: Point { x: 1, y: -6 },
            end: Point { x: 0, y: -7 },
        },
        Road {
            start: Point { x: 0, y: -7 },
            end: Point { x: 1, y: -7 },
        },
        Road {
            start: Point { x: 1, y: -7 },
            end: Point { x: 1, y: -6 },
        },
        Road {
            start: Point { x: 0, y: -7 },
            end: Point { x: 1, y: -8 },
        },
        Road {
            start: Point { x: 1, y: -8 },
            end: Point { x: 0, y: -10 },
        },
        // Bottom
        Road {
            start: Point { x: 3, y: -10 },
            end: Point { x: 3, y: -12 },
        },
        Road {
            start: Point { x: 3, y: -12 },
            end: Point { x: 2, y: -12 },
        },
        Road {
            start: Point { x: 2, y: -12 },
            end: Point { x: 2, y: -11 },
        },
        Road {
            start: Point { x: 2, y: -11 },
            end: Point { x: 3, y: -12 },
        },
        Road {
            start: Point { x: 3, y: -10 },
            end: Point { x: 4, y: -11 },
        },
        Road {
            start: Point { x: 4, y: -11 },
            end: Point { x: 5, y: -10 },
        },
        Road {
            start: Point { x: 4, y: -11 },
            end: Point { x: 4, y: -12 },
        },
        Road {
            start: Point { x: 0, y: -10 },
            end: Point { x: 2, y: -12 },
        },
        Road {
            start: Point { x: 3, y: -12 },
            end: Point { x: 4, y: -12 },
        },
    ];
}

impl fmt::Display for Road {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <=> {}", self.start, self.end)
    }
}
