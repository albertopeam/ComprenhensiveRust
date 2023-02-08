fn main() {    
    let mut x: i32 = 10;

    // Mutable pointer
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");
    
    // Non mutable pointer
    let ref_x2: & i32 = &x;
    // *ref_x2 = 20; // not compiles as ref_x2 is not mutable
    println!("ref_x2: {ref_x2}");

    // Dangling references
    let ref_x: &i32 = &10;
    {
        let x: i32 = 10; 
        // ref_x = &x; // Compilation error: ^^ borrowed `x` value does not live long enough
    }
    println!("ref_x: {ref_x}");
}
