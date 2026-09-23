//! The main module to generate SMT variables and assertions using the `loc`
//! submodule.

pub mod loc;
pub mod prelude;

use strum::IntoEnumIterator;

use crate::loc::Location;
use crate::loc::Route;
use crate::loc::Supply;
use crate::loc::Village;
use crate::prelude::*;

pub struct Smt {
    pub village_capacities: Vec<HashMap<Village, Int>>,
    pub locations: Vec<HashMap<Location, Bool>>,
    pub truck_capacities: Vec<Int>,
    pub times: Vec<Int>,
    pub solver: Solver,
}

/// A trait wrapper to implement foreign types.
pub trait Question3 {
    /// The capacity of the truck.
    const TRUCK_CAPACITY: u64;

    /// Creates the village variables, and asserts their range.
    fn create_village_variables(solver: &Solver, max_step: usize) -> Vec<HashMap<Village, Int>> {
        let result = (0..=max_step).fold(Vec::new(), |mut result, step| {
            result.push(Village::iter().fold(HashMap::new(), |mut map, village| {
                let name = format!("capacity_{village}_at_step_{step:03}");
                let capacity = Int::new_const(name);
                solver.assert(capacity.ge(0));
                solver.assert(capacity.le(village.capacity()));
                map.insert(village, capacity);
                map
            }));

            result
        });

        // asserting the villages start with 60
        for village in Village::iter() {
            let capacity = &result[0][&village];
            solver.assert(capacity.eq(village.starting_capacity()));
        }

        result
    }

    /// Creates the time variables, and asserts they must always increase.
    fn create_time_variables(solver: &Solver, max_step: usize) -> Vec<Int> {
        let result: Vec<Int> = (0..=max_step)
            .map(|step| {
                let name = format!("time_at_step_{step:03}");
                Int::new_const(name)
            })
            .collect();

        // asserting time must increase on each step
        for (previous, current) in result.iter().tuple_windows() {
            solver.assert(previous.lt(current))
        }

        // assert time starts at 0
        solver.assert(result[0].eq(0));

        result
    }

    /// Creates the truck variables, and asserts their range.
    fn create_truck_variables(solver: &Solver, max_step: usize) -> Vec<Int> {
        (0..=max_step).fold(Vec::new(), |mut result, step| {
            let name = format!("truck_capacity_at_step_{step:03}");
            let capacity = Int::new_const(name);
            solver.assert(capacity.le(Self::TRUCK_CAPACITY));
            solver.assert(capacity.ge(0));
            result.push(capacity);
            result
        })
    }

    /// Creates the location variables.
    fn create_location_variables(solver: &Solver, max_step: usize) -> Vec<HashMap<Location, Bool>> {
        let result = (0..=max_step).fold(Vec::new(), |mut result, step| {
            result.push(
                Location::iter().fold(HashMap::new(), |mut result, location| {
                    let name = format!("truck_is_at_location_{location}_at_step_{step:03}");
                    let is_there = Bool::new_const(name);
                    result.insert(location, is_there);
                    result
                }),
            );
            result
        });

        // Asserting you can be at only one location at a time.
        for state in result.iter() {
            solver.assert(Bool::pb_eq(
                &state
                    .values()
                    .map(|location| (location, 1))
                    .collect::<Vec<_>>(),
                1,
            ));
        }

        // Assert a route needs to exist or traveling is not allowed.
        for (previous, current) in result.iter().tuple_windows() {
            for (start, end) in Location::iter_tuple() {
                let is_at_start = &previous[&start];
                let is_at_end = &current[&end];
                match start.distance_to(&end) {
                    Some(_) => {}
                    None => {
                        let not_is_at_end = Bool::not(&is_at_end);
                        solver.assert(is_at_start.implies(&not_is_at_end));
                    }
                }
            }
        }

        // Start the truck at S at step 0
        solver.assert(&result[0][&Location::S]);

        result
    }

    /// Assert that time increases based on the distance you travel.
    fn assert_time_passes_as_you_travel(
        solver: &Solver,
        time: &Vec<Int>,
        locations: &Vec<HashMap<Location, Bool>>,
    ) {
        assert!(time.len() == locations.len());
        let max_steps = locations.len();
        for (previous, current) in (0..max_steps).tuple_windows() {
            let previous_time = &time[previous];
            let current_time = &time[current];
            for route in Route::iter() {
                let distance = route.distance();
                let previous_location = &locations[previous][&route.start()];
                let current_location = &locations[current][&route.end()];
                let condition = Bool::and(&[previous_location, current_location]);
                let implication = current_time.eq(previous_time + distance);
                solver.assert(condition.implies(implication));
            }
        }
    }

    /// Assert that trucks are refilled at supplies.
    fn assert_trucks_refill_at_supplies(
        solver: &Solver,
        truck_capacities: &Vec<Int>,
        locations: &Vec<HashMap<Location, Bool>>,
    ) {
        assert!(truck_capacities.len() == locations.len());
        let max_steps = locations.len();
        for step in 0..max_steps {
            let locations = &locations[step];
            let truck_capacity = &truck_capacities[step];
            for supply in Supply::iter() {
                let condition = &locations[&supply.into()];
                let implication = truck_capacity.eq(Self::TRUCK_CAPACITY);
                solver.assert(condition.implies(implication));
            }
        }
    }

    /// Asserts food in the villages goes down as time passes, but can be
    /// refilled by the truck passing.
    fn assert_food_changes(
        solver: &Solver,
        truck_capacities: &Vec<Int>,
        times: &Vec<Int>,
        locations: &Vec<HashMap<Location, Bool>>,
        village_capacities: &Vec<HashMap<Village, Int>>,
    ) {
        assert!(truck_capacities.len() == locations.len());
        assert!(truck_capacities.len() == village_capacities.len());
        let max_steps = locations.len();
        for (previous, current) in (0..max_steps).tuple_windows() {
            let previous_time = &times[previous];
            let current_time = &times[current];
            let time_difference = current_time - previous_time;
            for village in Village::iter() {
                let previous_village_capacity = &village_capacities[previous][&village];
                let current_village_capacity = &village_capacities[current][&village];
                let previous_truck_capacity = &truck_capacities[previous];
                let current_truck_capacity = &truck_capacities[current];
                let food_delivered = previous_truck_capacity - current_truck_capacity;
                let location = village.into();
                let condition = &locations[current][&location];
                solver.assert(
                    current_village_capacity.eq(previous_village_capacity - &time_difference
                        + condition.ite(&food_delivered, &Int::from_u64(0))),
                );
            }
        }
    }

    fn assert_food_in_truck_cannot_go_up_in_villages(
        solver: &Solver,
        truck_capacities: &Vec<Int>,
        locations: &Vec<HashMap<Location, Bool>>,
    ) {
        assert!(truck_capacities.len() == locations.len());
        let max_steps = locations.len();
        for (previous, current) in (0..max_steps).tuple_windows() {
            let previous_truck_capacity = &truck_capacities[previous];
            let current_truck_capacity = &truck_capacities[current];
            let current_locations = &locations[current];
            for location in Location::iter() {
                if location.is_supply() {
                    continue;
                }
                let is_there = &current_locations[&location];
                let implication = current_truck_capacity.le(previous_truck_capacity);
                solver.assert(is_there.implies(implication));
            }
        }
    }

    fn all(max_step: usize) -> Smt {
        let solver = Solver::new();

        let village_capacities = Self::create_village_variables(&solver, max_step);
        let truck_capacities = Self::create_truck_variables(&solver, max_step);
        let times = Self::create_time_variables(&solver, max_step);
        let locations = Self::create_location_variables(&solver, max_step);

        Self::assert_time_passes_as_you_travel(&solver, &times, &locations);
        Self::assert_trucks_refill_at_supplies(&solver, &truck_capacities, &locations);
        Self::assert_food_in_truck_cannot_go_up_in_villages(&solver, &truck_capacities, &locations);
        Self::assert_food_changes(
            &solver,
            &truck_capacities,
            &times,
            &locations,
            &village_capacities,
        );

        Smt {
            village_capacities,
            truck_capacities,
            locations,
            solver,
            times,
        }
    }

    fn smt(max_step: usize) -> Smt {
        Self::all(max_step)
    }
}

pub mod exercise {
    use super::*;
    use z3::ast::Ast;

    pub trait Exercise: Question3 {}

    /// The first sub-question.
    pub struct A;
    impl Question3 for A {
        const TRUCK_CAPACITY: u64 = 130;
    }

    /// The second sub-question.
    pub struct B;
    impl Question3 for B {
        const TRUCK_CAPACITY: u64 = 150;
        fn smt(max_step: usize) -> super::Smt {
            let result = Self::all(max_step);

            let mut same_state = Bool::from_bool(false);
            for step1 in 0..max_step {
                for step2 in (step1 + 1)..=max_step {
                    let mut init = Bool::from_bool(step1 != step2);

                    let capacity1 = &result.truck_capacities[step1];
                    let capacity2 = &result.truck_capacities[step2];
                    init = Bool::and(&[init, capacity1.eq(capacity2)]);

                    for location in Location::iter() {
                        let capacity1 = &result.locations[step1][&location];
                        let capacity2 = &result.locations[step2][&location];
                        init = Bool::and(&[capacity1.eq(capacity2), init]);
                    }

                    for village in Village::iter() {
                        let capacity1 = &result.village_capacities[step1][&village];
                        let capacity2 = &result.village_capacities[step2][&village];
                        init = Bool::and(&[capacity1.eq(capacity2), init]);
                    }

                    same_state = Bool::or(&[same_state, init.simplify()]);
                }
            }

            result.solver.assert(same_state);
            result
        }
    }
}
