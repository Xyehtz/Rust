fn main() {
    /* This code is set to fail, as the variable x cannot change, this is because the default setting when creating a variable is that it isn't mutable */
    // To solve this we add mut
    // ! This is known as a variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* On the other hand, constants are the values that are set to be immutable, meaning that they won't change value, constants are defined with the "const" keyword and need the type annotation
    They are written in upper snake*/
    const THREE_HOURS_IN_SECONDS: i32 = 3 * 60 * 60;

    // Shadowing
    /* Shadowing is the process of declaring a new variable using the name of an already created variable, what the compiler will see is the value of the second declaration  */
    // This is different from mutable variables because here we are declaring a new variable and assigning a value to it, not just assigning a new value to it, also, when a variable is mutable it needs to be assigned with a type equal to the original value, in this case there's no need
    let x = 5;
    let x = x +1;

    {
        let x = x * 2;
        println!("The inner value of x is: {x}");
    }

    println!("The value of x is: {x}");

    // ------[Lessons from Ownership are here]---------
    
    // There are two different types of strings in rust, the first one is the common string, which in Rust is immutable
    let _common_string = "Hello";

    // And the second one which is mutable, and in which we can add more text
    let mut new_string = String::from("Hello"); // Here we request memory to the memory allocator
    new_string.push_str(", world!"); // Now new_string changed from "Hello" to "Hello, world!"

    // In rust we can do the following
    let s1 = String::from("Hello");
    let s2 = s1

    // In other programming languages s1 and s2 will have the same data as they are a copy of each other, the problem comes when we look into the pointer, when s2 copies s1, the pointer will still point at the same place of s1, so when s1 or s2 drop we will have a double free error, for this reason when we copy s1 to s2, s1 is no longer valid

    // What we saw above is an example of a shallow copy, to make a deep copy we can do the following
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // This will copy ALL the data (both from the stack and the heap)

    println!("{new_string}")
}
