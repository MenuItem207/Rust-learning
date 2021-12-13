use std::io::BufRead;

fn main() {
    println!("Enter a number: ");
    let n: i32 = std::io::stdin()
        .lock()
        .lines()
        .next()
        .expect("stdin should be available")
        .expect("couldn't read from stdin")
        .trim()
        .parse()
        .expect("input was not an integer");
    println!("{:?}", fibs_nums(n));
}

fn fibs_nums(n: i32) -> Vec<i32> {
    let mut nums: Vec<i32> = vec![0]; // vector of fibonacci numbers
    while nums[nums.len() - 1] <= n {
        if nums.len() == 1 {
            nums.push(1);
        } else {
            let last_index: usize = nums.len() - 1;
            let last_last_index: usize = nums.len() - 2;
            nums.push(nums[last_index] + nums[last_last_index]);
        }
    }
    nums.pop();

    nums
}
