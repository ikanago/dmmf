use std::num::NonZeroUsize;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum OrderQuantityError {
    #[error("unit quantity must be positive")]
    UnitQuantityNonPositive,
    #[error("mass quantity must be positive")]
    KilogramsQuantityNonPositive,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitQuantity(NonZeroUsize);

impl UnitQuantity {
    pub fn new(value: usize) -> Result<UnitQuantity, OrderQuantityError> {
        NonZeroUsize::new(value)
            .ok_or(OrderQuantityError::UnitQuantityNonPositive)
            .map(UnitQuantity)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KilogramsQuantity(f64);

impl KilogramsQuantity {
    pub fn new(value: f64) -> Result<KilogramsQuantity, OrderQuantityError> {
        if value > 0.0 {
            Ok(KilogramsQuantity(value))
        } else {
            Err(OrderQuantityError::KilogramsQuantityNonPositive)
        }
    }
}

pub enum OrderQuantity {
    UnitQuantity(UnitQuantity),
    KilogramsQuantity(KilogramsQuantity),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_unit_quantity() {
        assert_eq!(
            UnitQuantity::new(42).unwrap(),
            UnitQuantity(NonZeroUsize::new(42).unwrap())
        );
    }

    #[test]
    fn test_create_unit_quantity_with_zero() {
        assert_eq!(
            UnitQuantity::new(0),
            Err(OrderQuantityError::UnitQuantityNonPositive)
        )
    }

    #[test]
    fn test_create_kilograms_quantity() {
        assert_eq!(
            KilogramsQuantity::new(1.23).unwrap(),
            KilogramsQuantity(1.23)
        )
    }
}
