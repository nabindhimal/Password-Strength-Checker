use std::io;
use colored::*;


fn main() {

    println!("{}","Enter Your Password::".blue());
    // Take password as user Input
    let mut password = String::new();

    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read lines!");
    
    let length = password.len();
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_numbers = false;
    let mut has_special_chars = false;

    for ch in password.chars() {
        if ch.is_ascii_uppercase() {
            has_uppercase = true;
        } else if ch.is_ascii_lowercase() {
            has_lowercase = true;
        } else if ch.is_ascii_digit() {
            has_numbers = true;
        } else {
            has_special_chars = true;
        }
    }
    
    if length < 8 {
        println!("{}","Short password!".red());
        return;
    }

    if has_special_chars && has_uppercase && has_lowercase && has_numbers {
        println!("{}","Very Strong Password".green());
        } else if has_special_chars && has_lowercase && has_uppercase && has_numbers == false {
            println!("{}","Relatively Strong. Try adding numbers.".blue());
        } else if has_special_chars ==false && has_lowercase && has_uppercase && has_numbers  {
            println!("{}","Relatively Strong. Try adding special characters.".blue());
        } else if has_special_chars && has_lowercase == false && has_uppercase && has_numbers {
            println!("{}","Relatively Strong. Try adding lowercase letters too.".blue());
        } else if  has_special_chars && has_lowercase && has_uppercase == false && has_numbers {
            println!("{}","Relatively Strong. Try adding uppercase letters too.".blue());
        } else     
        {
            println!("{}","Not very Strong.Try adding numbers, letters, uppercase, lowercase, special chars, etc.".red());
        }
    
}
