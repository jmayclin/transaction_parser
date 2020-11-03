use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::slice::Iter;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Lot {
    price: u32, // price in cents
    quantity: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct Date {
    day: u8,
    month: u8,
    year: u8,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum OptionType {
    Call,
    Put,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Commodity {
    Stock(String),
    Option {
        ticker: String,
        contract: OptionType,
        strike: u32, //price in cents
        expiration: String,
    },
}

fn main() {
    println!("Hello, world!");
    let an_option = Commodity::Option {
        ticker: String::from("UAA"),
        contract: OptionType::Call,
        strike: 183,
        expiration: String::from("10/30/20"),
    };
    println!("{:?}", an_option);
    if let Err(e) = run() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn price_parse(price_raw: &str) -> u32 {
    let price: Vec<&str> = price_raw.split(".").collect();
    let mut cents = price[0].parse::<u32>().unwrap() * 100;
    if price.len() > 1 {
        if price[1].len() == 1 {
            cents += price[1].parse::<u32>().unwrap() * 10;
        } else {
            cents += price[1].parse::<u32>().unwrap();
        }
    }
    cents
}

fn stock_parse(mut tokens: std::slice::Iter<&str>) -> Result<(Commodity, Lot), Box<dyn Error>> {
    tokens.next(); // consume id
    tokens.next(); // consume date
    let stock = Commodity::Stock(tokens.next().unwrap().to_string());
    let price = price_parse(tokens.next().unwrap());
    let quantity = tokens.next().unwrap().parse::<i32>().unwrap();
    let lot = Lot{
        price: price/(quantity.abs() as u32),
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
    let strike = price_parse(tokens.next().unwrap());
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
        price: price/(quantity.abs() as u32),
        quantity: quantity,
    }
    ))
}

fn parse(line: &str) -> Result<(Commodity, Lot), Box<dyn Error>> {
    let tokens: Vec<&str> = line.split(",").collect();
    if tokens[6] == "" { // hacky way to detect stock vs options
        return stock_parse(tokens.iter());
    } else {
        return option_parse(tokens.iter());
    }
}

fn option_return(transactions: std::vec::IntoIter<(Commodity, Lot)>) {
    // for calls, options return on collateral and collateral return
    // for puts, options return on cash coverage of put
}

fn profit_loss(transactions: std::vec::IntoIter<(Commodity, Lot)>) -> i32 {
    let mut book: HashMap<Commodity, Vec<Lot>> = HashMap::new();
    let mut profit = 0;
    for (commodity,mut lot) in transactions {
        println!("Inserting {:?} into book", &commodity);
        //println!("{:?}", book);
        let entry = book.entry(commodity.clone()).or_default();
        while entry.len() > 0 && ((lot.quantity > 0) != (entry[0].quantity > 0)) && lot.quantity != 0 {
            let settlement = cmp::max(entry[0].quantity.abs(), lot.quantity.abs());
            let buy_price = if lot.quantity > 0 {
                lot.price
            } else {
                entry[0].price
            };
            let sell_price = if lot.quantity < 0 {
                lot.price
            } else {
                entry[0].price
            };
            println!("\tSettling {} shares of {:?} for profit of {} dollars", settlement, &commodity, settlement * (sell_price as i32 - buy_price as i32)/100);
            profit += settlement * (sell_price as i32 - buy_price as i32);
            println!("\tNew profit is {}", profit);
            entry[0].quantity -= settlement;
            lot.quantity -= settlement;
            if entry[0].quantity == 0 {
                entry.remove(0);
            }
        }
        if lot.quantity.abs() != 0 {
            println!("REMAINING");
            entry.push(lot);
        }
        println!("\tBook for {:?} is {:?}", &commodity, &entry);
    }
    profit
}

fn run() -> Result<(), Box<dyn Error>> {
    let filename = String::from("trade_history.csv");
    let transactions: Vec<(Commodity, Lot)> = fs::read_to_string(filename)?
        .lines()
        .map(|line| parse(line))
        .map(Result::unwrap)
        .collect();

    profit_loss(transactions.into_iter());


    Ok(())
}
