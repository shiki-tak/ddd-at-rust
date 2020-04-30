use std::ops::Add;

use rust_decimal::Decimal;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Money {
    amount: Decimal,
    currency: Currency,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Currency {
    USD,
    JPY,
    EUR,
}

impl Currency {
    pub fn new(currency: &str) -> Option<Currency> {
        match currency {
            "USD" => Some(Currency::USD),
            "JPY" => Some(Currency::JPY),
            "EUR" => Some(Currency::EUR),
            _ => None,
        }
    }
}

impl Money {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self {
            amount: amount,
            currency: currency,
        }
    }
}

impl Add for Money {
    type Output = Money;

    fn add(self, other: Money) -> Self::Output {
        if self.currency != other.currency {
            panic!("invalid currency");
        }
        let new_amount = self.amount + other.amount;
        Money::new(new_amount, self.currency)
    }
}

#[test]
fn test_new_money() {
    let invalid_cnrrency = Currency::new("GBP");
    assert!(invalid_cnrrency.is_none());

    let valid_currency = Currency::new("JPY").unwrap();
    let money1 = Money::new(Decimal::new(1, 0), valid_currency);
    println!("{:?}", money1);
}

