fn main() {
    move_semantics();
    moves_in_function_calls();
}

fn move_semantics() {
    let s1: String = String::from("Hello!");
    let s2: String = s1; // s1.clone() will make print to work
    println!("s2: {s2}");
    // println!("s1: {s1}");
}

fn say_hello(name: String) {
    println!("Hello {name}")
}

fn moves_in_function_calls() {
    let name = String::from("Alice");
    say_hello(name);
    //say_hello(name); // using name.clone() in first say hello will make it work. another approach is to change say_hello(name: &String) and invoke it using say_hello(&name)
}
