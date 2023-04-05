fn main() {
    let its_cold = 12.0;
    println!("brr.. {} celsius", fahr_to_cels(its_cold));

    let a = fact(10);
    println!("{}", a);

    twelve_days();
}

fn fahr_to_cels(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}

fn cels_to_fahr(temp: f64) -> f64 {
    1.8 * (temp + 32.0)
}

fn fact(num: i32) -> i32 {
    if num == 0 {
        1
    } else {
        num * fact(num - 1)
    }
}

fn twelve_days() {
    let giftNames = [
        "A partridge in a pear tree",
        "Two turtledoves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut day = 0;
    let mut gifts = String::new();
    for gift in giftNames {
        gifts += gift;
        day += 1;
        println!("On the {day} of christmas my true love sent to me {gifts}");
    }
}