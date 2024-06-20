// Immutables and Scoping
fn main() {
    let x: i32 = 5;
    let x: i32 = x + 1;

    {
        println!("Global declaration from outer scope: {}", x);

        let x: i32 = (x * 2).into();
        println!("Inner scope: {}", x);

        let x: f64 = (x * 2).into(); // WE changed the type here, and its allowed.
        println!("Inner scope redeclaration f64: {}", x);
    }
    // even though the value and type of x was changed in the above scope
    // the 'x' swtiches back to its state before the scope started, i32 and x + 1 = 6;
    println!("Outer scope: {}", x);
}

// IMMUTABLES
// fn main()
// {
//     let _x: i32 = 10;
//     let _x: i32 = 20; // mutation is not allowed, but re-declaration is. prefix = _ needed

//     println!("{}", _x);
// }

// //CONSTANTS and variables
// fn main() {
//     const PI: f64 = 3.141; // constant
//     let x: i32 = 20; // immutable, is a variable
//                      // x = x + 1; // error
//     let mut is_true: bool = true; // mutable
//     is_true = is_true && x % 2 == 0; // Allowed, since mutable

//     if is_true {
//         println!("pi is {}", PI);
//     } else {
//         println!("x is {}", x)
//     }
// }

//use std::io::stdin;

// fn main() {
//     println!("Enter a number: ");
//     let mut str = String::new(); // By default a variable is immutable
//     stdin().read_line(&mut str).expect("Error!");
//     println!("Number: {}", str);
// }
