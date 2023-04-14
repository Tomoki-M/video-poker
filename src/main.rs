use std::io;

#[derive(Copy, Clone)]
enum Suit {
    Spade,
    Club,
    Diamond,
    Heart,
}

#[derive(Copy, Clone)]
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
            match suit {
                Suit::Spade => println!("spade"),
                Suit::Club => println!("club"),
                Suit::Diamond => println!("Diamond"),
                Suit::Heart => println!("Heart"),
            }
        }
    }

    deck
}


fn main() {
    println!("Guess the number!");          // 数を当ててごらん

    println!("Please input your guess.");   // ほら、予想を入力してね

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");     // 行の読み込みに失敗しました

    println!("You guessed: {}", guess);     // 次のように想しました: {}

    generate_deck();
}
