fn main() {
    // arrays
    let mut array: [i8; 2] = [1; 2];
    array[1] = 5;
    println!("{array:#?}");

    // tuples
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
}
