// Struct - similar to a class in other languages
// Instances of Deck have different vectors
#[derive(Debug)]
struct Deck {
    cards: Vec<String>  // Vector - ordered list of values that can grow and shrink, arrays are fixed
}

fn main() {
    // Rust Rover doesn't include the type inlays let deck: Deck
    let deck = Deck {cards: vec![] };
    // vec![] is the same as Vec::new
    // :? is a debug formatter
    println!("here comes your deck: {:?}", deck);
    println!("Test");
}
