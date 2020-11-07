use std::error::Error;
use std::process;
use std::slice::Iter;
use std::collections::HashMap;

use transaction_parser::{Commodity, OptionType, Lot};

mod parser;

use transaction_parser::parser::parse;
use transaction_parser::analyzer::profit_loss;

fn main() {
    println!("Hello, world!");
    let an_option = Commodity::Option {
        ticker: String::from("UAA"),
        contract: OptionType::Call,
        strike: 183,
        expiration: String::from("10/30/20"),
    };

    let transactions = parse("trade_history.csv").unwrap();
    println!("{:?}", transactions);
    profit_loss(transactions);

}


