use std::cmp::Reverse;
use std::str::FromStr;
use CardRank::*;
use HandRank::*;
use Suit::*;

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    if hands.is_empty() {
        return None;
    }

    let mut ranked_hands = hands
        .iter()
        .map(|&hand_str| {
            hand_str
                .parse::<PokerHand>()
                .map(|hand| (hand_str, hand.rank()))
        })
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    ranked_hands.sort_unstable_by(|(_, r1), (_, r2)| r1.cmp(r2).reverse());

    let highest_rank = &ranked_hands[0].1;
    let winners = ranked_hands
        .iter()
        .take_while(|(_, rank)| rank == highest_rank)
        .map(|&(hand_str, _)| hand_str)
        .collect();

    Some(winners)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "C" => Ok(Clubs),
            "D" => Ok(Diamonds),
            "H" => Ok(Hearts),
            "S" => Ok(Spades),
            other => Err(format!("Invalid suit: '{}'", other)),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CardRank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl CardRank {
    fn succeeds(self, other: CardRank) -> bool {
        match (self, other) {
            (Ace, King) => true,
            (King, Queen) => true,
            (Queen, Jack) => true,
            (Jack, Number(10)) => true,
            (Number(n), Number(m)) => n == m + 1,
            (Number(2), Ace) => true,
            _ => false,
        }
    }
}

impl FromStr for CardRank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Ace),
            "K" => Ok(King),
            "Q" => Ok(Queen),
            "J" => Ok(Jack),
            num => num
                .parse()
                .map_err(|_| format!("Invalid card rank: '{}'", num))
                .and_then(|n| {
                    if n >= 2 && n <= 10 {
                        Ok(Number(n))
                    } else {
                        Err(format!("Card number out of range: '{}'", num))
                    }
                }),
        }
    }
}

#[derive(Debug)]
struct Card {
    rank: CardRank,
    suit: Suit,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 || s.len() > 3 {
            return Err(format!("Invalid card: '{}'", s));
        }
        let (rank_str, suit_str) = s.split_at(s.len() - 1);
        let rank = rank_str.parse()?;
        let suit = suit_str.parse()?;
        Ok(Card { rank, suit })
    }
}

#[derive(Debug)]
struct PokerHand {
    cards: Vec<Card>,
}

impl PokerHand {
    fn rank(&self) -> HandRank {
        let flush = self.cards.iter().all(|c| c.suit == self.cards[0].suit);

        let mut ranks: Vec<_> = self.cards.iter().map(|c| c.rank).collect();
        ranks.sort_unstable_by_key(|&rank| Reverse(rank));

        if let Some(high) = Self::get_straight_high(&ranks) {
            if flush {
                return StraightFlush(high);
            } else {
                return Straight(high);
            }
        }

        match Self::group_card_ranks(&ranks).as_slice() {
            [(4, quad), (1, kicker)] => FourOfAKind(*quad, *kicker),
            [(3, triple), (2, pair)] => FullHouse(*triple, *pair),
            [(3, triple), (1, kicker1), (1, kicker2)] => {
                ThreeOfAKind(*triple, *kicker1, *kicker2)
            }
            [(2, pair1), (2, pair2), (1, kicker)] => {
                TwoPair(*pair1, *pair2, *kicker)
            }
            [(2, pair), (1, kicker1), (1, kicker2), (1, kicker3)] => {
                OnePair(*pair, *kicker1, *kicker2, *kicker3)
            }
            [(1, rank1), (1, rank2), (1, rank3), (1, rank4), (1, rank5)] => {
                if flush {
                    Flush(*rank1, *rank2, *rank3, *rank4, *rank5)
                } else {
                    HighCard(*rank1, *rank2, *rank3, *rank4, *rank5)
                }
            }
            _ => panic!("Failed to match poker hand"),
        }
    }

    fn get_straight_high(sorted_ranks: &[CardRank]) -> Option<CardRank> {
        let mut window = sorted_ranks.windows(2);
        let high = if sorted_ranks.starts_with(&[Ace, Number(5)]) {
            // May be a 5-high straight (A 5 4 3 2)
            window.next();
            sorted_ranks[1]
        } else {
            // Any other straight
            sorted_ranks[0]
        };

        if window.all(|ranks| ranks[0].succeeds(ranks[1])) {
            Some(high)
        } else {
            None
        }
    }

    fn group_card_ranks(sorted_ranks: &[CardRank]) -> Vec<(u8, CardRank)> {
        let mut count = 0;
        let mut groups = Vec::new();
        let mut ranks = sorted_ranks.iter().peekable();

        while let Some(rank) = ranks.next() {
            count += 1;
            if ranks.peek().map(|&next| next != rank).unwrap_or(true) {
                groups.push((count, *rank));
                count = 0;
            }
        }

        // Sort groups in decreasing number of repetitions and card rank
        groups.sort_unstable_by_key(|&tuple| Reverse(tuple));
        groups
    }
}

impl FromStr for PokerHand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .split_whitespace()
            .map(|card| card.parse::<Card>())
            .collect::<Result<Vec<_>, _>>()?;

        if cards.len() == 5 {
            Ok(PokerHand { cards })
        } else {
            Err(format!("Wrong number of cards: {}", cards.len()))
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandRank {
    HighCard(CardRank, CardRank, CardRank, CardRank, CardRank),
    OnePair(CardRank, CardRank, CardRank, CardRank),
    TwoPair(CardRank, CardRank, CardRank),
    ThreeOfAKind(CardRank, CardRank, CardRank),
    Straight(CardRank),
    Flush(CardRank, CardRank, CardRank, CardRank, CardRank),
    FullHouse(CardRank, CardRank),
    FourOfAKind(CardRank, CardRank),
    StraightFlush(CardRank),
}
