#[derive(Clone, Debug)]
struct Point(i32, i32, String); // String does not implement Copy trait.

fn main() {
    let p1 = Point(3, 4, String::from("A Coruña"));
    let p2 = p1.clone(); // Needed to clone otherwise it won't compile because of the print p1, p2 borrows p1(p2 = p1) so p1 must be cloned when is assigned to p2. 
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}