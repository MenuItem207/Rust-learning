use std::io::BufRead;

fn main() {
    print!("Enter a number: ");
    let n: u64 = std::io::stdin()
        .lock()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");
    println!("The factorial of {} is {}", n, factorial(n));
}

// finds the factorial of a number
fn factorial(mut n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut o: u64 = n;
    loop {
        if n == 1 {
            break;
        }
        n = n - 1;
        o = o * n;
    }
    return o;
}
