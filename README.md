# COMPREHENSIVE RUST

## INSTALL 

* macOS install options: 
    * `brew install rustup-init & sudo rustup-init`
    * or `curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh`
* [Doc](https://doc.rust-lang.org/book/ch01-01-installation.html)

## INSTALLED COMPONENTS
* compiler `rustc`
* build, test, deps resolver, etc.. `cargo`

## INTRO
### Welcome
* Rust is a statically compiled language in a similar role as C++

### Hello world

* Create a new package `cargo new hello-world`
* Run package(navigate to hello-world directory) `cargo run`
* Build package `cargo build`
* Check errors `cargo check`
* Build optimized release `cargo build --release`
* Adding dependencies to `Cargo.toml` and running any cargo command it will resolve and compile them automatically

## DAY 1 MORNING

### Small sample
* Collatz conjecture
* [Wiki](https://en.wikipedia.org/wiki/Collatz_conjecture)
```rust
fn main() {              // Program entry point
    let mut x: i32 = 6;  // Mutable variable binding
    print!("{x}");       // Macro for printing, like printf
    while x != 1 {       // No parenthesis around expression
        if x % 2 == 0 {  // Math like in other languages
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}");
    }
    println!();
}
```

### Libs
* [Websites](https://lib.rs/) to find libs

### Basic syntax
* [Table](https://google.github.io/comprehensive-rust/basic-syntax/scalar-types.html)

### Compound types
* Arrays: 
    * Are defined with type and size `let items: [i8; 2] = [1; 2];` in compile time    
    * Printing
        * debug  `{items:?}`
        * default `{items}`
        * pretty `{items:#?}`
* Tuples
    * Have a defined size
    * Can be accesed by the index of the value `let t: (i8, bool) = (7, true);` -> `t.0`
    * Empty tuple `()`, used to indicate no return type or void type

### References
* Mutable reference
```swift
let mut x: i32 = 1;
let ref_x: &mut i32 = &mut x;
*ref_x = 20;
```
* Non mutable reference
```swift
let mut x: i32 = 1;
let ref_x: & i32 = &x;
```

### Slices
* Slices always borrow from another object.
Being 
```rust 
let a: [i32; 6] = [1,2,3,4,5,6];
```
* Exlusive access
```rust
&a[2..4]
```
* Inclusive access
```rust
&a[2..=4]
```
* Start/End index can be omited, in this case it will work as inclusive

### String vs str
* `&str` an immutable reference to a string slice.
* `String` a mutable string buffer

### Functions
* Param's type declared after name, `fn fizzbuzz(n: u32) -> ()`
* Return type can be omitted if it is the unit type `()`, `fn fizzbuzz(n: u32)`
* Last instruction in a function body is the return, it can be omitted 
```rust
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression in a block is the return value
}
```

### Methods
* Functions that are associated with a particular type. The first argument of a method is an instance of the type it is associated with.
* They use the keyword `impl` to extend the type's behaviour.
```rust
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}
```

### Function Overloading
* Not supported
* Each function has only one implementation: fixed parameters and types.
* Default values not supported
* Generics supported
```rust
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}
```

### Exercise Implicit Conversions
Rust will not automatically apply implicit conversions between types (unlike C++). You can see this in a program like this:
```rust
fn multiply(x: i16, y: i16) -> i16 {
    x * y
}
fn main() {
    let x: i8 = 15;
    let y: i16 = 1000;
    println!("{x} * {y} = {}", multiply(x.into(), y));
}
```

The Rust integer types all implement the From<T> and Into<T> traits to let us convert between them. The From<T> trait has a single from() method and similarly, the Into<T> trait has a single into() method. Implementing these traits is how a type expresses that it can be converted into another type.

The standard library has an implementation of From<i8> for i16, which means that we can convert a variable x of type i8 to an i16 by calling i16::from(x). Or, simpler, with x.into(), because From<i8> for i16 implementation automatically create an implementation of Into<i16> for i8.

The same applies for your own From implementations for your own types, so it is sufficient to only implement From to get a respective Into implementation automatically.

Execute the above program and look at the compiler error.

Update the code above to use into() to do the conversion.

Change the types of x and y to other things (such as f32, bool, i128) to see which types you can convert to which other types. Try converting small types to big types and the other way around. Check the standard library documentation to see if From<T> is implemented for the pairs you check.
[DOC](https://doc.rust-lang.org/std/convert/trait.From.html)

### Exercise Arrays and For loops
```rust
fn main() {
    let array = [10, 20, 30];
    print!("Iterating over array:");
    for n in array {
        print!(" {n}");
    }
    println!();

    print!("Iterating over range:");
    for i in 0..3 {
        print!(" {}", array[i]);
    }
    println!();
}
```
Use the above to write a function pretty_print which pretty-print a matrix and a function transpose which will transpose a matrix (turn rows into columns):
Hard-code both functions to operate on 3 × 3 matrices.

### Variables type inference
* Static types
* Type inference
```rust
fn takes_u32(x: u32) {
    println!("u32: {x}");
}
fn main() {
    let x = 10;

    takes_u32(x);
}
```
* Inmutable variable bindings(can't change the type once initialized)

### Static varibales
