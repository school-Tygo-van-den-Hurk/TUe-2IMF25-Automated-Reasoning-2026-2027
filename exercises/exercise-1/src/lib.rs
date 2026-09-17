pub mod product;
pub mod truck;

use ::std::collections::HashMap;
use ::std::iter::FusedIterator;
use ::z3::Optimize;
use ::z3::ast::Ast;
use ::z3::ast::Bool;
use ::z3::ast::Int;
use ::z3::ast::atleast as at_least;

use crate::product::Product;
use crate::product::Product::*;
use crate::truck::Truck;

pub struct Exercise {
    pub products: HashMap<Product, Int>,
    pub trucks: Vec<Truck>,
    pub optimizer: Optimize,
}

impl Exercise {
    pub fn a() -> Self {
        let optimizer = Optimize::new();

        let trucks = Truck::z3(&optimizer);
        let products = Product::z3(&optimizer);
        optimizer.maximize(&products[&Dupples]);

        // only 2 trucks have facilities to house `Skipples`.
        for truck in trucks.iter().skip(2) {
            let product = &truck.amounts[&Skipples];
            optimizer.assert(product.eq(0));
        }

        // Prittles need to be distributed over at least five trucks.
        let prittles = trucks.iter().map(|truck| &truck.amounts[&Prittles]);
        let carries_prittles = prittles.map(|amount| amount.ge(1));
        let carries_prittles = carries_prittles.collect::<Vec<Bool>>();
        optimizer.assert(at_least(&carries_prittles, 5));

        // The total amount of each product being transported it the sum of the
        // amounts in trucks
        for product in Product::iter() {
            let total_amount = &products[&product];

            let init = Int::from_u64(0);
            let iter = trucks.iter();
            let sum_amounts_trucks = iter.fold(init, |sum, truck| {
                let product_amount = &truck.amounts[&product];
                sum + product_amount
            });

            let simplified = sum_amounts_trucks.simplify();
            optimizer.assert(total_amount.eq(simplified));
        }

        Self {
            products,
            trucks,
            optimizer,
        }
    }

    pub fn b() -> Self {
        let exercise = Exercise::a();

        // crottles get too cold if there are less than two dupples in the same
        // truck.
        for truck in exercise.trucks.iter() {
            let crottles = &truck.amounts[&Crottles];
            let dupples = &truck.amounts[&Dupples];
            let condition = crottles.gt(0);
            let assertion = condition.implies(dupples.ge(2));
            exercise.optimizer.assert(assertion);
        }

        exercise
    }

    pub fn solutions(self) -> impl FusedIterator<Item = Vec<Int>> {
        let mut variables = Vec::new();

        variables.extend(self.products.into_values());

        for truck in self.trucks {
            variables.extend(truck.amounts.into_values());
            variables.push(truck.weight);
        }

        let model_completion = false;
        self.optimizer.into_solutions(variables, model_completion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::z3::SatResult::Sat;

    #[test]
    fn model_a_is_sat() {
        let exercise = Exercise::a();
        assert_eq!(exercise.optimizer.check(&[]), Sat);
        let model = exercise.optimizer.get_model();
        assert!(model.is_some());
    }

    #[test]
    fn model_b_is_sat() {
        let exercise = Exercise::b();
        assert_eq!(exercise.optimizer.check(&[]), Sat);
        let model = exercise.optimizer.get_model();
        assert!(model.is_some());
    }
}
