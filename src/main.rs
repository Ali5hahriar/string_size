fn main() {
    let refrence_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("Hello");

    &s
}

// The Rules of Refrences
// 1. At any given time, you can have either one mutable refrence 
// or any numbe of immutable refrences.
// 2. Refrences must always be valid.
