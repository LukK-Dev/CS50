struct Criteria {
    length: Vec<u32>,
    starting_digits: Vec<u32>,
}

impl Criteria {
    fn new(length: Vec<u32>, starting_digits: Vec<u32>) -> Self {
        Criteria {
            length,
            starting_digits,
        }
    }
}

pub struct CreditCardType {
    name: String,
    criteria: Criteria,
}

impl CreditCardType {
    fn new(name: &str, criteria: Criteria) -> Self {
        CreditCardType {
            name: String::from(name),
            criteria,
        }
    }
}

pub fn validate(number: u64, credit_card_types: &[CreditCardType]) -> Result<String, &'static str> {
    let digits = number_to_digits(number);

    if !check(&digits) {
        return Err("failed to pass checksum");
    }

    for cct in credit_card_types.iter() {
        for length in cct.criteria.length.iter() {
            if *length == digits.len().try_into().unwrap() {
                for starting_digits in cct.criteria.starting_digits.iter() {
                    let sds = number_to_digits(*starting_digits as u64);
                    let mut counter = 0;

                    for i in 0..sds.len() {
                        if sds[i] == digits[i] {
                            counter += 1;
                            continue;
                        } else {
                            break;
                        }
                    }

                    if sds.len() == counter {
                        return Ok(cct.name.clone());
                    }
                }
            }
        }
    }

    Err("invalid credit card number")
}

fn check(digits: &[u32]) -> bool {
    let mut buffer: u32 = 0;
    let is_even = digits.len() % 2 == 0;

    if is_even {
        for d in digits.iter().step_by(2) {
            for v in number_to_digits((d * 2) as u64) {
                buffer += v;
            }
        }
        for d in digits.iter().skip(1).step_by(2) {
            buffer += d;
        }
    } else {
        for d in digits.iter().skip(1).step_by(2) {
            for v in number_to_digits((d * 2) as u64) {
                buffer += v;
            }
        }
        for d in digits.iter().step_by(2) {
            buffer += d;
        }
    }

    *number_to_digits(buffer as u64).last().unwrap() == 0
}

fn number_to_digits(number: u64) -> Vec<u32> {
    number
        .to_string()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect()
}

pub fn construct_credit_card_types() -> Vec<CreditCardType> {
    let amex = CreditCardType::new("AMEX", Criteria::new(vec![15], vec![34, 37]));
    let mastercard = CreditCardType::new(
        "MASTERCARD",
        Criteria::new(vec![16], vec![51, 52, 53, 54, 55]),
    );
    let visa = CreditCardType::new("VISA", Criteria::new(vec![13, 16], vec![4]));

    vec![amex, mastercard, visa]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_check() {
        assert!(check(&number_to_digits(4003600000000014)));
    }

    #[test]
    fn invalid_check() {
        assert!(!check(&number_to_digits(4003600000001014)));
    }

    #[test]
    fn is_amex() {
        assert_eq!(
            validate(378282246310005, &construct_credit_card_types()),
            Ok(String::from("AMEX"))
        );
    }

    #[test]
    fn is_mastercard() {
        assert_eq!(
            validate(5555555555554444, &construct_credit_card_types()),
            Ok(String::from("MASTERCARD"))
        );
    }

    #[test]
    fn is_visa() {
        assert_eq!(
            validate(4111111111111111, &construct_credit_card_types()),
            Ok(String::from("VISA"))
        );
    }

    #[test]
    fn is_invalid() {
        assert_eq!(
            validate(3111111111111111, &construct_credit_card_types()),
            Err("Failed to pass checksum.")
        );
    }
}
