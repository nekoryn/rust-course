use std::io::{stdin, Write};

fn main() {
    print!("Please enter your name: ");
    std::io::stdout().flush().unwrap();

    let mut user_name = String::new();
    match stdin().read_line(&mut user_name) {
        Ok(n) => {
            println!("{n} bytes read");
        } 
        Err(e) => {
            println!("Error reading input: {}", e);
        }
    }

    print!("Please enter your age: ");
    std::io::stdout().flush().unwrap();

    let mut age_input = String::new();
    match stdin().read_line(&mut age_input) {
        Ok(n) => {
            println!("{n} bytes read");
        }
        Err(e) => {
            println!("Error reading input: {}", e);
        }
    }

    print!("Please enter your weight: ");
    std::io::stdout().flush().unwrap();

    let mut weight_input = String::new();
    match stdin().read_line(&mut weight_input) {
        Ok(n) => {
            println!("{n} bytes read");
        }
        Err(e) => {
            println!("Error reading input: {}", e);
            return;
        }
    }

    print!("Please enter your height: ");
    std::io::stdout().flush().unwrap();

    let mut height_input = String::new();
    match stdin().read_line(&mut height_input) {
        Ok(n) => {
            println!("{n} bytes read");
        }
        Err(e) => {
            println!("Error reading input: {}", e);
            return;
        }
    }

    let weight: f32 = match weight_input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input, try again");
            return;
        }
    };

    let height: f32 = match height_input.trim().parse() {
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input, try again");
            return;
        }
    };

    let age: u8 = match age_input.trim().parse(){
        Ok(val) => val,
        Err(_) => {
            println!("Invalid input, try again");
            return;
        }
    };

    let bmi = weight / (height / 100.0).powi(2);

    println!("Hello, {}! You are {} years old, weigh {} kg, and are {} cm tall.", user_name.trim(), age, weight, height);
    println!("Your BMI is: {:.2}", bmi);
}