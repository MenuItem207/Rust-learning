fn main() {
    println!("Enter sentence");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut vowels_num = 0;
    let mut consonants_num = 0;
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let consonants = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];
    for c in input.chars() {
        if vowels.contains(&c) {
            vowels_num += 1;
        } else if consonants.contains(&c) {
            consonants_num += 1;
        } else {
            println!("Invalid character");
            return;
        }
    }
    println!("Vowels: {}", vowels_num);
    println!("Consonants: {}", consonants_num);
}
