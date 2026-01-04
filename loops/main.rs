fn main() {
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("Result is {result}");

    'counting_up: loop {
        count += 1;
        println!("Outer\n");
        loop {
            if count > 15 {
                break 'counting_up; // breaks outer loop
            }

            count += 1;
            println!("Inner\n");
        }
        println!("Count is {count}");
        if count == 20 {
            break;
        }
    }

    while count > 10 {
        count -= 2;
        println!("Count is {count}");
    }


    for number in 1..6 {
        println!("numer is {number}");
    }
}