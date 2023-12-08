use std::{
    cmp::Ordering,
    ops::{Deref, Index, IndexMut},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Card(u8); //{{{

impl Card {
    const MEMBERS: [(u8, u8); 13] = [
        (b'J', 0),
        (b'2', 1),
        (b'3', 2),
        (b'4', 3),
        (b'5', 4),
        (b'6', 5),
        (b'7', 6),
        (b'8', 7),
        (b'9', 8),
        (b'T', 9),
        (b'Q', 10),
        (b'K', 11),
        (b'A', 12),
    ];

    const JOKER: Self = Self(0);

    fn with_ascii_char(input: u8) -> Option<Self> {
        Self::MEMBERS
            .into_iter()
            .find_map(|(c, x)| (input == c).then_some(Self(x)))
    }
}
//}}}

#[derive(Debug, Clone, Copy)]
struct CardCounts {
    //{{{
    max: u8,
    second_max: u8,
}

impl CardCounts {
    fn new() -> Self {
        Self {
            max: 0,
            second_max: 0,
        }
    }

    fn register(&mut self, v: u8) {
        if v > self.max {
            self.second_max = self.max;
            self.max = v;
        } else if v > self.second_max {
            self.second_max = v;
        }
    }
    //}}}
}

#[derive(Debug, Clone, Copy, Default)]
struct CardCounter([u8; 13]); //{{{

impl CardCounter {
    fn new() -> Self {
        CardCounter([0; 13])
    }

    fn add(&mut self, c: Card) {
        self[c] += 1;
    }

    fn get(&self) -> CardCounts {
        let mut counts = CardCounts::new();
        for count in self.0.into_iter().skip(1) {
            counts.register(count);
        }
        counts.max += self[Card::JOKER];
        counts
    }
}

impl Index<Card> for CardCounter {
    type Output = u8;

    fn index(&self, index: Card) -> &Self::Output {
        self.0.index(index.0 as usize)
    }
}

impl IndexMut<Card> for CardCounter {
    fn index_mut(&mut self, index: Card) -> &mut Self::Output {
        self.0.index_mut(index.0 as usize)
    }
}
//}}}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandTier {
    //{{{
    Shit,
    One,
    Two,
    Three,
    House,
    Four,
    Five,
}

impl From<[Card; 5]> for HandTier {
    fn from(cards: [Card; 5]) -> Self {
        let card_counts = {
            let mut card_counter = CardCounter::new();
            for card in cards {
                card_counter.add(card);
            }
            card_counter.get()
        };

        println!("{:?}; {:?}", card_counts, cards);

        if card_counts.max == 5 {
            Self::Five
        } else if card_counts.max == 4 {
            Self::Four
        } else if card_counts.max == 3 {
            if card_counts.second_max == 2 {
                Self::House
            } else {
                Self::Three
            }
        } else if card_counts.max == 2 {
            if card_counts.second_max == 2 {
                Self::Two
            } else {
                Self::One
            }
        } else {
            Self::Shit
        }
    }
    //}}}
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
        .inspect(|x| {
            println!("{:?}", x);
        })
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum();
    Ok(answer)
}

fn main() -> Result<(), String> {
    println!("{}", solutionate(std::io::stdin().lines())?);
    Ok(())
}
