
struct Rectangle {
    width: u32,
    height: u32
}

// Extend Rectangle with custom methods
impl Rectangle {

    fn is_square(&self) -> bool {
        return self.width == self.height
    }

    fn print_description(&self) {
        print!("Rectangle {}x{}", self.width, self.height);
    }
}

fn main() {
    let my_rectangle = Rectangle { width: 20, height: 10 };

    my_rectangle.print_description();

    if my_rectangle.is_square() {
        println!(" is a square")
    }
    else {
        println!(" is not a square")
    }
}
