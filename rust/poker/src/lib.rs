use std::{
    cmp::{max, Ordering},
    error::Error,
    fmt::{Debug, Formatter},
    str::FromStr,
};

use itertools::Itertools;
use strum::{Display, EnumString, FromRepr};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which
/// happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    hands
        .into_iter()
        .map(|hand| {
            let ranking: Ranking = hand.parse::<Hand>().expect("can be parsed as hand").into();
            eprintln!("hand {} is ranking: {:?}", hand, ranking);
            (hand, ranking)
        })
        .sorted_by_key(|(_, ranking)| *ranking)
        .group_by(|(_, ranking)| *ranking)
        .into_iter()
        .map(|(_, ranked_hands)| ranked_hands.into_iter().map(|(&hand, _)| hand).collect())
        .last()
        .expect("should be at least 1 ranking group")
}

struct Hand {
    cards: Box<[Card; 5]>,
}
impl FromStr for Hand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cards: Box::new(
                s.split(' ')
                    .map(|card| card.parse::<Card>())
                    .collect::<Result<Vec<_>, _>>()?
                    .try_into()
                    .or(Err("Could not find enough cards in string."))?,
            ),
        })
    }
}

#[derive(Hash)]
struct Card {
    face: Face,
    suit: Suit,
}
impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.face, self.suit)
    }
}
impl FromStr for Card {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars().rev();
        Ok(Self {
            suit: chars
                .next()
                .ok_or("Not any chars in string.")?
                .to_string()
                .parse()?,
            face: chars.rev().collect::<String>().parse()?,
        })
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)] // strictly ordered even when hands aren't
enum Ranking {
    HighCard([Face; 5]),
    OnePair(Face, [Face; 3]),
    TwoPair {
        highest_pair: Face,
        lowest_pair: Face,
        kicker: Face,
    }, // I think this is the correct derive ordering?
    ThreeOfAKind(Face, [Face; 2]),
    Straight(Face),
    Flush([Face; 5]),
    FullHouse {
        three: Face,
        two: Face,
    },
    FourOfAKind(Face, Face),
    StraightFlush(Face),
    FiveOfAKind(Face),
}
impl From<Hand> for Ranking {
    fn from(hand: Hand) -> Self {
        let face_groups = hand.cards.iter().into_group_map_by(|hand| hand.face);
        // ^ is maybe only grouping continuously?
        let mut significance_group_iter = face_groups
            .into_iter()
            // sort by face group sizes, then face value
            .sorted_by(|(face_a, group_a), (face_b, group_b)| {
                match group_a.len().cmp(&group_b.len()) {
                    // ensure face ordering for two pair or high card
                    Ordering::Equal => face_a.cmp(face_b),
                    size_ordering => size_ordering,
                }
            })
            .rev(); // descending so that we can take most significant groups one at a time
        let most_significant_group = significance_group_iter
            .next()
            .expect("Not a single highest face group?");
        let cards_are_flush = || hand.cards.iter().map(|c| c.suit).all_equal();
        match most_significant_group.1.len() {
            // 5 is always the best it can be
            5 => Self::FiveOfAKind(most_significant_group.0),
            // not even a single pair
            1 => {
                // the best rankings with no pairs are straights
                let sorted_faces = hand
                    .cards
                    .iter()
                    .map(|c| c.face)
                    .sorted()
                    .collect::<Vec<_>>();
                if sorted_faces
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| {
                        (
                            a.incremented_value().expect(
                                "Ordered lower of two faces should be able to get incremented",
                            ),
                            b,
                        )
                    })
                    .all(|(a_incremented, &b)| {
                        // straights found by continuously exact rising elements,
                        // with exception for A->5 straight
                        a_incremented == b || (a_incremented == Face::N6 && b == Face::Ace)
                    })
                {
                    // straights can be straight flush if all 5 cards in hand have the same suit
                    let mut highest_faces_iter = sorted_faces.iter().rev();
                    let highest_counted_face =
                        match highest_faces_iter.next().expect("at least one face") {
                            Face::Ace if sorted_faces.first() == Some(&Face::N2) => {
                                highest_faces_iter.next().expect("at least two elements")
                            }
                            other => other,
                        };
                    if cards_are_flush() {
                        Self::StraightFlush(*highest_counted_face)
                    } else {
                        Self::Straight(*highest_counted_face)
                    }
                } else {
                    let ordered_values = vec![most_significant_group]
                        .into_iter()
                        .chain(significance_group_iter.collect::<Vec<_>>().into_iter())
                        .map(|(face, _)| face)
                        .collect::<Vec<Face>>()
                        .try_into()
                        .expect("can get correct thing");
                    if cards_are_flush() {
                        Self::Flush(ordered_values)
                    } else {
                        Self::HighCard(ordered_values)
                    }
                }
            }
            c => {
                let second_most_significant_group = significance_group_iter
                    .next()
                    .expect("If the biggest set isn't all the cards then there is another set");
                assert!((1..=2).contains(&second_most_significant_group.1.len())); // hand size should make this impossible
                let non_flush_rank: Self = match c {
                    4 => {
                        Self::FourOfAKind(most_significant_group.0, second_most_significant_group.0)
                    }
                    3 => {
                        if second_most_significant_group.1.len() == 2 {
                            Self::FullHouse {
                                three: most_significant_group.0,
                                two: second_most_significant_group.0,
                            }
                        } else {
                            Self::ThreeOfAKind(
                                most_significant_group.0,
                                [
                                    second_most_significant_group.0,
                                    significance_group_iter.next().expect("one more value").0,
                                ],
                            )
                        }
                    }
                    2 => {
                        if second_most_significant_group.1.len() == 2 {
                            Self::TwoPair {
                                highest_pair: most_significant_group.0,
                                lowest_pair: second_most_significant_group.0,
                                kicker: significance_group_iter.next().expect("one more value").0,
                            }
                        } else {
                            Self::OnePair(
                                most_significant_group.0,
                                vec![second_most_significant_group]
                                    .into_iter()
                                    .chain(
                                        significance_group_iter
                                            .clone()
                                            .collect::<Vec<_>>()
                                            .into_iter(),
                                    )
                                    .map(|(face, _)| face)
                                    .collect::<Vec<Face>>()
                                    .try_into()
                                    .expect("can get correct thing"),
                            )
                        }
                    }
                    _ => unreachable!("covered by outside matching"),
                };
                if cards_are_flush() {
                    max(
                        Self::Flush(
                            vec![most_significant_group]
                                .into_iter()
                                .chain(significance_group_iter.collect::<Vec<_>>().into_iter())
                                .map(|(face, _)| face)
                                .collect::<Vec<Face>>()
                                .try_into()
                                .expect("can get correct thing"),
                        ),
                        non_flush_rank,
                    )
                } else {
                    non_flush_rank
                }
            }
        }
    }
}

// ascending order top->bottom
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone, FromRepr, EnumString, Debug)]
enum Face {
    #[strum(serialize = "2")]
    N2,
    #[strum(serialize = "3")]
    N3,
    #[strum(serialize = "4")]
    N4,
    #[strum(serialize = "5")]
    N5,
    #[strum(serialize = "6")]
    N6,
    #[strum(serialize = "7")]
    N7,
    #[strum(serialize = "8")]
    N8,
    #[strum(serialize = "9")]
    N9,
    #[strum(serialize = "10")]
    N10,
    #[strum(serialize = "J")]
    Jack,
    #[strum(serialize = "Q")]
    Queen,
    #[strum(serialize = "K")]
    King,
    #[strum(serialize = "A")]
    Ace,
}
impl Face {
    fn incremented_value(&self) -> Option<Face> {
        Self::from_repr(*self as usize + 1)
    }
}
#[derive(Eq, PartialEq, Hash, Copy, Clone, EnumString, Display)]
enum Suit {
    S,
    D,
    H,
    C,
}
