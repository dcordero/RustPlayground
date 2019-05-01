fn main() {
    let mut numbers = [1, 2, 3, 4, 5];

    numbers[3] = 8;

    println!("Transverse with iterator");
    for value in numbers.iter() {
        println!("Value: {}", value);
    }

    println!("\nTransverse by index");
    for index in 0..numbers.len() {
        println!("Value: {}", numbers[index]);
    }

    // Indicate data type and length
    let _my_array: [u32; 3] = [3, 2, 1];

    // With a default value
    let _my_array_with_default_value_of_2 = [2; 400];
}
