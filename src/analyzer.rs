use super::{Commodity, Lot, OptionType, Settlement};

use std::cmp;
use std::collections::HashMap;

fn populate_book(
    transactions: std::slice::Iter<(Commodity, Lot)>,
) -> HashMap<&Commodity, Vec<Lot>> {
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

fn option_return(transactions: Vec<(Commodity, Lot)>, book: HashMap<&Commodity, Vec<Lot>>) {
    // total return, option return on collateral, collateral return, option: collateral:
    let calls: Vec<(Commodity, Lot)> = transactions
        .iter()
        .cloned()
        .filter(|(commodity, _lot)| match commodity {
            Commodity::Option {
                ticker: _,
                contract: OptionType::Call,
                strike: _,
                expiration: _,
            } => true,
            _ => false,
        })
        .collect();

    let puts: Vec<(Commodity, Lot)> = transactions
        .iter()
        .cloned()
        .filter(|(commodity, _lot)| match commodity {
            Commodity::Option {
                ticker: _,
                contract: OptionType::Put,
                strike: _,
                expiration: _,
            } => true,
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

pub fn settle(lot_open: &mut Lot, lot_close: &mut Lot) -> i32 {
    let settlement = cmp::min(lot_open.quantity.abs(), lot_close.quantity.abs());
    let (buy_price, sell_price) = if lot_open.quantity > 0 {
        (lot_open.price, lot_close.price)
    } else {
        (lot_close.price, lot_open.price)
    };
    let profit = settlement * (sell_price as i32 - buy_price as i32);

    if lot_open.quantity > 0 {
        lot_open.quantity -= settlement;
        lot_close.quantity += settlement;
    } else {
        lot_open.quantity += settlement;
        lot_close.quantity -= settlement;
    };
    profit
}

pub fn profit_loss(transactions: Vec<(Commodity, Lot)>) -> i32 {
    let mut settlements = Vec::new();
    let mut book: HashMap<Commodity, Vec<Lot>> = HashMap::new();
    let mut profit = 0;
    for (commodity, mut lot) in transactions {
        let entry = book.entry(commodity.clone()).or_default();
        while entry.len() > 0
            && ((lot.quantity > 0) != (entry[0].quantity > 0))
            && lot.quantity != 0
        {
            let settle_profit = settle(&mut lot, &mut entry[0]);
            profit += settle_profit;
            settlements.push(Settlement {
                open: entry[0].date,
                close: lot.date,
                commodity: commodity.clone(),
                profit: settle_profit,
            });

            if entry[0].quantity == 0 {
                entry.remove(0);
            }
        }
        if lot.quantity.abs() != 0 {
            entry.push(lot);
        }
        //println!("\tBook for {:?} is {:?}", &commodity, &entry);
    }
    for settle in settlements {
        println!(
            "{} | {} | {:<25} | ${:>5}.{:02}",
            settle.open,
            settle.close,
            format!("{}", settle.commodity),
            settle.profit / 10000,
            settle.profit % 10000 / 100
        );
    }
    println!("total profit: {}", profit);
    profit
}

#[cfg(test)]
mod tests {
    use super::super::Date;
    use super::*;

    const lot_buy1: Lot = Lot {
        date: Date {
            day: 10,
            month: 20,
            year: 10,
        },
        quantity: 100,
        price: 136777,
    };
    const lot_buy2: Lot = Lot {
        date: Date {
            day: 10,
            month: 20,
            year: 10,
        },
        quantity: 100,
        price: 138500,
    };
    const lot_sell: Lot = Lot {
        date: Date {
            day: 14,
            month: 20,
            year: 10,
        },
        quantity: -200,
        price: 137500,
    };

    #[test]
    fn settlement() {
        let profit1 = settle(&mut lot_buy1.clone(), &mut lot_sell.clone());
        let profit2 = settle(&mut lot_buy2.clone(), &mut lot_sell.clone());
        assert_eq!(profit1, 72300);
        assert_eq!(profit2, -100000);

        assert_eq!(
            settle(&mut lot_buy1.clone(), &mut lot_sell.clone()),
            settle(&mut lot_sell.clone(), &mut lot_buy1.clone())
        );
        assert_eq!(
            settle(&mut lot_buy2.clone(), &mut lot_sell.clone()),
            settle(&mut lot_sell.clone(), &mut lot_buy2.clone())
        )
    }

    #[test]
    fn profit() {
        let stock = Commodity::Stock(String::from("UAA"));
        let transactions = vec![
            (stock.clone(), lot_buy1),
            (stock.clone(), lot_buy2),
            (stock.clone(), lot_sell),
        ];
        let profit = profit_loss(transactions);
        assert_eq!(profit, -27700);
    }
}
