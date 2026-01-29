use rand::Rng;

#[derive(Debug, Clone)]
pub struct NRIC {
    value: String,
}

impl NRIC {
    pub fn new(value: Option<&str>) -> Self {
        let nric = match value {
            Some(v) => v.trim().to_uppercase(),
            None => String::new(),
        };
        NRIC { value: nric }
    }

    pub fn generate(first_char: Option<char>) -> Self {
        let computed_first_char = match first_char {
            Some(c) if "STFGM".contains(c.to_ascii_uppercase()) => c.to_ascii_uppercase(),
            _ => Self::get_random_first_char(),
        };

        let digits: String = (0..7)
            .map(|_| rand::thread_rng().gen_range(0..10).to_string())
            .collect();

        let checksum = Self::calculate_checksum(computed_first_char, &digits);
        let nric = format!("{}{}{}", computed_first_char, digits, checksum);

        NRIC { value: nric }
    }

    pub fn generate_many(amount: Option<usize>) -> Vec<NRIC> {
        let count = amount.unwrap_or(1).max(1);
        (0..count).map(|_| Self::generate(None)).collect()
    }

    fn get_random_first_char() -> char {
        let chars = ['S', 'T', 'F', 'G', 'M'];
        chars[rand::thread_rng().gen_range(0..chars.len())]
    }

    fn calculate_checksum(
        first_char: char,
        digits_str: &str,
    ) -> char {
        let weights = [2, 7, 6, 5, 4, 3, 2];
        let digits: Vec<u32> = digits_str
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        let weight: u32 = digits
            .iter()
            .enumerate()
            .map(|(i, &d)| d * weights[i])
            .sum();

        let offset = match first_char {
            'T' | 'G' => 4,
            'M' => 3,
            _ => 0,
        };

        let mut index = (offset + weight) % 11;

        if first_char == 'M' {
            index = 10 - index;
        }

        let table = Self::get_checksum_table(first_char);
        table[index as usize]
    }

    fn get_checksum_table(first_char: char) -> Vec<char> {
        match first_char {
            'S' | 'T' => vec!['J', 'Z', 'I', 'H', 'G', 'F', 'E', 'D', 'C', 'B', 'A'],
            'F' | 'G' => vec!['X', 'W', 'U', 'T', 'R', 'Q', 'P', 'N', 'M', 'L', 'K'],
            'M' => vec!['K', 'L', 'J', 'N', 'P', 'Q', 'R', 'T', 'U', 'W', 'X'],
            _ => panic!("Invalid first character"),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
