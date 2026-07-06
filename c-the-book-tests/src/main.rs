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
        .expect("Failed to read number");

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

fn get_nth_fibonacci(n: u32) -> u32 {
    const FIRST_FIBONACCI_ELEMENT: u32 = 0;
    const SECOND_FIBONACCI_ELEMENT: u32 = 1;

    if n == 1 {
        FIRST_FIBONACCI_ELEMENT
    } else if n == 2 {
        SECOND_FIBONACCI_ELEMENT
    } else {
        let mut n_minus_2: u32 = FIRST_FIBONACCI_ELEMENT;
        let mut n_minus_1: u32 = SECOND_FIBONACCI_ELEMENT;
        let mut current_number: u32 = 0;
        let mut counter = 3;
        while counter <= n {
            current_number = n_minus_1 + n_minus_2;
            n_minus_2 = n_minus_1;
            n_minus_1 = current_number;
            counter += 1
        }
        current_number
    }
}

fn nth_fibonacci() {
    println!("Get the nth number in the Fibonacci sequence, input a whole number:");
    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read number");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number.");
            return;
        }
    };
    if input == 0 {
        println!("Please enter a number greater than zero.");
        return;
    }

    let nth_number: u32 = get_nth_fibonacci(input);
    println!("Fibonacci number #{input} is: {nth_number}")
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
