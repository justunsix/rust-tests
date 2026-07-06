use std::io;

fn print_menu() {
    let selections = [
        "1. Fahrenheit to Celsius",
        "2. Return nth Fibonacci Number",
        "3. Lyrics of Christmas carol 'The Twelve days of Christmas'",
    ];

    println!("======================\n");
    for element in selections {
        println!("{element}");
    }
    println!("Select menu item by number or q to exit:");
}

fn fahrenheit_to_celsius() {
    println!("Input a Fahrenheit value to convert to Celsius:");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read selection");

    let input: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number.");
            return;
        }
    };

    let celsius: f64 = (input - 32.0) / 1.8;
    println!("The celsius of {input} F is {celsius} C")
}
fn nth_fibonacci() {
    let nth_number: f64 = 0.0;

    println!("{nth_number}");
}

fn lyrics_of_christmas_carol() {}

fn main() {
    loop {
        print_menu();
        let mut menu_selection: String = String::new();

        io::stdin()
            .read_line(&mut menu_selection)
            .expect("Failed to read selection");

        let selection: u32 = match menu_selection.trim().parse() {
            Ok(num) => num,
            // if q, quit, otherwise continue
            Err(_) => {
                if menu_selection.trim() == "q" {
                    break;
                } else {
                    println!("Please enter a number.");
                    continue;
                }
            }
        };
        println!("You selected {selection}");

        if selection == 1 {
            fahrenheit_to_celsius();
        } else if selection == 2 {
            nth_fibonacci();
        } else if selection == 3 {
            lyrics_of_christmas_carol();
        } else {
            println!("Invalid menu selection, Enter a number from the menu.")
        }
    }
}
