fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(10, 's');

    let y = {
        let x = 3;
        x + 1 // note the lack of a semicolon!
    };
    println!("the value of y is: {y}");

    let five = five();
    println!("The value of five is... {five}");

    println!("{}", successor(5));

    println!("{}", is_big_number(150));

    // using if in a let statement
    let condition = true;
    let z = if condition { 5 } else { 10 };
    println!("z = {z}");
}

// in all function signatures we MUST declare the type of each parameter
fn another_function(x: i32) {
    println!("Wowee! The value of x is {x}!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn five() -> i32 {
    5
}

fn successor(x: i32) -> i32 {
    // return x + 1;
    x + 1
}

fn is_big_number(num: i32) -> bool {
    if num > 100 {
        true
    } else {
        false
    }
}
