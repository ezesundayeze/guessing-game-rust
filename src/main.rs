use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    loop{
        println!("Guess the number");
        println!("Please input your guess.");
    
        let secret = rand::thread_rng().gen_range(1..101);
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
            };
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small, secret: {}", &secret),
            Ordering::Greater => println!("Too big, secret: {}", &secret),
            Ordering::Equal =>{
                println!("You win!");
                break;
            } 
        }
    }
   
}
