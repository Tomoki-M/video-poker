use std::io;
use rand::seq::SliceRandom;

#[derive(Debug,Copy, Clone)]
enum Suit {
    Spade,
    Club,
    Diamond,
    Heart,
}

#[derive(Debug,Copy, Clone)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Debug,Copy, Clone)]
enum Hand {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    JacksOrBetter,
}

#[derive(Debug, Copy, Clone)]
struct Card {
    suit: Suit,
    rank: Rank,
}

fn generate_deck() -> Vec<Card> {
    let ranks = [
        Rank::Ace,
        Rank::Two,
        Rank::Three,
        Rank::Four,
        Rank::Five,
        Rank::Six,
        Rank::Seven,
        Rank::Eight,
        Rank::Nine,
        Rank::Ten,
        Rank::Jack,
        Rank::Queen,
        Rank::King,
    ];
    let suits = [Suit::Spade, Suit::Club, Suit::Diamond, Suit::Heart];

    let mut deck: Vec<Card> = Vec::new();

    for suit in suits.iter() {
        for rank in ranks.iter() {
            deck.push(Card {
                suit: *suit,
                rank: *rank,
            });
        }
    }

    deck
}

fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
}

#[derive(Debug, Clone)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let ranks = [
            Rank::Ace,
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
        ];
        let suits = [Suit::Spade, Suit::Club, Suit::Diamond, Suit::Heart];

        let mut cards: Vec<Card> = Vec::new();

        for suit in suits.iter() {
            for rank in ranks.iter() {
                cards.push(Card {
                    rank: *rank,
                    suit: *suit,
                });
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn draw_hand(&mut self, num_cards: usize) -> Option<Vec<Card>> {
        if self.cards.len() < num_cards {
            return None;
        }
        let mut hand: Vec<Card> = Vec::with_capacity(num_cards);
        for _ in 0..num_cards {
            hand.push(self.draw().unwrap());
        }
        Some(hand)
    }
}

fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    println!("Please input your guess.");   // ほら、予想を入力してね

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");     // 行の読み込みに失敗しました

    println!("You guessed: {}", guess);     // 次のように想しました: {}

    let mut deck = generate_deck();
    shuffle_deck(&mut deck);

    let mut deck = Deck::new();
    println!("Unshuffled deck: {:?}", deck);

    deck.shuffle();
    println!("Shuffled deck: {:?}", deck);

    let hand = deck.draw_hand(5).unwrap();
    println!("Drawn hand: {:?}", hand);
}
