// Day 440: Demonstrates the three kinds of polymorphism.
// ad-hoc = trait impls, parametric = generics, subtype = trait objects (dynamic dispatch).
use std::f64::consts::PI;

// Ad-hoc polymorphism: the Add trait gives `+` type-specific meaning.
fn add_ints(a: i64, b: i64) -> i64 { a + b }
fn add_strs(a: &str, b: &str) -> String { format!("{}{}", a, b) }

// Parametric polymorphism: one generic fn over any ordered type.
fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if a >= b { a } else { b }
}

// Subtype polymorphism: trait objects dispatch dynamically.
trait Shape { fn area(&self) -> f64; }
struct Circle { r: f64 }
struct Square { s: f64 }
impl Shape for Circle { fn area(&self) -> f64 { PI * self.r * self.r } }
impl Shape for Square { fn area(&self) -> f64 { self.s * self.s } }

fn main() {
    println!("Ad-hoc polymorphism (overloading): same name, type-specific impls.");
    println!("  add(2,3) = {}, add(\"a\",\"b\") = {}", add_ints(2, 3), add_strs("a", "b"));

    println!("Parametric polymorphism (generics): one impl, any type.");
    println!("  max(3,7) = {}, max(\"ab\",\"zz\") = {}", max_of(3, 7), max_of("ab", "zz"));

    println!("Subtype polymorphism (overriding): base ref, derived behavior.");
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { r: 1.0 }), Box::new(Square { s: 2.0 })];
    for s in &shapes {
        println!("  area = {:.5}", s.area());
    }
}
