fn scalar() {
    // Scalar Datatypes

    // Signed Integer
    let int32: i32 = -2; // Default type for numbers, can be set to i8, i16, i32, i64, i128
    println!("Int32: {}", int32);

    // Unsigned Interger
    let unsigned_int32: u32 = 3; // can be set to u8, u16, u32, u64, u128
    println!("Unsigned Int: {}", unsigned_int32);

    // Floating point value
    let float32: f32 = 0.12323; // Default value, can be set to f32, f64
    println!("Float: {}", float32);

    // Boolean values
    let boolean: bool = true; // true = 1, false = 0
    println!("Boolean: {}", boolean);

    // Character
    let letter: char = ';';
    println!("Character: {}", letter);
    
}

fn compound() {
    // Scalar Datatypes

    // Tuple
    let tuple: (i32, f32, bool, char) = (2, -123.988, false, '\'');
    println!("Tuple: ({}, {}, {}, {})", tuple.0, tuple.1, tuple.2, tuple.3);

    // Array


    // String

}

fn main() {
    scalar();
    compound();
}
