// Demonstrates the three polymorphism types: ad-hoc (trait-based overloading), parametric (generics), subtype (trait objects).
use std::f64::consts::PI;

// Ad-hoc polymorphism: a trait gives `add` distinct behavior per type.
trait Add {
    type Out;
    fn add(self, other: Self) -> Self::Out;
}
impl Add for i32 {
    type Out = i32;
    fn add(self, other: i32) -> i32 { self + other }
}
impl Add for &str {
    type Out = String;
    fn add(self, other: &str) -> String { format!("{}{}", self, other) }
}

// Parametric polymorphism: generic over any ordered type.
fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Subtype polymorphism: trait objects with dynamic dispatch.
trait Shape {
    fn area(&self) -> f64;
}
struct Circle { r: f64 }
struct Square { s: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.r * self.r }
}
impl Shape for Square {
    fn area(&self) -> f64 { self.s * self.s }
}

fn main() {
    println!("Ad-hoc polymorphism: add(2,3)={}, add(\"a\",\"b\")={}", 2.add(3), "a".add("b"));
    println!("Parametric polymorphism: max(3,7)={}, max(2.5,1.5)={}", max_of(3, 7), max_of(2.5, 1.5));
    let c: Box<dyn Shape> = Box::new(Circle { r: 1.0 });
    let sq: Box<dyn Shape> = Box::new(Square { s: 2.0 });
    println!("Subtype polymorphism: Circle area={:.5}, Square area={:.1}", c.area(), sq.area());
}
