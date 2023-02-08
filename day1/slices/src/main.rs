fn main() {
    let a: [i32; 6] = [1,2,3,4,5,6];
    println!("a {a:#?}");

    // slice
    let s: &[i32] = &a[2..=4]; // `..=` end inclusive range, `..` end exclusive range
    println!("s {s:#?}");
}
