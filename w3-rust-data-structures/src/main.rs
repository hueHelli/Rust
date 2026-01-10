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
}
