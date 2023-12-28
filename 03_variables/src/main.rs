fn main() {
    let x = 100;
    let mut y = 0;
    // x = y;
    println!("Y is mutable, value of y before changing: {}", y);
    y = x;
    println!("X is immutable: {}", x);
    println!("Y is mutable, value of y after changing: {}", y);
}
