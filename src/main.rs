use std::io::{self, Write};

fn main() {
    println!(
        r#"
        ░▀█▀░█▀▀░█▄█░█▀█░░░█▀▀░█▀█░█▀█░█░█░░░█▀▀░█░░░▀█▀
        ░░█░░█▀▀░█░█░█▀▀░░░█░░░█░█░█░█░▀▄▀░░░█░░░█░░░░█░
        ░░▀░░▀▀▀░▀░▀░▀░░░░░▀▀▀░▀▀▀░▀░▀░░▀░░░░▀▀▀░▀▀▀░▀▀▀

    "#
    );
    println!("   -------------------- Main Menu ------------------------- \n");
    println!("   [ Options: Fah to Cels [1] | Cels to Fah [2] | Quit [q] ]");

    loop {
        let mut choice = String::new();

        print!("\n Enter an option: ");
        io::stdout().flush().expect("Flush failed");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input!");
        let choice = choice.trim();

        match choice {
            "q" => {
                println!("\n   ------------------ Quitting Program! ------------------- ");
                break;
            }
            "1" => {
                println!("\n Chose option [1]");
                print!("   󰘍 Enter a fahrenheit value: ");
                io::stdout().flush().expect("Flush failed!");

                match read_temprature() {
                    Some(num) => {
                        let celsius = calc_fahrenheit_to_celsius(num);
                        println!("\n   󰁔 Result: {}°F converted is {}°C", num, celsius.round());
                    }
                    None => {
                        println!("\n -- Returning to main menu...\n");
                    }
                }
            }
            "2" => {
                println!("\n Chose option [2]");
                print!("   󰘍 Enter a celsius value: ");
                io::stdout().flush().expect("Flush failed!");

                match read_temprature() {
                    Some(num) => {
                        let fahrenheit = calc_celsius_to_fahrenheit(num);
                        println!("\n   󰁔 Result: {}°C converted is {}°F", num, fahrenheit.round());
                    }
                    None => {
                        println!("\n -- Returning to main menu...\n");
                    }
                }

            }
            _ => {
                println!("\n Please enter a valid choice or press 'q' to Quit.");
            }
        }
    }
}

// ---------------------- COMPUTE FUNCTIONS -----------------------------

fn calc_fahrenheit_to_celsius(num: f64) -> f64 {
    (num - 32.0) * 5.0 / 9.0
}

fn calc_celsius_to_fahrenheit(num: f64) -> f64 {
    (num * 9.0 / 5.0) + 32.0
}

// ------------------------ HELPER FUNCTIONS  ------------------------------

fn read_temprature() -> Option<f64> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    if input == "q" || input == "quit" || input == "exit" {
        return None;
    }

    match input.parse::<f64>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Not a valid input, please enter a number or press 'q' to quit.");
            None
        }
    }
}
