use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hand {
    pub hand: String,
    pub values: Vec<usize>,
    pub bid: usize,
}

impl Hand {
    pub fn parse_hand(line: &str) -> Self {
        let (hand, bid) = line.split_once(" ").unwrap();
        return Hand {
            hand: hand.to_string(),
            values: Self::get_values(hand),
            bid: bid.parse::<usize>().unwrap(),
        };
    }

    fn get_values(hand: &str) -> Vec<usize> {
        let mut values = Vec::new();
        values.push(Hand::get_value(hand));

        for char in hand.chars() {
            values.push(Hand::translate_char(char));
        }
        return values;
    }

    fn translate_char(char: char) -> usize {
        match char {
            'A' => 20,
            'K' => 19,
            'Q' => 18,
            'J' => 17,
            'T' => 16,
            '9' => 15,
            '8' => 14,
            '7' => 13,
            '6' => 12,
            '5' => 11,
            '4' => 10,
            '3' => 9,
            '2' => 8,
            _ => panic!(),
        }
    }

    fn get_value(hand: &str) -> usize {
        let mut pieces: HashMap<char, usize> = HashMap::new();
        for char in hand.chars() {
            let entry = pieces.entry(char).or_default();
            *entry += 1;
        }

        // Five of a kind
        if pieces.len() == 1 {
            return 7;
        }

        let mut sorted_sizes = pieces.values().map(|v| v.clone()).collect::<Vec<usize>>();
        sorted_sizes.sort();

        // Four of a kind
        if pieces.len() == 2 && sorted_sizes[0] == 1 {
            return 6;
        }

        // Full house
        if pieces.len() == 2 && sorted_sizes[0] == 2 {
            return 5;
        }

        // Three of a kind
        if sorted_sizes.iter().any(|size| *size == 3) {
            return 4;
        }

        // Two pair
        if pieces.len() == 3 {
            // dbg!(hand, pieces, sorted_sizes);
            return 3;
        }

        // One pair
        if pieces.len() == 4 {
            return 2;
        }

        // High value
        return 1;
    }
}
