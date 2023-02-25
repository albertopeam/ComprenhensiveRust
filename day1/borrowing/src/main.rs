#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}

fn borrow() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
}

fn shared_and_unique_borrows() {
    let mut a: i32 = 10;
    let b: &i32 = &a;
    println!("b: {b}"); // added
    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }
    println!("a: {a}");
    // println!("b: {b}"); // removed
}

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn lifetimes_in_function_calls() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p3: &Point = left_most(&p1, &p2);
    println!("left-most point: {:?}", p3);
}

fn main() {
    borrow();
    shared_and_unique_borrows();
    lifetimes_in_function_calls();
}