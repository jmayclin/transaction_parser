use super::{Commodity, Lot, OptionType};

use std::error::Error;
use std::fs;

pub fn parse(filename: &str) -> Result<Vec<(Commodity, Lot)>, Box<dyn Error>> {
    let transactions: Vec<(Commodity, Lot)> = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse_line(line))
        .map(Result::unwrap)
        .collect();

    Ok(transactions)
}

fn price_parse(price_raw: &str) -> i32 {
    let price: Vec<&str> = price_raw.split(".").collect();
    let dollars = price[0].parse::<i32>().unwrap() * 100;
    let cents = if price.len() > 1 {
        if price[1].len() == 1 {
            price[1].parse::<i32>().unwrap() * 10
        } else {
            price[1].parse::<i32>().unwrap()
        }
    } else {
        0
    };
    dollars + cents
}

fn stock_parse(mut tokens: std::slice::Iter<&str>) -> Result<(Commodity, Lot), Box<dyn Error>> {
    tokens.next(); // consume id
    tokens.next(); // consume date
    let stock = Commodity::Stock(tokens.next().unwrap().to_string());
    let price = price_parse(tokens.next().unwrap());
    let quantity = tokens.next().unwrap().parse::<i32>().unwrap();
    let lot = Lot{
        price: price / (quantity.abs() as i32),
        quantity: quantity,
    };

    println!("{:?}", stock);
    println!("{:?}", lot);
    Ok((stock, lot))
}

fn option_parse(mut tokens: std::slice::Iter<&str>) -> Result<(Commodity, Lot), Box<dyn Error>> {
    tokens.next(); // consume id
    tokens.next(); // consume date
    let ticker = tokens.next().unwrap().to_string();
    let price = price_parse(tokens.next().unwrap());
    let quantity = tokens.next().unwrap().parse::<i32>().unwrap();
    let strike = price_parse(tokens.next().unwrap()) as u32;
    let contract;
    if tokens.next().unwrap().to_string() == "call" {
        contract = OptionType::Call;
    } else {
        contract = OptionType::Put;
    }
    let expiration = tokens.next().unwrap().to_string();

    Ok((
    Commodity::Option{
        ticker: ticker,
        contract:contract,
        strike:strike,
        expiration:expiration,
    },
    Lot{
        price: price/(quantity.abs() as i32),
        quantity: quantity,
    }
    ))
}

fn parse_line(line: &str) -> Result<(Commodity, Lot), Box<dyn Error>> {
    let tokens: Vec<&str> = line.split(",").collect();
    if tokens[6] == "" { // hacky way to detect stock vs options
        return stock_parse(tokens.iter());
    } else {
        return option_parse(tokens.iter());
    }
}
