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
    let myGrade: char = 'A';
    println!("{}", myGrade);

    // Booleans
    let is_logged_in: bool = true;
    println!("User logged in? {}", is_logged_in);

    // Constants (must have a type)
    const PI: f64 = 3.1415; // Declaring constants with uppercase is good practise
    println!("Pi: {}", PI);
}
