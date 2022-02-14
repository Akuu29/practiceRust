extern crate rand;

use rand::Rng;
use std::collections::HashMap;

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
            int_value = 1;
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
        // for イテレーターの変更はできない
        while i < deck_size {
            let rand_i = rand::thread_rng().gen_range(0..=deck_size);
            let copied_card = self.deck[rand_i].clone();
            self.deck[rand_i] = self.deck[i].clone();
            self.deck[i] = copied_card;
            i += 1;
        }
    }
}

struct Table<'a> {
    players: Vec<Vec<Card>>,
    game_mode: &'a str,
    deck: Deck
}

impl Table<'_> {
    fn start_game(amount_of_players: u32, game_mode: &str) -> Table {
        let mut table = Table {
            players: vec![],
            game_mode: game_mode,
            deck: Deck::generate_deck()
        };

        table.deck.shuffle_deck();

        let mut i = 0;
        while i < amount_of_players {
            let mut player_card = vec![];

            let mut j = 0;
            while j < Dealer::initial_cards(table.game_mode) {
                player_card.push(table.deck.draw());
                j += 1;
            }
            table.players.push(player_card);
            i += 1;
        }

        table
    }
}

struct Dealer {}

impl Dealer {
    fn initial_cards(game_mode: &str) -> i32 {
        match game_mode {
            "poker" => 5,
            "21" => 2,
            _ => 0
        }
    }

    fn print_table_information(table: &Table) -> () {
        let player_len = table.players.len();
        println!("Amount of players: {}...Game mode: {}. At this table:", player_len, table.game_mode);

        let mut player = 0;
        while player < player_len {
            println!("Player {} hand is: ", player + 1);
            for card in &table.players[player] {
                println!("{}", card.get_cards_string());
            }
            player += 1;
        }
    }

    fn score_21_individual(cards: &Vec<Card>) -> u32 {
        let mut value = 0;
        let cards_len = cards.len();
        let mut count = 0;
        while count < cards_len {
            value += cards[count].int_value;
            count += 1;
        }
        if value > 21 {
            value = 0;
        }
        value
    }

    fn winner_of_21(table: &Table) -> () {
        let mut points = vec![];
        let mut hashmap = HashMap::new();

        for player in table.players.iter() {
            let point = Dealer::score_21_individual(player);
            let result = hashmap.insert(point.clone(), 1);
            match result {
                None => None,
                Some(value) => hashmap.insert(point.clone(), value + 1),
            };

            points.push(point);
        }

        println!("{:?}", &points);

        let winner_index = HelperFunction::max_in_array_index(&points);

        let number_of_winner = hashmap[&points[winner_index as usize]];
        
        if number_of_winner > 1 {
            println!("It is a draw");
        } else if number_of_winner >= 0 {
            let winner_player = winner_index.clone() + 1;
            println!("player {} is the winner", winner_player);
        }else {
            println!("No winners..");
        }
    }
}

struct HelperFunction {}

impl HelperFunction {
    fn max_in_array_index (int_arr: &Vec<u32>) -> u32 {
        let mut max_index = 0;
        let mut max_value = int_arr[0];

        let mut i = 0;
        for int in int_arr.iter() {
            if int > &max_value {
                max_value = int.clone();
                max_index = i;
            }
            i += 1;
        }
        max_index
    }
}

fn main() {
    let game_mode = "21";
    let table = Table::start_game(4, game_mode);

    Dealer::print_table_information(&table);
    Dealer::winner_of_21(&table)
}