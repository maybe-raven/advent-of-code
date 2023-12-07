use std::{
    cmp::Ordering,
    ops::{Deref, Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Card(u8); //{{{

impl Card {
    const MEMBERS: [(u8, u8); 13] = [
        (b'2', 0),
        (b'3', 1),
        (b'4', 2),
        (b'5', 3),
        (b'6', 4),
        (b'7', 5),
        (b'8', 6),
        (b'9', 7),
        (b'T', 8),
        (b'J', 9),
        (b'Q', 10),
        (b'K', 11),
        (b'A', 12),
    ];

    fn with_ascii_char(input: u8) -> Option<Self> {
        Self::MEMBERS
            .into_iter()
            .find_map(|(c, x)| (input == c).then_some(Self(x)))
    }
}
//}}}

// #[derive(Debug, Clone, Copy, Default)]
// struct CardCounter([u8; 13]); //{{{
//
// impl CardCounter {
//     fn new() -> Self {
//         CardCounter([0; 13])
//     }
//
//     fn add(&mut self, c: Card) {
//         self[c] += 1;
//     }
//
//     fn get(&self) -> u8 {
//         self.0.into_iter().max().expect("`[u8; 13]` is not empty.")
//     }
// }
//
// impl Index<Card> for CardCounter {
//     type Output = u8;
//
//     fn index(&self, index: Card) -> &Self::Output {
//         self.0.index(index.0 as usize)
//     }
// }
//
// impl IndexMut<Card> for CardCounter {
//     fn index_mut(&mut self, index: Card) -> &mut Self::Output {
//         self.0.index_mut(index.0 as usize)
//     }
// }
//}}}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandTier {
    Five,
    Four,
    House,
    Three,
    Two,
    One,
    Shit,
}

impl From<[Card; 5]> for HandTier {
    fn from(mut cards: [Card; 5]) -> Self {
        cards.sort_unstable();

        let mut previous_card = cards[0];
        let mut current_count = 1;
        let mut max_count = 1;
        let mut second_max_count = 1;

        for card in cards.into_iter().skip(1) {
            if card == previous_card {
                current_count += 1;
            } else {
                if current_count > max_count {
                    second_max_count = max_count;
                    max_count = current_count;
                } else if current_count > second_max_count {
                    second_max_count = current_count
                }

                previous_card = card;
                current_count = 1;
            }
        }
        if current_count > max_count {
            second_max_count = max_count;
            max_count = current_count;
        } else if current_count > second_max_count {
            second_max_count = current_count
        }

        if max_count == 5 {
            Self::Five
        } else if max_count == 4 {
            Self::Four
        } else if max_count == 3 {
            if second_max_count == 2 {
                Self::House
            } else {
                Self::Three
            }
        } else if max_count == 2 {
            if second_max_count == 2 {
                Self::Two
            } else {
                Self::One
            }
        } else {
            Self::Shit
        }
    }
}

impl From<HandTier> for u8 {
    fn from(value: HandTier) -> Self {
        match value {
            HandTier::Five => 6,
            HandTier::Four => 5,
            HandTier::House => 4,
            HandTier::Three => 3,
            HandTier::Two => 2,
            HandTier::One => 1,
            HandTier::Shit => 0,
        }
    }
}

impl PartialOrd for HandTier {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandTier {
    fn cmp(&self, other: &Self) -> Ordering {
        u8::from(*self).cmp(&u8::from(*other))
    }
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    //{{{
    tier: HandTier,
    bid: usize,
    cards: [Card; 5],
}

impl Hand {
    fn with_str(s: &str) -> Option<Self> {
        let (cards_str, bid_str) = s.trim().split_once(' ')?;

        if cards_str.len() != 5 {
            return None;
        }

        let mut cards = [Card::default(); 5];
        for (i, c) in cards_str.bytes().enumerate() {
            cards[i] = Card::with_ascii_char(c)?;
        }

        Some(Self {
            tier: HandTier::from(cards),
            bid: bid_str.parse().ok()?,
            cards,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.tier.cmp(&other.tier) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self
                .cards
                .into_iter()
                .zip(other.cards)
                .find_map(|(a, b)| match a.cmp(&b) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater),
                    Ordering::Equal => None,
                })
                .unwrap_or(Ordering::Equal),
        }
    }
    //}}}
}

fn solutionate<S: Deref<Target = str>, E: ToString, I: IntoIterator<Item = Result<S, E>>>(
    input: I,
) -> Result<usize, String> {
    let iter = input.into_iter();
    let mut hands: Vec<Hand> = iter
        .map(|line_input| {
            let line = &*line_input.map_err(|e| e.to_string())?;
            let hand =
                Hand::with_str(line).ok_or_else(|| format!("Failed to parse line: {line}"))?;
            Ok(hand)
        })
        .collect::<Result<Vec<Hand>, String>>()?;
    hands.sort();
    let answer = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();
    Ok(answer)
}

fn main() -> Result<(), String> {
    println!("{}", solutionate(std::io::stdin().lines())?);
    Ok(())
}
