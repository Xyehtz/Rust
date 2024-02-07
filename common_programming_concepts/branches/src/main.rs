fn main() {
    let number: i32 = 7; // Declare a number with value 3

    // Check if the number is less than 5
    if number < 5 {
        println!("Condition was true"); // Print if the number is less than 5
    } else if  number == 0{ // add an if else statement to handle multiple if statements
        println!("Number is not equal to zero")
    } else {
        println!("Condition was false"); // Print if the number is greater than 5
    }

    // We can even declare variables by using an if statement at the right side of the equality
    let condition: bool = true;
    let number: i32 = if condition { 7 } else { 0 };
}
