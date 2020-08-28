extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your number !");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Input number");
        let guess: u32 = guess.trim().parse().expect("It's not a number!!");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!!"),
            Ordering::Less => println!("Too small!!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
