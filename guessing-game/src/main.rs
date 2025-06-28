use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("---> Type THE number (1 to 100): ");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failing to read the input");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Dont cheat me! Type a number!!!");
                continue;
            },
        };

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Small!!!"),
            Ordering::Equal => {
                println!("YAYYYYYYYY!!!!! You smart as hell ;)");
                break;
            },
            Ordering::Greater => println!("Big!!!"),
        }

    }
}
