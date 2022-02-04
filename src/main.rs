// #[derive(Debug)]

extern crate rand;

use rand::Rng;

struct Card {
    suit: String,
    value: String,
    int_value: u32
}

impl Card {
    fn get_cards_string(&self) -> String {
        format!("{} {} ({})", self.suit, self.value, self.int_value.to_string())
    }
}

struct Deck {
    deck: Vec<Card>,
}

impl Deck {
    fn generate_deck() -> Deck {
        let mut new_deck = vec![];
        let suits = ["♣", "♦", "♥", "♠"];
        let values = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];

        let mut int_value = 1;
        for suit in suits.iter() {
            for value in values.iter() {
                let card = Card {
                    suit: suit.to_string(),
                    value: value.to_string(),
                    int_value: int_value
                };
                new_deck.push(card);
                int_value += 1;
            }
        }

        Deck {deck: new_deck}
    }

    fn print_deck(&self) -> () {
        println!("Displaying cards...");
        for card in self.deck.iter() {
            println!("{}", card.get_cards_string());
        }
    }

    fn shuffle_deck(&self) -> () {
        let deck_size = self.deck.len();
        for card in self.deck.iter() {
            let rand_i = rand::thread_rng().gen_range(1..=deck_size);
            let temp = self.deck[rand_i];
            self.deck[rand_i] = card;
            card = &temp;
        }
    }
}

fn main() {
    let deck = Deck::generate_deck();
    deck.print_deck();
    deck.shuffle_deck();
    deck.print_deck();
}