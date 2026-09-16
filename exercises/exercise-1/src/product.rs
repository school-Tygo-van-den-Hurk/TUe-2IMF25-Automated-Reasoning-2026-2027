use ::std::collections::HashMap;
use ::std::fmt;
use ::z3::Optimize;
use ::z3::ast::Int;
use ::strum::IntoEnumIterator;
use ::strum_macros::EnumIter;

/// The weight per `Dupple` in kilograms.
pub static WEIGHT_PER_DUPPLES: i32 = 600;

/// The amount of `Nuzzles` required in total.
pub static REQUIRED_TOTAL_AMOUNT_OF_NUZZLES: i32 = 6;

/// The weight per `Nuzzles` in kilograms.
pub static WEIGHT_PER_NUZZLE: i32 = 800;

/// The amount of `Prittles` required in total.
pub static REQUIRED_TOTAL_AMOUNT_OF_PRITTLES: i32 = 12;

/// The weight per `Prittle` in kilograms.
pub static WEIGHT_PER_PRITTLES: i32 = 405;

/// The amount of `Skipples` required in total.
pub static REQUIRED_TOTAL_AMOUNT_OF_SKIPPLES: i32 = 15;

/// The weight per `Skipple` in kilograms.
pub static WEIGHT_PER_SKIPPLES: i32 = 500;

/// The amount of `Crottles` required in total.
pub static REQUIRED_TOTAL_AMOUNT_OF_CROTTLES: i32 = 8;

/// The weight per `Crottle` in kilograms.
pub static WEIGHT_PER_CROTTLES: i32 = 2_500;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, EnumIter)]
pub enum Product {
    Dupples,
    Nuzzles,
    Prittles,
    Skipples,
    Crottles,
}

impl Product {
    /// The amount of this product that is required.
    pub fn required_amount(&self) -> Option<i32> {
        match self {
            Self::Dupples => None,
            Self::Nuzzles => Some(REQUIRED_TOTAL_AMOUNT_OF_NUZZLES),
            Self::Prittles => Some(REQUIRED_TOTAL_AMOUNT_OF_PRITTLES),
            Self::Skipples => Some(REQUIRED_TOTAL_AMOUNT_OF_SKIPPLES),
            Self::Crottles => Some(REQUIRED_TOTAL_AMOUNT_OF_CROTTLES),
        }
    }

    /// The weight of this product.
    pub fn weight(&self) -> i32 {
        match self {
            Self::Dupples => WEIGHT_PER_DUPPLES,
            Self::Nuzzles => WEIGHT_PER_NUZZLE,
            Self::Prittles => WEIGHT_PER_PRITTLES,
            Self::Skipples => WEIGHT_PER_SKIPPLES,
            Self::Crottles => WEIGHT_PER_CROTTLES,
        }
    }

    /// Returns the name of the product.
    pub fn name(&self) -> String {
        format!("{}", self)
    }

    /// Iterates over all enum options.
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
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
            Self::Dupples => write!(f, "dupples"),
            Self::Nuzzles => write!(f, "nuzzles"),
            Self::Prittles => write!(f, "prittles"),
            Self::Skipples => write!(f, "skipples"),
            Self::Crottles => write!(f, "crottles"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Product::*;
    use super::*;

    #[test]
    fn weights_matches_statics() {
        assert_eq!(Dupples.weight(), WEIGHT_PER_DUPPLES);
        assert_eq!(Nuzzles.weight(), WEIGHT_PER_NUZZLE);
        assert_eq!(Prittles.weight(), WEIGHT_PER_PRITTLES);
        assert_eq!(Skipples.weight(), WEIGHT_PER_SKIPPLES);
        assert_eq!(Crottles.weight(), WEIGHT_PER_CROTTLES);
    }

    #[test]
    fn amounts_matches_statics() {
        assert_eq!(Dupples.required_amount(), None);

        assert_eq!(
            Nuzzles.required_amount(),
            Some(REQUIRED_TOTAL_AMOUNT_OF_NUZZLES)
        );

        assert_eq!(
            Prittles.required_amount(),
            Some(REQUIRED_TOTAL_AMOUNT_OF_PRITTLES)
        );

        assert_eq!(
            Skipples.required_amount(),
            Some(REQUIRED_TOTAL_AMOUNT_OF_SKIPPLES)
        );

        assert_eq!(
            Crottles.required_amount(),
            Some(REQUIRED_TOTAL_AMOUNT_OF_CROTTLES)
        );
    }

    #[test]
    fn display_matches_lowercase() {
        assert_eq!(format!("{}", Dupples), "dupples");
        assert_eq!(format!("{}", Nuzzles), "nuzzles");
        assert_eq!(format!("{}", Prittles), "prittles");
        assert_eq!(format!("{}", Skipples), "skipples");
        assert_eq!(format!("{}", Crottles), "crottles");
    }

    #[test]
    fn name_matches_lowercase() {
        assert_eq!(Dupples.name(), "dupples");
        assert_eq!(Nuzzles.name(), "nuzzles");
        assert_eq!(Prittles.name(), "prittles");
        assert_eq!(Skipples.name(), "skipples");
        assert_eq!(Crottles.name(), "crottles");
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
