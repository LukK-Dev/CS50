pub fn encrypt(text: String, key: String) -> String {
    let out: String = text
        .chars()
        .map(|char| -> char { map_char(char, &key) })
        .collect();
    out
}

fn map_char(char: char, key: &str) -> char {
    if !in_alphabet(char) {
        return char;
    }

    if char.is_uppercase() {
        return key
            .to_uppercase()
            .chars()
            .nth(get_index(char).unwrap())
            .unwrap();
    } else {
        return key
            .to_lowercase()
            .chars()
            .nth(get_index(char).unwrap())
            .unwrap();
    }
}

fn get_index(char: char) -> Result<usize, &'static str> {
    let letters = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    for (index, letter) in letters.iter().enumerate() {
        if letter == &char.to_lowercase().next().unwrap() {
            return Ok(index);
        }
    }

    Err("Input in NOT contained in the alphabet.")
}

fn in_alphabet(char: char) -> bool {
    let letters = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    letters.contains(&char)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encryption() {
        assert_eq!(
            encrypt(
                String::from("Hello, World!"),
                String::from("pvwabchqrguoefyijsxlmnzdkt")
            ),
            String::from("Qbooy, Zysoa!")
        );
    }
}
