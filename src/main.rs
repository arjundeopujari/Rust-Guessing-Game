use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number : u32 = rand::thread_rng().gen_range(1..101);

    loop {

        println!("Guess number!");

        let mut guess : String = String::new();

        io::stdin().read_line(&mut guess).expect("Could not read the line.");

        let guess : u32 = guess.trim().parse().expect("Could not convert input into a number.");

        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },

        }

    }
  
}
