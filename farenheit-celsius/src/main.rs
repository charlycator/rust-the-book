use std::io;

fn farenheit_to_celsius(farenheit_number: f32) -> f32 {
     (farenheit_number - 32.0) / 1.8
}

fn celsius_to_farenheit(celsius_number: f32) -> f32 {
    (celsius_number * 1.8) + 32.0
}

fn get_option () -> String {
    let mut option = String::new();

    println!("Choose a conversion:");
    println!("1- Farenheit to Celsius");
    println!("2- Celsius to Farenheit");

    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read your option");

    option
}

fn get_user_degrees() -> f32 {
    let mut typed_degrees = String::new();

    println!("Degrees:");

    io::stdin()
        .read_line(&mut typed_degrees)
        .expect("Failed to read the degrees");

    typed_degrees.trim().parse().unwrap_or_else(|_| {
        println!("Error typing degrees");
        0.0
    })

}

fn main() {
    loop {
        let option: String = get_option();
        let result: f32;
        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let typed_degrees: f32 = get_user_degrees();

        if option == 1 {
            result = farenheit_to_celsius(typed_degrees);

            println!("{typed_degrees} Farenheit degrees are {result} Celsius degrees");
        } else if option == 2 {
            result = celsius_to_farenheit(typed_degrees);

            println!("{typed_degrees} Celsius degrees are {result} Farenheit degrees");
        }
    }
}
