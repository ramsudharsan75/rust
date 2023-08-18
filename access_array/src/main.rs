use std::io;

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Enter the index to access");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Index was not a number");

    let element = arr[index];

    println!("The element is {element}");
}
