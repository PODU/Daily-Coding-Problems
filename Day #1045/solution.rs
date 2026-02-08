// Demonstrates three kinds of polymorphism: ad-hoc (trait-based overloading),
// parametric (generics), subtype (trait objects / dynamic dispatch). O(1) demo.

// Ad-hoc: overload `add` for different types via a trait (Rust's overloading model).
trait Add2 {
    type Out;
    fn add2(self, other: Self) -> Self::Out;
}
impl Add2 for i32 {
    type Out = i32;
    fn add2(self, other: i32) -> i32 { self + other }
}
impl Add2 for &str {
    type Out = String;
    fn add2(self, other: &str) -> String { format!("{}{}", self, other) }
}

// Parametric: generic function works for any type that is Copy.
fn first<T: Copy>(v: &[T]) -> T { v[0] }

// Subtype: trait with method overridden by implementors, called via trait object.
trait Animal {
    fn speak(&self) -> &'static str;
}
struct Dog;
struct Cat;
impl Animal for Dog { fn speak(&self) -> &'static str { "Woof" } }
impl Animal for Cat { fn speak(&self) -> &'static str { "Meow" } }

fn main() {
    println!("Ad-hoc polymorphism: same name, different argument types (overloading).");
    println!("Ad-hoc: add(2,3)={}, add(\"a\",\"b\")={}", 2.add2(3), "a".add2("b"));

    println!("Parametric polymorphism: one generic definition works for any type.");
    println!("Parametric: first([1,2,3])={}, first([\"x\",\"y\"])={}",
        first(&[1, 2, 3]), first(&["x", "y"]));

    println!("Subtype polymorphism: a base reference dispatches to the subclass override.");
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    println!("Subtype: Dog says {}, Cat says {}", animals[0].speak(), animals[1].speak());
}
