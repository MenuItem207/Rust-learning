use std::io::BufRead;

fn main() {
    let n: f64 = std::io::stdin()
        .lock()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");
    println!("{} sgd is {} aud", n, sgd_to_aud(n));
}

// converts sgd to aud
fn sgd_to_aud(sgd: f64) -> f64 {
    return sgd * 1.02;
}
