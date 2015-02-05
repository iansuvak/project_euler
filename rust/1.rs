use std::iter::AdditiveIterator;
use std::iter;
fn main() {
    let mut sum = 0u32;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("Procedural: Sum of numbers between 1 and 1000 divisible by 3 and 5 is {}", sum);

    let answer = iter::range(1,1000).filter(|n| is_valid((*n))).sum();
    println!("Functional: Sum of numbers between 1 and 1000 divisible by 3 and 5 is {}", answer);
}

fn is_valid(n: u32) -> bool {
    n % 3 == 0 || n % 5 == 0
}
