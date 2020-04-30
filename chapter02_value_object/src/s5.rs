use std::marker::PhantomData;
use std::ops::Add;

use rust_decimal::Decimal;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Money<T> {
    amount: Decimal,
    currency: PhantomData<T>,
}

impl<T> Money<T> {
    fn new(amount: Decimal) -> Self {
        Self {
            amount,
            currency: PhantomData::<T>,
        }
    }
}

impl<T> Add for Money<T> {
    type Output = Money<T>;

    fn add(self, other: Money<T>) -> Self::Output {
        Self::new(self.amount + other.amount)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JPY {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum USD {}

#[test]
fn test_phantom_money() {
    let jpy_1 = Money::<JPY>::new(Decimal::new(1, 0));
    let jpy_2 = Money::<JPY>::new(Decimal::new(2, 0));

    let _usd = Money::<USD>::new(Decimal::new(3, 0));

    let result = jpy_1 + jpy_2;
    assert_eq!(result, Money::<JPY>::new(Decimal::new(3, 0)));

    // let invalid_result = jpy_1 + usd;
}