#[derive(Debug)] // This allows the compiler to add a basic implementation of the debug trait
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { // This implementation of the Rectangle struct will add a method to any instance
    fn area(&self) -> u32 { // This method is defined the same way as the instance, but because this is an implementation we can easily refer to the Struct and play with the values of it by using the self keyword
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

impl Rectangle { // This is an associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

struct User { // Create a new structure that will contain a username, an email, a sing in count and a is active boolean
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

fn main() {
    let mut user_one: User = User { // Create a mutable instance of User
        email: String::from("user@example.com"),
        username: String::from("Xyehtz"),
        is_active: true,
        sign_in_count: 1
    };

    let mut name = user_one.username; // Create a variable based from the user_one User instance
    user_one.username = String::from("Xyehtz_new"); // Change the value of the username

    let user_two: User = build_user( // Create a user instance with the help of the build_user function
        String::from("askwl@gmail.com"),
        String::from("new_Xyehtz")
    );

    let user_three: User = User { // Create an instance of User and determine the las values based on the values of user_two
        email: String::from("jasnu@outlook.com"),
        username: String::from("jasnuu123"),
        ..user_two
    };

    // This is a tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Here we have a use case of a tuple struct, let's say we want to know the area of a rectangle, we can create a tuple that contains the dimensions of our rectangle
    // let rectangle_dimensions: (u32, u32) = (30, 30); // This is a little bit ambiguous because we don't know which one is the width or which one is the height, we can fix this with a normal struct (look at line 1)

    // In this example we created a struct that lets us know which field is what
    let rectangle_dimensions: Rectangle = Rectangle {
        width: 30,
        height: 30
    };

    // What if we wanted to print this? We can do it, but not so easy, we need to add some changes to the println! statement
    // The first problem comes with the Display trait, this means that Rectangle doesn't know how to show itself when printed
    // The second problem come with the debug trait, this means that Rectangle doesn't print useful information to devs, to solve this we can add #[derive(Debug)] at the top of the Rectangle struct
    println!("{:?}", rectangle_dimensions);

    // In this print statement we call the area function to get the area of the rectangle based on the rectangle dimensions that we have above
    println!("The are of the rectangle is: {}", rectangle_dimensions.area());

    let one_more_rect: Rectangle = Rectangle {
        width: 30,
        height: 30
    };
    let second_more_rect: Rectangle = Rectangle {
        width: 20,
        height: 20
    };
    let last_square: Rectangle = Rectangle::square(25); // We can access associated fucntions like this

    println!("Can rect 1 hold rect 2: {}", one_more_rect.can_hold(&second_more_rect));
    println!("Can rect 2 hold rect 1: {}", second_more_rect.can_hold(&second_more_rect));
}

// This function will receive a tuple struct, and return an unsigned integer
// To use a struct we need to change it to reference said struct
// We can improve this even further by using an implementation, thus, adding this function as a method of the Rectangle instance
// fn area(rectangle: &Rectangle) -> u32 {
//     // We access the values of the tuple using a dot
//     // dimensions.0 * dimensions.1
//     rectangle.height * rectangle.width
// }

// Function that will return an instance of an user, the function will receive two of the arguments of the struct
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        is_active: false
    }
}
