use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Money {
    amount: i32,
    currency: Currency,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Currency {
    Dollar,
    Franc,
}

impl Money {
    pub fn dollar(amount: i32) -> Money {
        Money {
            amount,
            currency: Currency::Dollar,
        }
    }

    pub fn franc(amount: i32) -> Money {
        Money {
            amount,
            currency: Currency::Franc,
        }
    }
}

impl ops::Mul<i32> for Money {
    type Output = Money;

    fn mul(self, multiplier: i32) -> Self::Output {
        Money {
            amount: self.amount * multiplier,
            ..self
        }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Money) -> bool {
        self.amount == other.amount &&
            self.currency == other.currency
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);

        assert_eq!(five * 2, Money::dollar(10));
        assert_eq!(five * 3, Money::dollar(15));
    }

    #[test]
    fn test_equality() {
        assert!(Money::dollar(5) == Money::dollar(5));
        assert!(Money::dollar(5) != Money::dollar(6));

        assert!(Money::franc(5) == Money::franc(5));
        assert!(Money::franc(5) != Money::franc(6));

        assert!(Money::franc(5) != Money::dollar(5));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Money::franc(5);

        assert_eq!(five * 2, Money::franc(10));
        assert_eq!(five * 3, Money::franc(15));
    }
}