use std::{fs, io};

fn main() {
    //challenge 1
    // print("hello rustaceans".to_string());

    // challenge 2
    // mutable();

    //challenge 3
    // read_value();

    //challenge 4
    // read_value_without_trim();

    // throw_exception();

    exist_file();
}

#[allow(dead_code)]
fn print(value: String) {
    println!("{}", value);
}

#[allow(dead_code)]
fn mutable() {
    let mut x = 10; // variable mutable
    let y = &mut x; // crear referencia mutable a x (solo PUEDE HABER UNA)
    *y += 5; // modificar x a traves de la referencia mutable
    println!("x: {}", x); // salida de x
}

#[allow(dead_code)]
fn read_value() {
    let mut input = String::new(); // crear una variable mutable
    io::stdin()
        .read_line(&mut input) // add \n asi que hay que quitarla con trim
        .expect("Failed to read input");

    let trimmed_input = input.trim();
    print(trimmed_input.to_string());
}

#[allow(dead_code)]
fn read_value_without_trim() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    println!("{}", input);
}

#[allow(dead_code)]
fn throw_exception() {
    let simulated_error: Result<(), io::Error> =
        Err(io::Error::new(io::ErrorKind::NotFound, "Simulated error"));
    simulated_error.expect("This is the fail message");
}

#[allow(dead_code)]
fn exist_file() {
    // modo 1
    let mut result = fs::read_to_string("non_existent.txt");

    match result {
        Ok(content) => println!("File content: {}", content),
        Err(error) => eprintln!("Failed to read the file: {}", error),
    }

    // modo 2
    result = fs::read_to_string("non_existent.txt");

    if let Ok(content) = result {
        println!("File content: {}", content);
    } else {
        println!("Failed to read file 2");
    }

    // modo 3
    result = fs::read_to_string("non_existent.txt");
    result.unwrap(); // default message

    // modo 4
    match propagates_error() {
        Ok(content) => println!("File content: {}", content),
        Err(error) => eprintln!("Error reading file: {}", error),
    }

    // modo 5
    let result =
        fs::read_to_string("non_existent_file.txt").map_err(|e| format!("Custom error: {}", e));

    match result {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("{}", error),
    }

    // modo 6
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => eprintln!("Error: {}", error),
    }

    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => eprintln!("Error: {}", error),
    }
}

fn propagates_error() -> Result<String, io::Error> {
    let content = fs::read_to_string("non_existent4.txt")?;
    Ok(content)
}

fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}
