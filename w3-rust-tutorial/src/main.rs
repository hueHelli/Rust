use std::fmt::format;

fn main() {
    // Strings
    let mut name: &str = "Aaron"; // mut to make the variable "mutable"(changeble)
    println!("Hello, {}", name);
    name = "Sigma";
    println!("Hello, {}", name);

    // Whole numbers
    let age: i32 = 16;
    println!("Age is: {}", age);

    // Floating point numbers
    let price: f64 = 69.69;
    println!("Price is: {}", price);

    // Characters
    let my_grade: char = 'A';
    println!("{}", my_grade);

    // Booleans
    let is_logged_in: bool = true;
    println!("User logged in? {}", is_logged_in);

    // Constants (must have a type)
    const PI: f64 = 3.1415; // Declaring constants with uppercase is good practise
    println!("Pi: {}", PI);

    // Arithmetic Operators
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    // Assignment Operators
    let mut x = 10;
    println!("Start: {}", x);

    x += 5;
    println!("After += 5: {}", x);

    x -= 2;
    println!("After -= 2: {}", x);

    x *= 2;
    println!("After *= 2: {}", x);

    x /= 3;
    println!("After /= 3: {}", x);

    x %= 4;
    println!("After %= 4: {}", x);

    // Comparison Operators
    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b);
    println!("5 != 10: {}", a != b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);

    // Logical Operators
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin);
    println!("Has any access: {}", logged_in || is_admin);
    println!("Not logged in: {}", !logged_in);

    // Control Structures
    // If / Else
    let is_logged_in: bool = true;

    if is_logged_in {
        println!("Welcome back!");
    } else {
        println!("Please log in.");
    }

    let is_allowed: &str = if is_logged_in { "Yes" } else { "No" };
    println!("{}", is_allowed);

    // Match
    let day: i32 = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    let result = match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day.",
    };

    println!("{}", result);

    // Loops
    // Loop
    let mut count = 1;
    loop {
        println!("Hello World!");

        if count == 3 {
            break;
        }

        count += 1;
    }

    count = 1;
    let result = loop {
        println!("Hello!");

        if count == 3 {
            break count; // Stop the loop and return the number 3
        }

        count += 1;
    };
    println!("The loop stopped at: {}", result);

    // While
    count = 1;
    while count <= 5 {
        println!("Count: {}", count);
        count += 1;
    }

    while count <= 5 {
        println!("This won't be printed.");
    }
    // You can also use break and continue for While-Loops

    // For
    for i in 1..6 {
        println!("i is: {}", i);
    }
    for i in 1..=6 {
        println!("i is: {}", i);
    }
    // You can also use break and continue for For-Loops

    // Functions
    say_hello();
    greet("Aaron");
    println!("Sum is: {}", sum(3, 5));

    // Scope: Values only accessible on same or deeper layer.
    // If the a new variable with the same name on a deeper layer is declared, the initial variable isn't affected

    // Strings
    // &str is for strings that don't change
    // String is for strings that can change
    let text1: String = "Hello World".to_string();
    let text2: String = String::from("Hello World");

    // Use push_str() to add text to a string
    // Use push() to add a single char to a string
    let mut greeting: String = String::from("Hello");
    greeting.push_str(" World");

    // You can combine strings with the format!() macro, similar to print!()
    let text1: String = String::from("Hello");
    let text2: String = String::from("World");
    let result: String = format!("{} {}", text1, text2); // Hello World
    // You can also use + but it's cleaner with format!()
    // Use len() for the length of a String
    let result_length = result.len();

    // Ownership
    let a = String::from("Hello");
    let b = a;
    // println!("{}", a); Error: a no longer owns the value
    println!("{}", b); // Ok: b now owns the value 

    let a = String::from("Hello");
    let b = a.clone(); // Now both have the same value
    println!("a = {}", a); // Works
    println!("b = {}", b); // Works 
}

fn say_hello() {
    println!("Hello from a function")
}

fn greet(name: &str) {
    println!("Hello {}", name)
}

fn sum(a: i32, b: i32) -> i32 {
    return a + b;
    // Would work theoretically as well, but I don't like it
    // a + b // Without return and without semicolon
}
