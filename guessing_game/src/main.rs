use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number : u8 = rand::thread_rng().gen_range(1..=100);

    loop{

        let guess: String = get_guess();
        let guess_num: u8;

        match guess.trim().parse() {
            Ok(num) => guess_num = num,
            Err(e) => {
                println!("Please type an actual number! {e}");
                
                continue;
            },
        };

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("You guessed {}, Too small!", guess_num),
            Ordering::Greater => println!("You guessed {}, Too big!", guess_num),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
}
}


fn get_guess() -> String {
    let mut guess: String = String::new();
    println!("Guess the number brah");
    match io::stdin().read_line(&mut guess){
        Ok(_) => {},
        Err(_) => {
            panic!("Failed to read line");
        },
    }
    guess
}