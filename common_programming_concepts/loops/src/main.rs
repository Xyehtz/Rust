fn main() {
    // This will create an infinite loop of printing Hello, world!
    // loop {
    //     println!("Hello, world!");
    // }

    // We can also define values using loops
    let mut counter: i32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is: {result}");

    // We can add labels to loops so that we can use continue and break from inside nested loops
    'counting_up: loop {
        println!("Count = {counter}");
        let mut remaining: i32 = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }

    println!("End count: {counter}");

    // Both for and while loops are very similar to what we can see in python or in JS
}
