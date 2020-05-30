use std::io;

fn main() {
    println!("Guess the number");

    println!("Please input yout guess.");

    let mus guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Yout guessed: {}", guess);
}
