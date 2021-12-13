use rand::Rng;
use std::io::BufRead;

fn main() {
    let num: i32 = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number!");
    let n: i32 = std::io::stdin()
        .lock()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");

    if n > num {
        println!("Too high!, n = {}", num);
    } else if n < num {
        println!("Too low!, n = {}", num);
    } else {
        println!("You win!");
    }
}
