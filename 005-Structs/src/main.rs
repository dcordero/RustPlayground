
// Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct
struct Coordinate(u32, u32);


fn main() {
    // Struct
    let mut my_color = Color { red: 255, green: 70, blue: 15};
    my_color.blue = 45;

    println!("{}, {}, {}", my_color.red, my_color.green, my_color.blue);

    // Tuple struct
    let mut my_coordinate = Coordinate(20, 60);
    my_coordinate.1 = 120;

    println!("{}, {}", my_coordinate.0, my_coordinate.1)
}
