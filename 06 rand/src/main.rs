use rand::Rng;

fn main() {
    match rand::thread_rng().gen_range(1..=3) {
        1 => one(),
        2 => two(),
        3 => three(),
        _ => println!("Error"),
    }
}

fn one() {
    println!("{}", 1);
}

fn two() {
    println!("{}", 2);
}

fn three() {
    println!("{}", 3);
}
