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
}
