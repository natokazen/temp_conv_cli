use std::io::{self, Write};

fn main() {
    println!(r#"
        ░▀█▀░█▀▀░█▄█░█▀█░░░█▀▀░█▀█░█▀█░█░█░░░█▀▀░█░░░▀█▀
        ░░█░░█▀▀░█░█░█▀▀░░░█░░░█░█░█░█░▀▄▀░░░█░░░█░░░░█░
        ░░▀░░▀▀▀░▀░▀░▀░░░░░▀▀▀░▀▀▀░▀░▀░░▀░░░░▀▀▀░▀▀▀░▀▀▀

    "#);
    // println!("   [ Options: Fah to Cels [1] | Cels to Fah [2] | Quit [q] ]");

        let mut number = String::new();

        print!("Enter a number: ");
        io::stdout().flush().expect("Flush failed");

        io::stdin().read_line( &mut number).expect("Failed to read input!");
        let number :f64 = number.trim().parse().expect("Failed to parse input");

        let calc_fah_to_cels = (number - 32.0 ) * 5.0 / 9.0;
        println!("Number converted is : {}", calc_fah_to_cels.floor());

}
