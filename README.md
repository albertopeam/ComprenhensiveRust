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

## DAY 1

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