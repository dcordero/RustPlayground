
fn main() {
    log_even_numbers_until(100);
}

fn log_even_numbers_until(num: u32) {
    for i in 0..num {
        if is_even(i) {
            println!("{}", i);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
