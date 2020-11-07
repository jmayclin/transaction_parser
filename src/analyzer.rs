use super::{Commodity, Lot, OptionType};

use std::collections::HashMap;
use std::cmp;

fn populate_book(transactions: std::slice::Iter<(Commodity, Lot)>) -> HashMap<&Commodity, Vec<Lot>> {
    let mut book: HashMap<&Commodity, Vec<Lot>> = HashMap::new();
    for (commodity, lot) in transactions {
        let entry = book.entry(commodity).or_default();
        entry.push(*lot);
    }
    println!("PRINTING BOOK");
    for (commodity, lots) in book.iter() {
        println!("{:?} -> {:?}", commodity, lots);
    }

    println!("DONE PRINTING BOOK");
    book
}

fn option_return(transactions: Vec<(Commodity, Lot)>, book: HashMap<&Commodity, Vec<Lot>> ) {
    // total return, option return on collateral, collateral return, option: collateral:
    let calls: Vec<(Commodity, Lot)> = transactions
        .iter()
        .cloned()
        .filter(|(commodity, _lot)| match commodity {
            Commodity::Option{ticker:_, contract:OptionType::Call, strike:_, expiration:_} => true,
            _ => false,
        })
        .collect();

    let puts: Vec<(Commodity, Lot)> = transactions
        .iter()
        .cloned()
        .filter(|(commodity, _lot)| match commodity {
            Commodity::Option{ticker:_, contract:OptionType::Put, strike:_, expiration:_} => true,
            _ => false,
        })
        .collect();

    

    
    // gather all of the option sales
    // for calls, options return on collateral and collateral return
    // for puts, options return on cash coverage of put
    //println!("{:?}", calls);
    //println!("{:?}", puts);
    //println!("{:?}", stocks);
}

pub fn profit_loss(transactions: Vec<(Commodity, Lot)>) -> i32 {
    let mut book: HashMap<Commodity, Vec<Lot>> = HashMap::new();
    let mut profit = 0;
    for (commodity,mut lot) in transactions {
        println!("Inserting {:?} into book", &commodity);
        //println!("{:?}", book);
        let entry = book.entry(commodity.clone()).or_default();
        while entry.len() > 0 && ((lot.quantity > 0) != (entry[0].quantity > 0)) && lot.quantity != 0 {
            let settlement = cmp::max(entry[0].quantity.abs(), lot.quantity.abs());
            let (buy_price, sell_price) = if lot.quantity > 0 {
                (lot.price, entry[0].price)
            } else {
                (entry[0].price, lot.price)
            };
            println!("\tSettling {} shares of {:?} for profit of {} dollars", settlement, &commodity, settlement * (sell_price as i32 - buy_price as i32)/100);
            profit += settlement * (sell_price as i32 - buy_price as i32);
            println!("\tNew profit is {}", profit);
            let update = if lot.quantity > 0 {
                (lot.quantity - settlement, entry[0].quantity + settlement)
            } else {
                (lot.quantity + settlement, entry[0].quantity - settlement)
            };
            entry[0].quantity = update.0;
            lot.quantity = update.1;

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
