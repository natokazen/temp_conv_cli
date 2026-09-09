use std::io::{self, Write};

fn main() {
    println!(
        r#"
        ░▀█▀░█▀▀░█▄█░█▀█░░░█▀▀░█▀█░█▀█░█░█░░░█▀▀░█░░░▀█▀
        ░░█░░█▀▀░█░█░█▀▀░░░█░░░█░█░█░█░▀▄▀░░░█░░░█░░░░█░
        ░░▀░░▀▀▀░▀░▀░▀░░░░░▀▀▀░▀▀▀░▀░▀░░▀░░░░▀▀▀░▀▀▀░▀▀▀

    "#
    );
    println!("   [ Options: Fah to Cels [1] | Cels to Fah [2] | Quit [q] ]");

    loop {

        let mut option_input = String::new();

        print!("\n Enter an option: ");
        io::stdout().flush().expect("Flush failed");

        io::stdin().read_line(&mut option_input).expect("Failed to read input!");
        let option_input = option_input.trim();

        if option_input == "1" {
            let mut number = String::new();

            print!("\n Chose option [1] | Enter a number: ");
            io::stdout().flush().expect("Flush failed");

            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read input!");
            let number: f64 = number.trim().parse().expect("Failed to parse input");

            let calc_cels = calc_fah_to_cels(number);
            println!(" Number converted Fahrenheit to Celsius is : {}°C", calc_cels.round());
            break;

        } else if option_input == "2" {
            let mut number = String::new();

            print!("\n Chose option [2] | Enter a number: ");
            io::stdout().flush().expect("Flush failed");

            io::stdin()
                .read_line(&mut number)
                .expect("Failed to read input!");
            let number: f64 = number.trim().parse().expect("Failed to parse input");

            let calc_fah = calc_cels_to_fah(number);

            println!(" Number converted from Celsius to Fahrenheit is : {}°F", calc_fah.round());
            break;
        }
    }
}

fn calc_fah_to_cels(num: f64) -> f64 {
    (num - 32.0) * 5.0 / 9.0
}

fn calc_cels_to_fah(num: f64) -> f64 {
    (num + 32.0) * 9.0 / 5.0
}
