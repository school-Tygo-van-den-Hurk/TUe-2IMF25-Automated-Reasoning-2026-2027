pub mod consts;
pub mod point;
pub mod prelude;
pub mod road;

use crate::consts::*;
use crate::point::Point;
use crate::prelude::*;
use crate::road::Road;

/// Creates the road variables.
pub fn create_roads() -> HashMap<Road, Bool> {
    Road::iter().fold(HashMap::new(), |mut result, road| {
        let name = format!("road_{}", road);
        let variable = Bool::new_const(name);
        result.insert(road.clone(), variable);
        result
    })
}

/// Points of interest.
pub fn ___(solver: &Solver, points: HashMap<Point, Bool>) {
    let iter = Point::points_of_interest();
    let zero = Int::from_u64(0);
    let total = iter.fold(zero, |total, point| {
        let is_there = &points[point];
        total + is_there.ite(&Int::from_u64(1), &Int::from_u64(0))
    });

    let minimum = MINIMUM_AMOUNT_OF_POINTS_OF_INTEREST;
    let minimum = Int::from_u64(minimum as u64);
    solver.assert(total.simplify().gt(minimum));
}
