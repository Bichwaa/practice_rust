use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut guess: String = String::new();

    let secret_number : u8 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);
    println!("Guess Your number brah");

    loop{
    match io::stdin().read_line(&mut guess){
        Ok(_) => {},
        Err(_) => {
            println!("Failed to read line");
            break;
        },
    }

    let guess: u8 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please type an actual number!");
            continue;
        },
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
}
