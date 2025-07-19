use std::io;

fn calc_fibo(num: i64, prev: i64, curr: i64) -> i64 {
    println!("NUM: {num} -- prev: {prev} -- curr: {curr}");

    if num == 0 {
        prev
    } else if num == 1 {
        curr
    } else {
        calc_fibo(num - 1, curr, prev + curr)
    }
}

fn main() {
    let mut typed_number = String::new();

    println!("Give me a number");

    io::stdin()
        .read_line(&mut typed_number)
        .expect("Failed to read the number you typed");

    let typed_number = typed_number
        .trim()
        .parse()
        .expect("Type a number!!!");
    let result: i64 = calc_fibo(typed_number, 0, 1);

    println!("Typed number -- {typed_number}");

    println!("Fibo calculation is {result}");
}
