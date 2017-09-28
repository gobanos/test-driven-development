use std::ops;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Currency {
    Dollar,
    Franc,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Money {
    amount: i32,
    currency: Currency,
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

#[derive(Debug)]
pub enum Expression {
    Money(Money),
    Sum(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn reduce(self, bank: &Bank, to: Currency) -> Money {
        match self {
            Expression::Money(Money { amount, currency: from }) => {
                let rate = bank.rate(from, to);
                Money { amount: amount / rate, currency: to }
            }
            Expression::Sum(augend, addend) => {
                let Money { amount: augend, .. } = augend.reduce(bank, to);
                let Money { amount: addend, .. } = addend.reduce(bank, to);
                Money { amount: augend + addend, currency: to }
            }
        }
    }

    pub fn dollar(amount: i32) -> Expression {
        Money::dollar(amount).into()
    }

    pub fn franc(amount: i32) -> Expression {
        Money::franc(amount).into()
    }
}

impl From<Money> for Expression {
    fn from(money: Money) -> Self {
        Expression::Money(money)
    }
}

impl ops::Mul<i32> for Expression {
    type Output = Expression;

    fn mul(self, multiplier: i32) -> Self::Output {
        match self {
            Expression::Money(Money { amount, currency }) => {
                Expression::Money(Money { amount: amount * multiplier, currency })
            },
            Expression::Sum(augend, addend) => {
                Expression::Sum(
                    Box::new(*augend * multiplier),
                    Box::new(*addend * multiplier)
                )
            }
        }
    }
}

impl ops::Add for Expression {
    type Output = Expression;

    fn add(self, other: Expression) -> Self::Output {
        match (&self, &other) {
            (&Expression::Money(augend), &Expression::Money(addend)) if augend.currency == addend.currency => {
                Money { amount: augend.amount + addend.amount, currency: augend.currency }.into()
            },
            _ => {
                Expression::Sum(
                    Box::new(self),
                    Box::new(other)
                )
            }
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Expression) -> bool {
        if let (&Expression::Money(a), &Expression::Money(b)) = (self, other) {
            a == b
        } else {
            false
        }
    }
}

pub struct Bank {
    rates: HashMap<(Currency, Currency), i32>,
}

impl Bank {
    pub fn new() -> Bank {
        Bank { rates: HashMap::new() }
    }

    pub fn reduce(&self, expr: Expression, to: Currency) -> Money {
        expr.reduce(self, to)
    }

    pub fn add_rate(&mut self, from: Currency, to: Currency, rate: i32) {
        self.rates.insert((from, to), rate);
    }

    pub fn rate(&self, from: Currency, to: Currency) -> i32 {
        if from == to {
            1
        } else {
            *self.rates.get(&(from, to)).unwrap()
        }
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
    fn multiplication() {
        let five = Money::dollar(5);

        assert_eq!(Expression::from(five) * 2, Expression::dollar(10));
        assert_eq!(Expression::from(five) * 3, Expression::dollar(15));
    }

    #[test]
    fn equality() {
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(6));

        assert_ne!(Money::franc(5), Money::dollar(5));
    }

    #[test]
    fn currency() {
        check_currency(Money::dollar(1).currency, "USD");
        check_currency(Money::franc(1).currency, "CHF");
    }

    fn check_currency(currency: Currency, str_value: &str) {
        assert_eq!(currency.to_string(), str_value.to_string());
    }

    #[test]
    fn simple_addition() {
        let bank = Bank::new();
        let sum = Expression::dollar(5) + Expression::dollar(5);

        let reduced = bank.reduce(sum, Currency::Dollar);
        assert_eq!(reduced, Money::dollar(10));
    }

    #[test]
    fn reduce_sum() {
        let bank = Bank::new();
        let sum = Expression::dollar(3) + Expression::dollar(4);
        let result = bank.reduce(sum, Currency::Dollar);
        assert_eq!(result, Money::dollar(7));
    }

    #[test]
    fn reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);

        let result = bank.reduce(Expression::franc(2), Currency::Dollar);
        assert_eq!(result, Money::dollar(1));
    }

    #[test]
    fn identity_rate() {
        assert_eq!(Bank::new().rate(Currency::Dollar, Currency::Dollar), 1);
    }

    #[test]
    fn mixed_addition() {
        let five_bucks = Expression::dollar(5);
        let ten_francs = Expression::franc(10);

        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);

        let result = bank.reduce(five_bucks + ten_francs, Currency::Dollar);
        assert_eq!(result, Money::dollar(10));
    }

    #[test]
    fn sum_plus_money() {
        let five_bucks = Expression::dollar(5);
        let ten_francs = Expression::franc(10);

        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);

        let sum = five_bucks + ten_francs + Expression::dollar(5);
        let result = bank.reduce(sum, Currency::Dollar);

        assert_eq!(result, Money::dollar(15));
    }

    #[test]
    fn sum_times() {
        let five_bucks = Expression::dollar(5);
        let ten_francs = Expression::franc(10);

        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Dollar, 2);

        let sum = (five_bucks + ten_francs) * 2;
        let result = bank.reduce(sum, Currency::Dollar);

        assert_eq!(result, Money::dollar(20));
    }

    #[test]
    fn plus_same_currency_returns_money() {
        let sum = Expression::dollar(1) + Expression::dollar(1);
        if let Expression::Money(_) = sum {

        } else {
            panic!("Expected Money, got {:?}", sum);
        }
    }
}