use ::std::collections::HashMap;
use ::std::fmt;
use ::z3::Optimize;
use ::z3::ast::Int;

/// The amount of `Nuzzles` required in total.
pub static REQUIRED_TOTAL_AMOUNT_OF_NUZZLES: i32 = 6;

/// The weight per `Nuzzles` in kilograms.
pub static WEIGHT_PER_OF_NUZZLE: i32 = 800;

/// The amount of `Prittles` required in total.
pub static REQUIRED_TOTAL_AMOUNT_OF_PRITTLES: i32 = 12;

/// The weight per `Prittle` in kilograms.
pub static WEIGHT_PER_OF_PRITTLES: i32 = 405;

/// The amount of `Skipples` required in total.
pub static REQUIRED_TOTAL_AMOUNT_OF_SKIPPLES: i32 = 15;

/// The weight per `Skipple` in kilograms.
pub static WEIGHT_PER_OF_SKIPPLES: i32 = 500;

/// The amount of `Crottles` required in total.
pub static REQUIRED_TOTAL_AMOUNT_OF_CROTTLES: i32 = 8;

/// The weight per `Crottle` in kilograms.
pub static WEIGHT_PER_OF_CROTTLES: i32 = 2_500;

/// The weight per `Dupple` in kilograms.
pub static WEIGHT_PER_OF_DUPPLES: i32 = 600;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Product {
    Nuzzles,
    Prittles,
    Skipples,
    Crottles,
    Dupples,
}

impl Product {
    /// The amount of this product that is required.
    pub fn required_amount(&self) -> Option<i32> {
        match self {
            Self::Nuzzles => Some(REQUIRED_TOTAL_AMOUNT_OF_NUZZLES),
            Self::Prittles => Some(REQUIRED_TOTAL_AMOUNT_OF_PRITTLES),
            Self::Skipples => Some(REQUIRED_TOTAL_AMOUNT_OF_SKIPPLES),
            Self::Crottles => Some(REQUIRED_TOTAL_AMOUNT_OF_CROTTLES),
            Self::Dupples => None,
        }
    }

    /// The weight of this product.
    pub fn weight(&self) -> i32 {
        match self {
            Self::Nuzzles => WEIGHT_PER_OF_NUZZLE,
            Self::Prittles => WEIGHT_PER_OF_PRITTLES,
            Self::Skipples => WEIGHT_PER_OF_SKIPPLES,
            Self::Crottles => WEIGHT_PER_OF_CROTTLES,
            Self::Dupples => WEIGHT_PER_OF_DUPPLES,
        }
    }

    /// Returns the name of the product.
    pub fn name(&self) -> String {
        format!("{}", self)
    }

    /// Iterates over all enum options.
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Nuzzles,
            Self::Prittles,
            Self::Skipples,
            Self::Crottles,
            Self::Dupples,
        ]
        .into_iter()
    }

    /// Goes over all products with a flexible amount and adds their totals to
    /// the solver.
    pub fn z3(optimizer: &Optimize) -> HashMap<Product, Int> {
        let products = Product::iter();
        products.fold(HashMap::new(), |mut result, product| {
            let name = format!("total_{product}");
            let total = Int::new_const(name);

            if let Some(amount) = product.required_amount() {
                optimizer.assert(total.eq(amount));
            }

            result.insert(product, total);
            result
        })
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nuzzles => write!(f, "nuzzles"),
            Self::Prittles => write!(f, "prittles"),
            Self::Skipples => write!(f, "skipples"),
            Self::Crottles => write!(f, "crottles"),
            Self::Dupples => write!(f, "dupples"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights_matches_statics() {
        assert_eq!(Product::Nuzzles.weight(), WEIGHT_PER_OF_NUZZLE);
        assert_eq!(Product::Prittles.weight(), WEIGHT_PER_OF_PRITTLES);
        assert_eq!(Product::Skipples.weight(), WEIGHT_PER_OF_SKIPPLES);
        assert_eq!(Product::Crottles.weight(), WEIGHT_PER_OF_CROTTLES);
        assert_eq!(Product::Dupples.weight(), WEIGHT_PER_OF_DUPPLES);
    }

    #[test]
    fn amounts_matches_statics() {
        assert_eq!(
            Product::Nuzzles.required_amount(),
            Some(REQUIRED_TOTAL_AMOUNT_OF_NUZZLES)
        );
        assert_eq!(
            Product::Prittles.required_amount(),
            Some(REQUIRED_TOTAL_AMOUNT_OF_PRITTLES)
        );
        assert_eq!(
            Product::Skipples.required_amount(),
            Some(REQUIRED_TOTAL_AMOUNT_OF_SKIPPLES)
        );
        assert_eq!(
            Product::Crottles.required_amount(),
            Some(REQUIRED_TOTAL_AMOUNT_OF_CROTTLES)
        );
        assert_eq!(Product::Dupples.required_amount(), None);
    }

    #[test]
    fn display_matches_lowercase() {
        assert_eq!(format!("{}", Product::Nuzzles), "nuzzles");
        assert_eq!(format!("{}", Product::Prittles), "prittles");
        assert_eq!(format!("{}", Product::Skipples), "skipples");
        assert_eq!(format!("{}", Product::Crottles), "crottles");
        assert_eq!(format!("{}", Product::Dupples), "dupples");
    }

    #[test]
    fn name_matches_lowercase() {
        assert_eq!(Product::Nuzzles.name(), "nuzzles");
        assert_eq!(Product::Prittles.name(), "prittles");
        assert_eq!(Product::Skipples.name(), "skipples");
        assert_eq!(Product::Crottles.name(), "crottles");
        assert_eq!(Product::Dupples.name(), "dupples");
    }

    #[test]
    fn thus_far_is_sat() {
        use ::z3::SatResult::Sat;
        let optimizer = Optimize::new();
        Product::z3(&optimizer);
        assert_eq!(optimizer.check(&[]), Sat);
        let model = optimizer.get_model();
        assert!(model.is_some());
    }
}
