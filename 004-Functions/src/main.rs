
fn main() {
    log_even_numbers_until(100);
}

fn log_even_numbers_until(num: u32) {
    for i in 0..num {
        if is_even(i) {
            print_number(&i);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}

// Example of a function receiving parameter by reference
fn print_number(num: &u32) {
    println!("Value: {}", num);
}