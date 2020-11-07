
pub mod parser;
pub mod analyzer;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Lot {
    pub price: i32, // price in cents
    pub quantity: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u8,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum OptionType {
    Call,
    Put,
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