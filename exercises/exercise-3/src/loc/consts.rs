//! This module is just to define constants that don't need to change in
//! between sub-questions. It defines constants for:
//!  - distance between locations,
//!  - capacity of villages,
//!  - IDs of locations.

// Capacities

/// The capacity of village `A`.
pub(super) static CAPACITY_A: u64 = 90;

/// The capacity of village `B`.
pub(super) static CAPACITY_B: u64 = 120;

/// The capacity of village `C`.
pub(super) static CAPACITY_C: u64 = 90;

/// The starting capacity of all villages.
pub(super) static VILLAGE_STARTING_CAPACITY: u64 = 60;

// Distances

/// The distance from `A` to `S`.
pub(super) static DISTANCE_A_S: u64 = 15;

/// The distance from `S` to `A`.
pub(super) static DISTANCE_S_A: u64 = 15;

/// The distance from `A` to `B`.
pub(super) static DISTANCE_A_B: u64 = 17;

/// The distance from `B` to `A`.
pub(super) static DISTANCE_B_A: u64 = 17;

/// The distance from `A` to `C`.
pub(super) static DISTANCE_A_C: u64 = 12;

/// The distance from `C` to `A`.
pub(super) static DISTANCE_C_A: u64 = 12;

/// The distance from `C` to `S`.
pub(super) static DISTANCE_C_S: u64 = 15;

/// The distance from `S` to `C`.
pub(super) static DISTANCE_S_C: u64 = 15;

/// The distance from `B` to `C`.
pub(super) static DISTANCE_B_C: u64 = 13;

/// The distance from `C` to `B`.
pub(super) static DISTANCE_C_B: u64 = 9;

/// The distance from a city to itself.
pub(super) static DISTANCE_SELF: u64 = 1;

// IDs

/// The id of the location of village `A`.
pub(super) static LOCATION_ID_A: u64 = 0;

/// The id of the location of village `B`.
pub(super) static LOCATION_ID_B: u64 = 1;

/// The id of the location of village `C`.
pub(super) static LOCATION_ID_C: u64 = 2;

/// The id of the location of supply `S`.
pub(super) static LOCATION_ID_S: u64 = 3;
