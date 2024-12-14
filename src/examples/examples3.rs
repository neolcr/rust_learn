mod modules;

fn main() {
    // Step 1: Create a String
    let owned_string = String::from("I am a normal string"); // Ownership of a heap-allocated string
    println!("Owned String: {}", owned_string);

    // Step 2: Borrow the String as &String
    let borrowed_string: &String = &owned_string; // Borrow the String
    println!("Borrowed &String: {}", borrowed_string);

    // Step 3: Convert &String to &str
    let borrowed_str: &str = borrowed_string.as_str(); // Deference &String to &str
    println!("Borrowed &str: {}", borrowed_str);

    // Step 4: Pass the &str to a function that takes it as a borrowed string slice
    print_borrowed_str(borrowed_str);

    // Step 5: Convert &str back to a String
    let new_owned_string = borrowed_str.to_string(); // create a new string from &str
    println!("Convert back to owned string: {}", new_owned_string);

    // Step 6: Transfer ownership to a function
    take_ownership(new_owned_string);

    // Step 7: Try recreating a String from a literal (static &str)
    let static_str: &'static str = "Static string slice"; // &'static str
    println!("Static &str: {}", static_str);

    let recreated_string = static_str.to_string(); // convert &'static str to String
    println!("Recreated String from String &str: {}", recreated_string);

    // Step 8: Pass ownership of recreated String
    let final_string = recreated_string.clone(); // Clone it to keep using it
    take_ownership(recreated_string); // Ownership is moved here

    // Step 9: Borrow again for immutability
    let borrowed_final: &String = &final_string; // Borrow again
    println!("Borrowed Again as &String: {}", borrowed_final);

    // Step 10: Borrow as &str again and print
    let final_borrowed_str: &str = borrowed_final.as_str();
    println!("Final Borrowed &str: {}", final_borrowed_str);

    // Step 11: Ownership and borrowing chain complete
    println!("Original String still exists: {}", final_string);
}

fn print_borrowed_str(input: &str) {
    println!("Borrowed &str in function: {}", input);
}

fn take_ownership(input: String) {
    println!("Took ownership of String: {}", input);
}
