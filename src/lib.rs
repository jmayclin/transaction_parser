
pub mod parser;
pub mod analyzer;

use std::fmt;

pub const STARTING_BALANCE: i32 = 3282000;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}/{:02}/{:02}", self.month, self.day, self.year)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Settlement {
    pub open: Date,
    pub close: Date,
    pub commodity: Commodity,
    pub quantity: i32,
    pub profit: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Lot {
    pub date: Date,
    pub price: i32, // price in cents
    pub quantity: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum OptionType {
    Call,
    Put,
}
impl fmt::Display for OptionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if *self == OptionType::Call {
            write!(f, "call")
        } else {
            write!(f, "put")
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Commodity {
    Stock(String),
    Option {
        ticker: String,
        contract: OptionType,
        strike: u32, //price in cents
        expiration: String,
    },
}
impl fmt::Display for Commodity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Commodity::Option{ticker:t, contract: c, strike: s, expiration: e} => write!(f, "{}_{}_{}.{}_{}", t, c, s / 10000, s % 10000 / 100, e),
            Commodity::Stock(ticker) => write!(f, "{}", ticker),
        }
    }
}

impl Commodity {
    fn ticker(&self) -> &str {
        match self {
            Commodity::Option{ticker:t, contract: _, strike: _, expiration: _} => t,
            Commodity::Stock(t) => t,
        }
    }

    fn same_instrument(&self, other: &Commodity) -> bool {
        let instrument = match self {
            Commodity::Option{ticker:_, contract: _, strike: _, expiration: _} => "option",
            Commodity::Stock(_) => "stock",
        };
        let instrument_other = match other {
            Commodity::Option{ticker:_, contract: _, strike: _, expiration: _} => "option",
            Commodity::Stock(_) => "stock",
        };
        instrument == instrument_other
    }
}
