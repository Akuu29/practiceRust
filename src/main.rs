struct Card {
    suit: String,
    value: String,
    int_value: u64
}

fn build_card(suit: String, value: String, int_value: u64) -> Card {
    Card {
        suit,
        value,
        int_value
    }
}

impl Card {
    fn getCardString(&self) -> self {
        
    }
}

fn main() {
    let card1 = build_card (
        String::from("♦︎"),
        String::from("A"),
        1
    );

    // let a = card1.getCardString();

    // dbg!(a);
}