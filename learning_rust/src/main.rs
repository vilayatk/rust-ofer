use std::io::stdin;

fn main() {
    println!("Enter a number: ");
    let mut str = String::new();
    stdin().read_line(&mut str).expect("Error!");
    println!("Number: {}", str);
}
