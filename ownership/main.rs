fn main() {
    let s1 = String::from("hello!");
    // let s2 = s1; // compiler error!
    let s2 = s1.clone();
    println!("{s2}");


    let s3 = give_ownership();
    println!("{s3}");

    take_ownership(s3);
    //println!("s3 is no longer valid {s3}"); // Compiler error
    let mut x = String::from("Something: ");
    borrowing_and_updating(&mut x); // passing as reference and mutable.

    println!("{x}");

    borrow_and_return_object(&x);
    println!("X still exists after the call. Borrowed and returned back.");

    mutability_and_references();
}

fn give_ownership() -> String {
    let s = String::from("Hello World!");
    s
}

fn take_ownership(x:String) {
   println!("Taken ownership {x}"); 
}

fn borrowing_and_updating(x : &mut String) {
    x.push_str(". String is appended!");
}

fn borrow_and_return_object(x: &String) {
    println!("Some usage of {x}");
    println!("Returns the object back");
}

//fn dangle() -> &String {
//    let s = String::from("Hello!");
//    &s
//}

fn mutability_and_references() {
    let mut r = String::from("This is a var.");
    r.push_str(". XYZ.");
    let x = &r;
    let y = &r;
    // let z = &mut x; // NOT Ok
    //x.push_str("Rasengan!");
    println!("{x}\n {y}");
    let z = &mut r; // OK. Because x and y are not used after this assignment
    println!("{z}");
    // You cannot mix mutable and immutable refernces. This is because rust prevents data races at compilation
}