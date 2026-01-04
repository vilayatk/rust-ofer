fn main() {
    let s1 = String::from("hello!");
    // let s2 = s1; // compiler error!
    let s2 = s1.clone();
    println!("{s2}");


    let s3 = give_ownership();
    println!("{s3}");

    take_ownership(s3);
    //println!("s3 is no longer valid {s3}"); // Compiler error

}

fn give_ownership() -> String {
    let s = String::from("Hello World!");
    s
}

fn take_ownership(x:String) {
   println!("Taken ownership {x}"); 
}

//fn dangle() -> &String {
//    let s = String::from("Hello!");
//    &s
//}