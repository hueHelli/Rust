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
}
