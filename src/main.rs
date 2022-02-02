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

fn main() {
    let card1 = Card {
        suit: String::from("♦︎"),
        value: String::from("A"),
        int_value: 1,
    };
    println!("{}", card1.get_cards_string());
}