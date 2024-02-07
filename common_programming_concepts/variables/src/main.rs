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
}
