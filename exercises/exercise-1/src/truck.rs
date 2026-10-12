use crate::product::Product;
use ::std::collections::HashMap;
use ::z3::Optimize;
use ::z3::ast::Ast;
use ::z3::ast::Int;

#[derive(PartialEq, Eq, Debug)]
pub struct Truck {
    pub amounts: HashMap<Product, Int>,
    pub weight: Int,
}

impl Truck {
    /// The amount of trucks available.
    pub const AMOUNT: usize = 6;

    /// The weight a truck can transport.
    pub const MAX_WEIGHT: i32 = 8_000;

    pub fn indexes() -> impl Iterator<Item = usize> {
        0..Self::AMOUNT
    }

    pub fn z3(optimizer: &Optimize) -> Vec<Truck> {
        Self::indexes().fold(Vec::new(), |mut result, index| {
            let name = format!("total_weight_truck_{index}");
            let weight = Int::new_const(name);

            let mut calculated = Int::from_u64(0);
            let amounts = Product::iter().fold(HashMap::new(), |mut map, product| {
                let name = format!("total_{product}_truck_{index}");
                let total = Int::new_const(name);
                optimizer.assert(total.ge(0));
                calculated += &total * product.weight();
                map.insert(product, total);
                map
            });

            // Trucks can carry between 0 and `Truck::MAX_WEIGHT` kilograms
            optimizer.assert(weight.le(Truck::MAX_WEIGHT));
            optimizer.assert(weight.eq(calculated.simplify()));
            optimizer.assert(weight.ge(0));

            // Trucks can carry at most 10 blocks
            let init = Int::from_u64(0);
            let iter = amounts.values();
            let sum = iter.fold(init, |sum, product| sum + product);
            let sum = sum.simplify();
            optimizer.assert(sum.le(10));

            result.push(Self { amounts, weight });
            result
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thus_far_is_sat() {
        use ::z3::SatResult::Sat;
        let optimizer = Optimize::new();
        Truck::z3(&optimizer);
        assert_eq!(optimizer.check(&[]), Sat);
        let model = optimizer.get_model();
        assert!(model.is_some());
    }
}
