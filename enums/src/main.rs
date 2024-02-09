// We can create an enum that lists all the possible versions of an IP address
// We can let Rust know directly if we want to add more information like string or number like this
enum IpAddressKind {
    V4(String),
    V6(String),
}

// We can use enumms inside of structs too
struct IpAddress {
    version: IpAddressKind,
    address: String,
}

// In Rust there is no null value, this means that we also need to create a null value if we know we need it, for this we can use the option enum
// With this we'll be making sure we properly use an optional value or declare a null one
enum Option<T> {
    Some(T),
    None,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // We can create a variable of type enum IpAddressKind lie this, by using the namespace identifier
    // let version_four: IpAddressKind = IpAddressKind::V4;
    // let version_six: IpAddressKind = IpAddressKind::V6;

    // Use the struct created above
    // This can be improved even further, look at the enum declaration
    // let localhost: IpAddress = IpAddress {
    //     version: IpAddressKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // We can use the new version of our enum like this
    let localhost: IpAddressKind = IpAddressKind::V4(String::from("127.0.0.1"));
    let version_six_localhost: IpAddressKind = IpAddressKind::V4(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));

    // If we want to create an optional value or a null value we can do this
    let some_number: Option<i32> = Option::Some(5);
    let some_string: Option<&str> = Option::Some("Hello");
    let absent_number: Option<i32> = Option::None;

    // We can use optional values like integers to work with other integers
    let sum: i32 = 5 + some_number.unwrap_or(0); // In this case we will unwrap and obtain the value, if the value is null then it will default to zero

    // If we only care about one value, and not the rest we can use the if let statement check it
    let some_value: Option<i32> = Option::Some(20);
    if Option::Some(20) = some_value {
        // Here we only care about one value, in this case 20, therefore we will only check for it
        println!("Twenty")
    }
}

// We can use match to check the value of elements compared to others
// We need to clarify what is the outcome for each enum value, and we can even set code blocks inside of it
fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Dime => {
            println!("Dime");
            1
        }
        Coin::Nickel => 5,
        Coin::Penny => 10,
        Coin::Quarter => 25,
    }
}

// We can also use our enum as an argument on a function
fn route(ip_version: IpAddressKind) {
    //
}
