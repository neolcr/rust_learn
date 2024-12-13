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

    // exist_file();

    // calculator();

    string_to_str(String::from("hola"));

    let s: String = str_to_string("hola2");
    println!("{}", s);
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

#[allow(dead_code)]
fn propagates_error() -> Result<String, io::Error> {
    let content = fs::read_to_string("non_existent4.txt")?;
    Ok(content)
}

#[allow(dead_code)]
fn divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(a / b)
    }
}

#[allow(dead_code)]
fn calculator() {
    let mut op = String::new();
    let mut v1 = String::new();
    let mut v2 = String::new();

    io::stdin()
        .read_line(&mut op)
        .expect("Failed input operator");
    println!("Operator {}", op);

    io::stdin()
        .read_line(&mut v1)
        .expect("Failed input value 1");
    println!("Value 1: {}", v1);

    io::stdin()
        .read_line(&mut v2)
        .expect("Failed input value 2");
    println!("Value 2: {}", v2);

    let n1: i32 = v1.trim().parse().unwrap();
    let n2: i32 = v2.trim().parse().unwrap();
    let result = n1 + n2;

    println!("Resultado: {}", result);
}

fn string_to_str(s: String) {
    let s_ref: &str = &s;
    // can't return a &str due to it is not owned but borrowed'
    println!("{}", s_ref);
}

fn str_to_string(s: &str) -> String {
    let s = String::from(s);
    s
}
