extern crate rand;

use rand::Rng;

#[derive(Clone, Debug)]
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

    fn draw(&mut self) -> Card {
        let card = self.deck.pop(); // optionで返却される
        card.unwrap()
    }

    fn print_deck(&self) -> () {
        println!("Displaying cards...");
        for card in self.deck.iter() {
            println!("{}", card.get_cards_string());
        }
    }

    fn shuffle_deck(&mut self) -> () {
        let deck_size = self.deck.len() - 1;
        let mut i = 0;
        while i < deck_size {
            let rand_i = rand::thread_rng().gen_range(0..=deck_size);
            let copied_card = self.deck[rand_i].clone();
            self.deck[rand_i] = self.deck[i].clone();
            self.deck[i] = copied_card;
            i += 1;
        }
    }
}

struct Dealer {}

impl Dealer {
    fn start_game(amount_of_players: u32, game_mode: &str) -> Vec<Vec<Card>> {
        struct Table {
            players: Vec<Vec<Card>>,
            deck: Deck
        }

        let mut table = Table {
            players: vec![],
            deck: Deck::generate_deck()
        };

        table.deck.shuffle_deck();

        let mut i = 0;
        while i < amount_of_players {
            let mut player_card = vec![];

            let mut j = 0;
            while j < Dealer::initial_cards(game_mode) { // 手札2枚
                player_card.push(table.deck.draw());
                j += 1;
            }
            table.players.push(player_card);
            i += 1;
        }

        table.players
    }

    fn initial_cards(game_mode: &str) -> i32 {
        match game_mode {
            "poker" => 5,
            "21" => 2,
            _ => 0
        }
    }
}

fn main() {
    let dealer = Dealer::start_game(4, "poker");
    println!("{:?}", dealer);
}