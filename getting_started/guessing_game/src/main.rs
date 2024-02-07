use std::io; // -> To obtain input we need the io (input output) library, this is inside of the std (standard) library, the :: acts something like . in Python at the moment of importing, we could say that std::io would be something like std.io in Python
use rand::Rng; // This is the dependency, and we are accessing a method to generate random numbers
use std::cmp::Ordering; // Bring a new dependency that has enums that are going to be useful to compare the user number and the generated number

fn main() { // -> Entry point of the program
    // Prompt the user to know what the game is
    println!("Guess the number!");
    
    // Create a random number using the rand dependency
    /* rand::thread_rng() is the function that is going to give us a particular number (the number is going to be local to the current thread and is seeded by the OS)
    Next we have the get_range method, this is defined by the Rng at the start of the program, this method accepts an expression as an argument, in this case the expression is the range of the number we want
    In this case gen_range will create a random number between 1 and 100 */
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Show what the number is
    // ? println!("The secret number is: {secret_number}");

    // Now we are going to put everything into a loop so the user can have multiple choices
    loop {
        // Prompt the user to choose a number
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

        /* In this line we are casting the guess variable, converting it from a String to an unassigned 32bit number, in this case we are using something called shadowing that allows us to shadow the original value of a variable and change it
        In this case we are also adding a colon at the start `guess:` after this we can see `u32`, this means that we are telling Rust that the guess variable is going to have an unsigned 32bit number as the value
        After this we reference the original guess variable which is a String and trim it to delete any type of whitespace (in this case guess had a `r\n\ whitespace because my OS is windows`)
        Then we parse the String to convert it to a number, this also works result enum that we saw on the input on line 27, in this case if Result is Err we will print "Please type a number" as the error */

        // Now with the following change we are going to change the behavior of the program so that when a user enters something that is not a number the game continues instead of crashing
        // ? let guess: u32 = guess.trim().parse().expect("Please type a number!");

        /* In this case parse is going to return a value either Ok or Err, and the match method is going to compare it to its arms (Ok and Err)
        Ok will have inside of it the number that we just parsed, when the match method matches with the Ok arm it will return the num variable and put it inside of guess, meaning that guess will now have the input of the user as a number
        
        In the case where the input is not a number the match method will match with the second arm Err, Err contains an underscore (_) meaning that is a catchall value, this means that we are going to match every type of error no matter the info inside of it, thus, making that when an input is not a number it will iterate again and ask for a number */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        // This line will print now the user input, thats why guess is inside of curly braces, because this is a placeholder that will hold the value in place
        println!("You guessed: {guess}");

        /* The cmp method is in charge of comparing two numbers, the user number and the generated number, this will return one of the three enums inside of Ordering, either Ordering::Greater, ::Equal or ::Less
        
        This value will be given to match and match will compare it to its arms, this means that if cmp returned Ordering::Greater it will go through each arm until it finds a patter that matches Ordering::Greater, and the execute whatever is after the arrow
        
        This means that if the cmp returns Ordering::Equal the match method is going to find a match with Ordering::Equal and print "You found the number" */
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("{guess} is way too big"),
            Ordering::Less => println!("{guess} is way too small"),
            Ordering::Equal => {
                println!("You guessed the number!");
                break;
            }
        }
    }
}
