use std::{fs, io, collections::HashMap};

fn input_lines(filename: &str) -> io::Result<Vec<String>> {
    fs::read_to_string(filename).map(|s| s.lines().map(ToString::to_string).collect())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    // Jack,
    Queen,
    King,
    Ace
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'J' => Self::Joker,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            // 'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => unreachable!()
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Box<[Card]>,
    bid: u32
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_counts: HashMap<Card, u32> = self.cards.iter().fold(HashMap::new(), |mut counts, card| {
            match counts.get_mut(card) {
                Some(card_count) => { *card_count += 1; }
                None => { counts.insert(*card, 1); }
            }
            counts
        });

        let highest_key = card_counts.iter()
            .filter(|(k, _)| **k != Card::Joker)
            .max_by_key(|(_, v)| **v);

        if let Some((highest_card, _)) = highest_key {
            let highest_card = highest_card.clone();
            if let Some(joker_count) = card_counts.remove(&Card::Joker) {
                if let Some(count) = card_counts.get_mut(&highest_card) {
                    *count += joker_count;
                }
            }
        }

        let mut card_count_values: Vec<u32> = card_counts.into_values().collect();
        card_count_values.sort_by(|a, b| b.cmp(a));
        
        match &card_count_values[..] {
            [5] => HandType::FiveOfAKind,
            [4, ..] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, ..] => HandType::ThreeOfAKind,
            [2, 2, ..] => HandType::TwoPair,
            [2, ..] => HandType::OnePair,
            _ => HandType::HighCard
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type().partial_cmp(&other.hand_type()) {
            Some(std::cmp::Ordering::Equal) => {
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    if let Some(std::cmp::Ordering::Equal) = c1.partial_cmp(c2) {
                        continue;
                    }
                    return c1.partial_cmp(c2);
                }
                Some(std::cmp::Ordering::Equal)
            }
            ordering => ordering
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            std::cmp::Ordering::Equal => {
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    if let std::cmp::Ordering::Equal = c1.cmp(c2) {
                        continue;
                    }
                    return c1.cmp(c2);
                }
                std::cmp::Ordering::Equal
            }
            ordering => ordering
        }
    }
}

fn main() -> io::Result<()> {
    let mut hands: Vec<Hand> = input_lines("resources/input.txt")?
        .iter()
        .map(|line| line.split(" ").collect::<Vec<_>>())
        .map(|cards_and_bid| Hand {
            cards: cards_and_bid[0].chars().map(Card::from).collect(),
            bid: cards_and_bid[1].parse().unwrap()
        })
        .collect();
    hands.sort();

    let total_winnings: u32 = hands.iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum();

    dbg!(total_winnings);

    Ok(())
}
