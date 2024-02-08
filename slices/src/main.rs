fn main() {
    // This code will create a string on the heap and send it to the first_word function, to then clear it
    let mut s: String = String::from("Hello world");
    let s2: &str = "Hello world!";
    let word: &str = first_word(s2);

    // ----------------------------------------------------------------
    // To solve this we can use slices, in this case we are accessing the variables as a reference, but not all the variable, just a slice of it, we could say that we have two pointers, one starting at index 0 and going to index 5, and one at index 6 going to index 11
    // let hello: &str = &s[0..5]; // This can be further improved
    // let hello: &str = &s[..5];
    // let world: &str = &s[6..11]; // This can be further improved
    // let world: &str = &s[6..];
    s.clear();
}

fn first_word(s: &String) -> &str { // Receive a reference to a string and return a usize variable
    let bytes: &[u8] = s.as_bytes(); // Convert the string to bytes

    for (i, &item) in bytes.iter().enumerate() { // Enumerate each byte, when a byte is a white space, return the index value
        if item == b' ' {
            // return i; // Instead of this, do this
            return &s[..i]
        }
    }

    // If there's no white space return the length of the string
    // s.len() // Instead of this, do this
    &s[..]
}
