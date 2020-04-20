fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 2;

        if counter == 120 {
            break counter * 4;
        }
    };

    println!("The result is {}", result);
}
