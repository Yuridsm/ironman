fn main() {
    println!("{} days", 31);
    println!("{} days", 31i64);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // You can right-align text with a specified width. This will output
    // "     1". 5 white space and a "1".
    println!("{number:width$}", number = 1, width = 5);
}
