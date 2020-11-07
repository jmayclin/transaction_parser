
use super::{Lot, Date, Commodity, OptionType};

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

fn date_parse(date_raw: &str) -> Date {
    let date_raw: Vec<&str> = date_raw.split("/").collect();
    let month = date_raw[0].parse::<u8>().unwrap();
    let day = date_raw[1].parse::<u8>().unwrap();
    let year = date_raw[2].parse::<u8>().unwrap();
    Date{
        month: month,
        day: day,
        year: year,
    }
}

fn price_parse(price_raw: &str) -> i32 {
    let price: Vec<&str> = price_raw.split(".").collect();
    let dollars = price[0].parse::<i32>().unwrap() * 100 * 100;
    let cents = if price.len() > 1 {
        if price[1].len() == 1 {
            price[1].parse::<i32>().unwrap() * 10 * 100
        } else {
            price[1].parse::<i32>().unwrap() * 100
        }
    } else {
        0
    };
    dollars + cents
}

fn stock_parse(mut tokens: std::slice::Iter<&str>) -> Result<(Commodity, Lot), Box<dyn Error>> {
    tokens.next(); // consume id
    let date = date_parse(tokens.next().unwrap()); // consume date
    let stock = Commodity::Stock(tokens.next().unwrap().to_string());
    let price = price_parse(tokens.next().unwrap());
    let quantity = tokens.next().unwrap().parse::<i32>().unwrap();
    let lot = Lot{
        date: date,
        price: price / (quantity.abs() as i32),
        quantity: quantity,
    };

    println!("{:?}", stock);
    println!("{:?}", lot);
    Ok((stock, lot))
}

fn option_parse(mut tokens: std::slice::Iter<&str>) -> Result<(Commodity, Lot), Box<dyn Error>> {
    tokens.next(); // consume id
    let date = date_parse(tokens.next().unwrap()); // consume date
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
        date: date,
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

// test for parsing price
// test parsing stock

// test parsing option

#[cfg(test)]
mod tests {
    use super::*;

    fn type_commodity(com: &Commodity) -> &str {
        match com {
            Commodity::Option{ticker:_, contract: _, strike: _, expiration: _} => "option",
            Commodity::Stock(_) => "stock",
        }
    }

    #[test]
    fn check_type_utility() {
        let stock = Commodity::Stock(String::from("UAA"));
        assert_eq!(type_commodity(&stock), "stock");

        let option = Commodity::Option {
            ticker: String::from("UAA"),
            contract: OptionType::Call,
            strike: 183,
            expiration: String::from("10/30/20"),
        };
        assert_eq!(type_commodity(&option), "option");

    }

    #[test]
    fn parse_stock() {
        let stock_example = String::from(r"1,10/29/20,UAA,1367.77,100,,,");
        let (commodity, lot) = parse_line(&stock_example).unwrap();
        assert_eq!(type_commodity(&commodity), "stock");
        if let Commodity::Stock(t) = commodity {
            assert_eq!(t, "UAA");
        } else {
            panic!("ticker not read correctly");
        }
        assert_eq!(lot.quantity, 100);
        assert_eq!(lot.price, 136777);
        assert_eq!(lot.date, Date{day: 29, month: 10, year: 20});
    }

    #[test]
    fn parse_option() {
        let option_example = String::from(r"9,10/30/20,GE,90,-1,6.5,call,11/06/20");
        let (commodity, lot) = parse_line(&option_example).unwrap();
        assert_eq!(type_commodity(&commodity), "option");
        if let Commodity::Option{ticker:t, contract: c, strike: s, expiration: e} = commodity {
            assert_eq!(t, "GE");
            assert_eq!(c, OptionType::Call);
            assert_eq!(s, 65000);
            assert_eq!(e, "11/06/20");
        } else {
            panic!("commodity not read correctly");
        }
        assert_eq!(lot.quantity, -1);
        assert_eq!(lot.price, 900000);
        assert_eq!(lot.date, Date{day: 30, month: 10, year: 20});
    }
}