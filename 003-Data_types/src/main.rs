
// Global constant variables

const PI: f32 = 3.14159;

fn main() {
    println!("{}", PI);

    // Mutable vs Immutable variables

    let immutable_variable = 45;
    println!("The value of the immutable variable is {}", immutable_variable);

    let mut mutable_variable = 50;
    println!("The value of the mutable variable is first {}", mutable_variable);
    mutable_variable = 60;
    println!("and then {}", mutable_variable);

    // Variable with explicit data type

    let _a: i64 = 1; // 64 bits integer 
    let _b: i32 = 1; // 32 bits integer
    let _c: u64 = 23; // Unsigned 64 bits integer
    let _d: f32 = 1.2; // 32 bits float
    let _e: bool = false; // Boolean

    // Tuples
    let my_tuple = (20, "hello", 30, "world");
    println!("The thrid value in my tuple is {}", my_tuple.3);

    let tuple_with_embeded_tuple = ("hello", "there", (1, 2, 3));
    println!("Accessing subtuple {}", (tuple_with_embeded_tuple.2).2);

    let (_, _, extracted_value3, _) = my_tuple;
    println!("Extracted value 3: {}", extracted_value3);

    // References
    let mut variable = "Hi there";
    
    let immutable_reference_to_variable = &variable;
    println!("Using the immutable reference: {}", immutable_reference_to_variable);

    let mutable_reference_to_variable = &mut variable;
    *mutable_reference_to_variable = "New value";
    println!("Using mutable reference: {}", mutable_reference_to_variable);

    // All data types can be found in the documentation:
    //  https://doc.rust-lang.org/book/ch03-02-data-types.html
}
