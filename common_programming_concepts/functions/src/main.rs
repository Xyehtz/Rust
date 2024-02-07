// fn main() {
//     println!("Hello world!");

//     // Call another function
//     another_function(5); // Pass a value for the parameter
// }

// Create a new function using fn keyword and add code inside
// fn another_function(x: i32) { // Now we add a parameter to the function
//     println!("The value of x is: {x}");
// }

// ----------------------------------------------------------------

fn main() {
    print_labeled_measurement(10, 'h')
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}
