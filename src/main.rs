fn main() {
    let s1 = String::from("Hello world");
    let len = calculate_length(&s1); // We do not take ownership of s1, we just refrencing to it, in other word we 'barrow' it.
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() return the length of a String
    length
}
