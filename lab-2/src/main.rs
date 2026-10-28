use std::io::Write;

fn add(a: i32, b: i32) -> i32 {
    a + b
}
    
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> f64 {
    a as f64 / b as f64
}

fn main() {
    let a: i32 = loop {
        print!("Введите первое число: ");
        std::io::stdout().flush().unwrap();
    
        let mut a = String::new();
        match std::io::stdin().read_line(&mut a) {
            Ok(0) => {
                eprintln!("Ошибка: введена пустая строка");
                return;
            },
            Ok(_) => {},
            Err(_) => {
                eprintln!("Ошибка при чтении числа");
                return;
            }
        };

        match a.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                eprintln!("Ошибка: введено не число");
            }
        };
    };

    let b: i32 = loop {
        print!("Введите второе число: ");
        std::io::stdout().flush().unwrap();
    
        let mut b = String::new();
        match std::io::stdin().read_line(&mut b) {
            Ok(0) => {
                eprintln!("Ошибка: введена пустая строка");
                return;
            },
            Ok(_) => {},
            Err(_) => {
                eprintln!("Ошибка при чтении числа");
                return;
            }
        };

        match b.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                eprintln!("Ошибка: введено не число");
            }
        };
    };
    let op = loop {
        print!("Введите операцию (+, -, *, /): ");
        std::io::stdout().flush().unwrap();
    
        let mut op = String::new();
        match std::io::stdin().read_line(&mut op) {
            Ok(0) => {
                eprintln!("Ошибка: введена пустая строка");
                return;
            },
            Ok(_) => {
                break op.trim().to_string();
            },
            Err(_) => {
                eprintln!("Ошибка при чтении операции");
                return;
            }
        };
    };

    match op.as_str() {
        "+" => println!("Результат: {}", add(a, b)),
        "-" => println!("Результат: {}", subtract(a, b)),
        "*" => println!("Результат: {}", multiply(a, b)),
        "/" => {
            if b == 0 {
                eprintln!("Ошибка: деление на ноль");
            } else {
                println!("Результат: {}", divide(a, b));
            }
        }
        _ => eprintln!("Ошибка: неизвестная операция"),
    }
}
