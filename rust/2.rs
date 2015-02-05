extern crate test;

use test::Bencher;

use std::iter::AdditiveIterator;
use std::iter;

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fib() -> Fibonacci {
    Fibonacci { curr: 1, next: 1}
}

fn main() {
    let sum = procedural();
    println!("Procedural: Sum of even fibonacci numbers less than 4M is {}", sum);
    let fold = fib().take_while(|&n| n < 4_000_000).filter(|n| n % 2 == 0).sum();
    println!("Functional: Sum of even fibonacci numbers less than 4M is {}", fold);
}

fn procedural() -> u32 {
    let mut sum = 0u32;
    for i in fib() {
        if i > 4_000_000 {
            break;
        } else if i % 2 == 0 {
            sum += i;
        }
    }
    sum
}

#[bench]
fn bench_procedural(b: &mut Bencher) {
    b.iter(|| {
        let mut sum = 0u32;
        for i in fib() {
            if i > 4_000_000 {
                break;
            } else if i % 2 == 0 {
                sum += i;
            }
        }
        sum
    });
}

#[bench]
fn bench_functional(b: &mut Bencher) {
    b.iter(|| {
        let sum = fib().take_while(|&n| n < 4_000_000).filter(|n| n % 2 == 0).sum();
        sum
    });
}
