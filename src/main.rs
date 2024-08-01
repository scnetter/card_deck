// Struct - similar to a class in other languages
// Instances of Deck have different vectors
#[derive(Debug)]
struct Deck {
    cards: Vec<String>  // Vector - ordered list of values that can grow and shrink, arrays are fixed
}

fn main() {
    // List of suites - 'hearts' 'spades' 'clubs' 'diamonds'
    // List of values
    let suits = ["Hearts", "Clubs", "Diamonds", "Spades"];
    let values = ["Ace", "Two", "Three", "Four", "Five", "Six", "Seven",
        "Eight", "Nine", "Ten", "Jack", "Queen", "King"];
    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }
    // Rust Rover doesn't include the type inlays let deck: Deck
    let deck = Deck {cards};
    // vec![] is the same as Vec::new
    // :? is a debug formatter
    println!("here comes your deck: {:?}", deck);
}
