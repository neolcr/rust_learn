mod modules;

fn main() {
    handle_string();
    handle_mut_string();
    handle_rstring();
    handle_mut_rstring();
    handle_str();
    handle_mut_str();
}

#[allow(dead_code)]
fn handle_string() {
    let x = String::from("Hello");  // x owns the string
    let y = x.clone();             // ownwershipt is transferred to y unless I use .clone()
    modules::utils::print1(x);
    modules::utils::print1(y);
}

#[allow(dead_code)]
fn handle_mut_string() {
    let mut x = String::from("Hello");  // x owns the string
    let mut y = x.clone();             // ownwershipt is transferred to y unless I use .clone()
    modules::utils::print1(x);
    modules::utils::print1(y);
    x = String::from("otro x");
    y = String::from("otro y");
    modules::utils::print1(x);
    modules::utils::print1(y);
}

#[allow(dead_code)]
fn handle_rstring() {
    let x = String::from("Hello you"); // x owns the string
    let y = &x.clone();                         // borrow x
    modules::utils::print1(x);
    modules::utils::print2(y);
}

#[allow(dead_code)]
fn handle_mut_rstring() {
    let mut x = String::from("Hello you"); // x owns the string
    let mut y = &x.clone();                         // borrow x
    modules::utils::print1(x);
    modules::utils::print2(y);
    x = String::from("otro x");
    let temp = String::from("otro y");
    y = &temp;
    modules::utils::print1(x);
    modules::utils::print2(y);
}

#[allow(dead_code)]
fn handle_str() {
    let x = "xxx";
    let y = x;
    modules::utils::print3(x);
    modules::utils::print3(y);
}

#[allow(dead_code)]
fn handle_mut_str() {
    let mut x = "xxx";
    let mut y = x;
    modules::utils::print3(x);
    modules::utils::print3(y);
    x = "zzz";
    modules::utils::print3(x);
}


