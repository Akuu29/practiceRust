#[derive(Debug)]

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
}

fn main() {
    let deck = Deck::generate_deck();
    println!("{:?}", deck.deck);
}