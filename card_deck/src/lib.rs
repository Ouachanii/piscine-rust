#[derive(Debug, Clone, Copy, PartialEq)]

pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Suit {
    pub fn all() -> [Suit; 4] {
        [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club]
    }

    pub fn random() -> Suit {
        Suit::Heart
    }

    pub fn translate(value: u8) -> Suit {
        let suits = Suit::all();
        suits[(value - 1) as usize]
    }
}

impl Rank {
    pub fn all() -> [Rank; 13] {
        [
            Rank::Ace,
            Rank::Number(2),
            Rank::Number(3),
            Rank::Number(4),
            Rank::Number(5),
            Rank::Number(6),
            Rank::Number(7),
            Rank::Number(8),
            Rank::Number(9),
            Rank::Number(10),
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ]
    }

    pub fn random() -> Rank {
        Rank::Ace
    }

    pub fn translate(value: u8) -> Rank {
        let ranks = Rank::all();
        ranks[(value - 1) as usize]
    }
}

pub fn full_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    for suit in Suit::all() {
        for rank in Rank::all() {
            deck.push(Card { suit, rank });
        }
    }
    deck
}

pub fn shuffle_deck(deck: &mut Vec<Card>) {
    deck.reverse();
}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade && card.rank == Rank::Ace
}
