use std::ops;
use std::hash;
use std::collections::HashMap;

pub trait Expression {
    fn reduce(&self, bank: &Bank, to: Currency) -> Money;
}

#[derive(Copy, Clone, Debug)]
struct Sum {
    augend: Money,
    addend: Money,
}

impl Expression for Sum {
    fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        let a = self.augend.reduce(bank, to);
        let b = self.addend.reduce(bank, to);

        Money {
            amount: a.amount + b.amount,
            currency: to,
        }
    }
}

pub struct Bank {
    rates: HashMap<Pair, i32>,
}

impl Bank {
    fn new() -> Bank {
        Bank { rates: HashMap::new() }
    }

    fn reduce(&self, expr: Box<Expression>, to: Currency) -> Money {
        expr.reduce(self, to)
    }

    fn add_rate(&mut self, from: Currency, to: Currency, rate: i32) {
        let pair = Pair { from, to };
        self.rates.insert(pair, rate);
    }

    fn rate(&self, from: Currency, to: Currency) -> i32 {
        if from == to {
            1
        } else {
            *self.rates.get(&Pair { from, to }).unwrap()
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Money {
    amount: i32,
    currency: Currency,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Currency {
    Dollar,
    Franc,
}

impl Money {
    fn new(amount: i32, currency: Currency) -> Money {
        Money {
            amount,
            currency,
        }
    }

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

impl Expression for Money {
    fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        let rate = bank.rate(self.currency, to);
        Money {
            amount: self.amount / rate,
            currency: to,
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

impl ops::Add for Money {
    type Output = Box<Expression>;

    fn add(self, other: Money) -> Self::Output {
        Box::new(
            Sum {
                augend: self,
                addend: other,
            }
        )
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

#[derive(Copy, Clone, Debug, Eq)]
struct Pair {
    from: Currency,
    to: Currency,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Pair) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl hash::Hash for Pair {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {

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
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(6));

        assert_ne!(Money::franc(5), Money::dollar(5));
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

    #[test]
    fn test_reduce_sum() {
        let bank = Bank::new();
        let sum = Money::dollar(3) + Money::dollar(4);
        let result = bank.reduce(sum, Currency::Dollar);
        assert_eq!(result, Money::dollar(7));
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);

        let result = bank.reduce(Box::new(Money::franc(2)), Currency::Dollar);
        assert_eq!(result, Money::dollar(1));
    }

    #[test]
    fn test_identity_rate() {
        assert_eq!(Bank::new().rate(Currency::Dollar, Currency::Dollar), 1);
    }
}