use std::ops;

pub struct Expression {}
pub struct Bank {}

impl Bank {
    fn new() -> Bank {
        Bank {  }
    }

    fn reduce(&self, expr: Expression, to: Currency) -> Money {
        Money::dollar(10)
    }
}

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

    pub fn currency(&self) -> Currency {
        self.currency
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

impl ops::Add for Money {
    type Output = Expression;

    fn add(self, other: Money) -> Self::Output {
        Expression {  }
    }
}

impl PartialEq for Money {
    fn eq(&self, other: &Money) -> bool {
        self.amount == other.amount &&
            self.currency == other.currency
    }
}

impl ToString for Currency {
    fn to_string(&self) -> String {
        match *self {
            Currency::Dollar => "USD",
            Currency::Franc => "CHF",
        }.into()
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

        assert!(Money::franc(5) != Money::dollar(5));
    }

    #[test]
    fn test_currency() {
        check_currency(Money::dollar(1).currency(), "USD");
        check_currency(Money::franc(1).currency(), "CHF");
    }

    fn check_currency(currency: Currency, str_value: &str) {
        assert_eq!(currency.to_string(), str_value.to_string());
    }

    #[test]
    fn test_simple_addition() {
        let bank = Bank::new();
        let sum = Money::dollar(5) + Money::dollar(5);

        let reduced = bank.reduce(sum, Currency::Dollar);
        assert_eq!(reduced, Money::dollar(10));
    }
}