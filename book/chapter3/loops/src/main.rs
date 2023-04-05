fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("current count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining iterations = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("end count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for ele in a {
        println!("the value is: {ele}");
    }

    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("BLASTING OFF AGAIN!!!");
}
