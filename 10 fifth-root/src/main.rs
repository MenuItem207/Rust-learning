/// calculates num to the power of n
macro_rules! nth {
    ($n:expr, $num:expr) => {{
        let mut result = $num;
        for _ in 1..$n {
            result *= $num;
        }
        result
    }};
}

fn main() {
    let (mut current_num, mut sum, mut odd_num) = (1, 0, 0);
    let sum_of_odd_squares: i32 = loop {
        current_num += 2;
        odd_num = odd_num + 1;
        if odd_num >= 100 {
            break sum + 1;
        } else {
            sum += current_num * current_num;
        }
    };
    println!(
        "The sum of the first 100 odd squares is: {}",
        sum_of_odd_squares
    );
    /*
    // to find closest int, iterate through all int to find the smallest int that is greater than sum_of_odd_squares
    // closest int will be the one that is smaller
    let mut current_num = 1;
    let closest_int_root: i32 = loop {
        let result_fifth = nth!(5, current_num);
        if result_fifth > sum_of_odd_squares {
            break current_num - 1;
        } else if result_fifth == sum_of_odd_squares {
            break current_num;
        } else {
            current_num += 1;
        }
    };
    println!(
        "The closest int to the square root of the sum of the first 100 odd squares is: {}",
        closest_int_root
    );

    */
    println!(
        "the fifth root of the sum of squares of the first 100 odd numbers is: {}",
        nth_root(5, sum_of_odd_squares as f32)
    );
}

// calculates the nth root of a number num
fn nth_root(n: i32, num: f32) -> f32 {
    const MAX_DECIMAL_POINTS: i32 = 8;
    let mut current_point: i32 = 1; // the current decimal point 10^current_point
    let mut current_num: f32 = 1.0;
    // the current closest root
    let mut current_root: f32 = loop {
        let result_fifth = nth!(n, current_num);
        if result_fifth > num {
            break current_num - 1.0;
        } else if result_fifth == num {
            break current_num;
        } else {
            current_num += 1.0;
        }
    };

    // iterate through all decimal points to MAX_DECIMAL_POINTS
    loop {
        if current_root == num as f32 || current_point > MAX_DECIMAL_POINTS {
            // if result found, return current_root
            break current_root;
        } else {
            // iterate from 0 to 9 to find closest int at decimal point 10^current_point
            println!("current_point: {}", current_point);
            let current_divider: f32 = nth!(current_point, 10) as f32;
            println!("current_multiplier: {}", current_divider);
            for i in 0..10 {
                let sum: f32 = current_root + (i as f32 / current_divider);
                let result_nth: f32 = nth!(n, sum);
                if result_nth > num {
                    current_root = current_root + ((i - 1) as f32 / current_divider);
                    break;
                } else if result_nth == num {
                    current_root = sum;
                    break;
                }
            }
            println!("nothing");
            current_point += 1;
        }
    }
}
