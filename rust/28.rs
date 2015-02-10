fn main() {
    let mut i = 1;
    let mut sum = 1;
    let mut side_length = 2;
    let mut counter = 0;
    loop {
        if counter == 4 {
            counter = 0;
            side_length += 2;
        }
        i += side_length;
        println!("{}", i);
        sum += i;
        counter += 1;
        if i >= 1002001 {break; } {continue;}
    }
    println!("{}", sum);
}
