// Simple program to print "Hello world!"
// Used command `rustc main.rs` and created main.exe and main.pdb files (compiled the main.file)

fn main() { /* -> This is a function (fn is the keyword)
  main is the name of the function (in rust similar to Java and C#, all the programs start with the main function)
  () will hold parameters
  And inside the curly braces we find the body function */

    println!("Hello world!"); /* There are different interesting things here:
    First: In rust the indentation is 4 SPACES not tabs
    Second: println! is calling a Rust macro not a function this is thanks to the !, macros don't always follow function rules
    Third: "Hello world!" is the string passed to the println! macro
    Fourth: Most of the lines end in ; */
}