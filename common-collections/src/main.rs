use unicode_segmentation::UnicodeSegmentation; // Library for unicode segmentation
use std::collections::HashMap; // Import hashmaps

fn main() {
    let a = [1, 2, 3];

    // There are different collectors in Rust
    // We can create a vector like this and also add elements to it
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // We can directly add the elements to the vector
    let v2:Vec<i32> = vec![1, 2, 3];

    // If we want to access a vector we have two options
    // The first option is using indexes
    let third: &i32 = &v2[0]; // 1

    // The second option is using the get method, here we also need to use the match method
    // With this method we can avoid crashes when the desired index is out of range
    match v2.get(0) {
        Some(first) => println!("The first element is {}", first),
        None => println!("There is no first element")
    }

    // If we want to iterate over the vector we can do it easily with a for loop
    for i in &v {
        println!("{}", i);
    }

    // We can also use a mutable reference, this will change the original values
    for i in &mut v {
        *i * 50;
    }

    // We can store enums inside of a vector
    // Let's create the vector
    enum SpreadSheet {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Now lets add the enums into the vector
    let row: Vec<SpreadSheet> = vec![SpreadSheet::Int(5), SpreadSheet::Float(10.10), SpreadSheet::Text(String::from("Hello"))];

    // In the case where we want to access the characters in a string we need to use .bytes()
    for i in "Hello".bytes() {
        println!("{}", i);
    }

    // We can also use a variable
    let text: String = String::from("Hello");

    for i in &text {
        println!("{}", i);
    }

    // Here we have a string with the value of 뉴 메탈, which in English means Nu Metal
    let korean_text: String = String::from("뉴 메탈 ");

    // To access each value we need to do something called Grapheme Clusters
    // [뉴,  메, 탈]
    // To access each of these characters we need to import the unicode-segmentation library, look at the top of this file and in the Cargo.toml and you'll see

    // This will print each character of the word Nu Metal in korean, we need to use the graphemes method to be able to do it
    for g in &korean_text.graphemes(true) {
        println!("{}", g);
    }

    // To use hashmaps we first need to bring them to scope, and, to declare them we can do the following
    // Let's say we want to create a HashMap that will contain a game, we will put the team name as the keys and the score as the value
    let blue: String = String::from("Blue");
    let yellow: String = String::from("Yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    // To add values and keys to the hashmap we can do this
    scores.insert(blue, 2);
    scores.insert(yellow, 2);

    // We can get the values of a key by using the get method
    let team_blue: String = String::from("Blue");
    let blue_score: Option<&i32> = scores.get(&team_blue); // In this example we are using an option as the type of the variable because we are not sure if the value of that team exist, meaning it's an optional value

    // We can iterate through the values using a for loop
    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }

    // Let's say we have this string
    let second_text: String = String::from("Hello world, what a world");

    // Let's create a hashmap
    let mut map = HashMap::new();

    // Now let's add every word on the string into the hashmap and put as a value the amount of times that word appears
    // We are going to split the string into words
    for word in text.split_whitespace() {
        // This will go through every word in the string, if the word is not on the hashmap, it will add it into the hashmap and give it a value of 0, this value will be returned as a reference to count, and then, we are going to dereference it and add one to it
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)
}
