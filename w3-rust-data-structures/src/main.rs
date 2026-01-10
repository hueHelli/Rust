fn main() {
    // Arrays
    // Arrays are lists of a fixed value that can't be changed in size
    let mut numbers = [1, 2, 3, 4, 5];
    println!("The third number is: {}", numbers[2]);
    // The values inside the array can be changed though
    numbers[2] = 10;
    println!("The third number is: {}", numbers[2]);

    // Loop through an array
    let fruits = ["apple", "banana", "orange"];
    for fruit in fruits {
        println!("I like {}.", fruit);
    }

    // When printing the whole array, you must use {:?} inside println!()
    let numbers = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);
    // You can also use len() on Arrays

    // Vectors
    // A vector is a reziable array, it has all the functions an array has as well. To create one use the vec! macro
    let mut fruits = vec!["apple", "banana"];
    fruits.push("cherry");
    println!("{:?}", fruits); // ["apple", "banana", "cherry"]
    fruits.pop();
    println!("{:?}", fruits); // ["apple", "banana"]

    // You can also insert values at specific indexes
    let mut fruits = vec!["banana", "orange"];
    fruits.insert(0, "apple");
    println!("{:?}", fruits); // ["apple", "banana", "orange"]
    fruits.remove(0); // The lower the index the slower it is, because a lot of items have to shift positions
    println!("{:?}", fruits); // ["banana", "orange"]

    // When you loop through a Vector you should use a reference, so you can still use it later

    // Tuples
    // Tuples are like Arrays (not resizable) but with different types
    // Tuples are written using parentheses (), with values separated by commas
    let person = ("John", 30, true);
    // For accessing tuple values you can use a . followed by the index
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is active: {}", person.2);
    // You can also "unpack" a tuple (like destructuring in JavaScript)
    let person = ("Jenny", 45, false);
    let (name, age, active) = person;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", active);

    // You can also return a Tuple from a function
    let user = get_user();
    println!("User: {} ({} years old)", user.0, user.1);
}

fn get_user() -> (String, i32) {
    (String::from("Liam"), 25)
}
