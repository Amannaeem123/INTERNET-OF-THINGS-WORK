fn main() {
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 6 == 0 {
        println!("number is divisible by 6");
    } else {
        println!("number is not divisible by 4, 3, or 6");
    }
}

