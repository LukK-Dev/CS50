use credit::{construct_credit_card_types, validate};
use std::env;

fn main() {
    let credit_card_number = env::args().nth(1).unwrap().parse::<u64>().expect("Error parsing the input");
    let result = validate(credit_card_number, &construct_credit_card_types());

    match result {
        Ok(card) => println!("Credit card type: {}", card),
        Err(error) => println!("An error occured: {}", error),
    }
}
