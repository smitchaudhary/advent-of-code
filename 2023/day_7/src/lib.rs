pub mod cards {
    use std::{cmp::Ordering, collections::HashMap, error::Error, fs};

    #[derive(Eq, PartialEq, Clone, Copy)]
    pub enum ScoringMode {
        Normal,
        WithJoker,
    }

    pub fn run(file_name: &String, scoring_mode: ScoringMode) -> Result<u32, Box<dyn Error>> {
        let contents = fs::read_to_string(file_name)?;

        let mut hands: Vec<Hand> = Vec::new();

        for line in contents.lines() {
            let mut iter = line.split_whitespace();
            let cards = iter.next().unwrap().to_string();
            let bid = iter.next().unwrap().parse::<u32>().unwrap();
            let score = get_score_from_cards(&cards, scoring_mode);

            hands.push(Hand {
                cards,
                bid,
                score,
                scoring_mode,
            });
        }

        hands.sort();

        let mut total_winnings = 0;

        for idx in 0..hands.len() {
            let rank = (idx + 1) as u32;
            total_winnings += rank * hands[idx].bid;
        }

        Ok(total_winnings)
    }

    #[derive(PartialEq, Eq)]
    struct Hand {
        cards: String,
        bid: u32,
        score: u32,
        scoring_mode: ScoringMode,
    }
    // Originally, I decided to implement this traits for Hand because
    // I wanted to use > while sorting, and it would be a good exercise
    // to implement traits for custom structs.
    // But then found sort directly, so did not need to write it, but it
    // also requires these trait implementations.
    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.score.partial_cmp(&other.score) {
                Some(Ordering::Equal) => {
                    second_ordering_rule(&self.cards, &other.cards, self.scoring_mode)
                }
                other_results => other_results,
            }
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.score.cmp(&other.score) {
                Ordering::Equal => {
                    second_ordering_rule(&self.cards, &other.cards, self.scoring_mode).unwrap()
                }
                other_results => other_results,
            }
        }
    }

    fn second_ordering_rule(
        card_1: &String,
        card_2: &String,
        scoring_mode: ScoringMode,
    ) -> Option<Ordering> {
        for i in 0..card_1.len() {
            if card_1.chars().nth(i).unwrap() == card_2.chars().nth(i).unwrap() {
                continue;
            } else {
                return char_to_num(card_1.chars().nth(i).unwrap(), scoring_mode)
                    .partial_cmp(&char_to_num(card_2.chars().nth(i).unwrap(), scoring_mode));
            }
        }
        Some(Ordering::Equal)
    }

    fn char_to_num(c: char, scoring_mode: ScoringMode) -> u8 {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => match scoring_mode {
                ScoringMode::Normal => 11,
                ScoringMode::WithJoker => 1,
            },
            'T' => 10,
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            _ => 0,
        }
    }

    fn get_score_from_cards(cards: &String, scoring_mode: ScoringMode) -> u32 {
        // Socres are as follows:
        // 5 of-a-kind -> 6
        // 4 of-a-kind -> 5
        // full house -> 4
        // 3 of-a-kind -> 3
        // 2 pairs -> 2
        // 1 pair -> 1
        // high card -> 0
        let mut counter_hmap: HashMap<char, u32> = HashMap::new();
        for ch in cards.chars() {
            if counter_hmap.contains_key(&ch) {
                *counter_hmap.get_mut(&ch).unwrap() += 1;
            } else {
                counter_hmap.insert(ch, 1);
            }
        }

        let joker_count = match scoring_mode {
            ScoringMode::Normal => 0,
            ScoringMode::WithJoker => *counter_hmap.get(&'J').unwrap_or(&0),
        };

        if counter_hmap.len() == 1 {
            return 6;
        } else if counter_hmap.len() == 2 {
            if joker_count > 0 {
                return 6;
            } else if counter_hmap.values().collect::<Vec<&u32>>().contains(&&4) {
                return 5;
            } else {
                return 4;
            }
        } else if counter_hmap.len() == 3 {
            if counter_hmap.values().collect::<Vec<&u32>>().contains(&&3) {
                if joker_count > 0 {
                    return 5;
                } else {
                    return 3;
                }
            } else {
                if joker_count == 2 {
                    return 5;
                } else if joker_count == 1 {
                    return 4;
                }
                return 2;
            }
        } else if counter_hmap.len() == 4 {
            if joker_count > 0 {
                return 3;
            } else {
                return 1;
            }
        } else {
            return 0 + joker_count;
        }
    }
}
