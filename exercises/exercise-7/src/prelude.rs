//! Just a bunch of functions I don't wanna have to import everywhere, so I put
//! them here so I can import this instead.

#![allow(unused_imports)]

pub(crate) use ::itertools::Itertools;
pub(crate) use ::std::collections::HashMap;
pub(crate) use ::z3::Optimize;
pub(crate) use ::z3::SatResult::*;
pub(crate) use ::z3::Solver;
pub(crate) use ::z3::ast::Bool;
pub(crate) use ::z3::ast::Int;
pub(crate) use ::z3::ast::atleast as at_least;
pub(crate) use ::z3::ast::atmost as at_most;
pub(crate) use z3::ast::Ast;
