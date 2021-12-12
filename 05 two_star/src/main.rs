use std::io::BufRead;

fn main() {
    let n: i32 = std::io::stdin()
        .lock()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");
    for i in 0..=n {
        print_star(i * 2 + 1);
    }
}

// prints n number of stars
fn print_star(n: i32) {
    let mut star_string = String::new();
    for _ in 0..n {
        star_string.push_str("*");
    }
    println!("{}", star_string);
}
