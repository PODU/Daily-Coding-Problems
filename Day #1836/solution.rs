// Day 1836: Demonstrate ad-hoc, parametric, and subtype polymorphism.
// Each demo is O(1); Rust uses traits (ad-hoc), generics (parametric), trait objects (subtype).

// Ad-hoc polymorphism: a trait implemented differently per type.
trait Add {
    fn add(self, other: Self) -> Self;
}
impl Add for i32 {
    fn add(self, other: i32) -> i32 { self + other }
}
impl Add for String {
    fn add(self, other: String) -> String { self + &other }
}

// Parametric polymorphism: one generic definition for any type.
fn identity<T>(x: T) -> T { x }

// Subtype polymorphism: a trait object dispatched dynamically at runtime.
trait Shape { fn name(&self) -> &'static str; }
struct Circle;
struct Square;
impl Shape for Circle { fn name(&self) -> &'static str { "circle" } }
impl Shape for Square { fn name(&self) -> &'static str { "square" } }

fn main() {
    println!("Ad-hoc:     add(2,3)={}, add(\"a\",\"b\")={}",
        2i32.add(3), String::from("a").add(String::from("b")));
    println!("Parametric: identity(7)={}, identity(\"hi\")={}", identity(7), identity("hi"));
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle), Box::new(Square)];
    let names: Vec<&str> = shapes.iter().map(|s| s.name()).collect();
    println!("Subtype:    {}", names.join(" "));
}
