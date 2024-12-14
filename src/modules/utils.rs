use std::any::type_name;

pub fn type_of<T>(_ : &T) -> &'static str {
    type_name::<T>()
}

pub fn print1(s: String) {
    println!("{}", s);
}

pub fn print2(s: &String) {
    println!("{}", s);
}

pub fn print3(s: &str) {
    println!("{}", s);
}

