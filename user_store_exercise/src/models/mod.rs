use serde::{Deserialize, Serialize};
use std::fmt::Display;

const MULTIPLIER_100: i32 = 100;
const MULTIPLIER_100F: f32 = 100.0;

#[derive(Serialize, Deserialize, Clone)]
enum UserType {
    User,
    Admin,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    password: String,
    full_name: String,
    pub balance: Money,
    kind: UserType,
}

impl User {
    pub fn new(full_name: String, balance: Money) -> Self {
        User {
            id: xid::new().to_string(),
            full_name: full_name.trim().into(),
            balance,
            username: full_name
                .trim()
                .to_string()
                .replace(" ", ".")
                .to_lowercase(),
            password: "qwerty".into(),
            kind: UserType::User,
        }
    }

    pub fn check_password(&self, password: String) -> bool {
        self.password == password
    }

    pub fn change_password(&mut self, new_password: String) {
        self.password = new_password;
    }

    pub fn deposit(&mut self, amount: Money) {
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: Money) {
        self.balance -= amount;
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "u({}): {} - {}", self.id, self.full_name, self.balance)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Money {
    val: i32,
    pub currency: Currency,
}

impl Money {
    pub fn new(val: i32, currency: Currency) -> Self {
        Money { val, currency }
    }

    pub fn from_float(val: f32, currency: Currency) -> Self {
        Money {
            val: (val.trunc() as i32 * MULTIPLIER_100) + (val.fract() * MULTIPLIER_100F) as i32,
            currency,
        }
    }

    pub fn from_unit_fract(unit: i32, fractional: i32, currency: Currency) -> Self {
        Money {
            val: unit * MULTIPLIER_100 + fractional,
            currency,
        }
    }

    pub fn from_unit(unit: i32, currency: Currency) -> Self {
        Self::from_unit_fract(unit, 0, currency)
    }

    fn value(&self) -> f32 {
        self.val as f32 / 100.0
    }
}

impl std::ops::Add<Money> for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if self.currency != rhs.currency {
            panic!("Cannot add two different currencies");
        }

        Money {
            val: self.val + rhs.val,
            currency: self.currency,
        }
    }
}

impl std::ops::AddAssign<Money> for Money {
    fn add_assign(&mut self, rhs: Money) {
        self.val = self.val + rhs.val;
    }
}

impl std::ops::SubAssign<Money> for Money {
    fn sub_assign(&mut self, rhs: Money) {
        self.val = self.val - rhs.val;
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.currency {
            Currency::Dollar => write!(f, "{}{:.2}", self.currency, self.value()),
            Currency::Pound => write!(f, "{}{:.2}", self.currency, self.value()),
            Currency::Euro => write!(f, "{:.2}{}", self.value(), self.currency),
        }
    }
}

#[test]
fn test_money_from_float() {
    let val = 3.2;
    let res = Money::from_float(val, Currency::Dollar);
    assert_eq!(320, res.val);
}

#[test]
fn test_money_from_unit_fract() {
    let money = Money::from_unit_fract(10, 25, Currency::Pound);
    assert_eq!("£10.25", format!("{}", money));
    let money_full = Money::from_unit(10, Currency::Pound);
    assert_eq!("£10.00", format!("{}", money_full));
}

#[test]
fn test_money_to_str() {
    let dollar = Money::new(100, Currency::Dollar);
    let euro = Money::new(100, Currency::Euro);
    let pound = Money::new(100, Currency::Pound);

    let more_pounds = Money::new(15025, Currency::Pound);

    assert_eq!("$1.00", format!("{}", dollar));
    assert_eq!("1.00€", format!("{}", euro));
    assert_eq!("£1.00", format!("{}", pound));
    assert_eq!("£150.25", format!("{}", more_pounds));
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum Currency {
    Dollar,
    Euro,
    Pound,
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Currency::Dollar => "$",
            Currency::Euro => "€",
            Currency::Pound => "£",
        };
        write!(f, "{}", str)
    }
}

#[test]
fn test_currency_to_str() {
    assert_eq!("$", format!("{}", Currency::Dollar));
    assert_eq!("€", format!("{}", Currency::Euro));
    assert_eq!("£", format!("{}", Currency::Pound));
}
