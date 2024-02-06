use std::io; // -> To obtain input we need the io (input output) library, this is inside of the std (standard) library, the :: acts something like . in Python at the moment of importing, we could say that std::io would be something like std.io in Python

fn main() { // -> Entry point of the program
    // Prompt the user to know what the game is and what to do
    println!("Guess the number!");
    println!("Please input your guess...");

    /* Now we create a string that will save the input of the user
        In this case we add the keyword mut, meaning that the guess variable can mutate
        String::new() means that we are creating a string but not initializing it by adding a value, not yet */
    let mut guess = String::new();

    /* We access the input output library and call an associate function of it
        This is in charge of adding the value that the user inputs to the guess variable */
    io::stdin()
        .read_line(&mut guess) /* Call the read_line method on the input handle and passing `&mut guess` as the argument, so that rust adds the value of the user to the variable guess
        The job of read_line is to take what the user types and append it to the variable, the string argument needs to be mutable to change the content of the string
        ? In this example we see &, this means a reference to the guess variable allowing us to access that part of code from anywhere without copying the data multiple times
        References can be mutable too, thats why we add the mut keyword*/

        // ----------------------------------------------------------------

        /* read_line always returns something called Result, this is an enum, and it can be either Ok or Err, Ok meaning the operation was successful and Err meaning that the operation failed
        This values also contain methods, in this case expect
        I the result indeed is Err expect will crash the program and print the message inside of the parenthesis
        If the result is Ok expect will return the value to us so we can use it later
        ! In a case where we don't use expect, Rust will show a warning message*/
        .expect("Failed to read input.");

    // This line will print now the user input, thats why guess is inside of curly braces, because this is a placeholder that will hold the value in place
    println!("You guessed: {guess}");
}
