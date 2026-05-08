// Demonstrates ad-hoc (trait-based overloading), parametric (generics),
// and subtype (trait-object dynamic dispatch) polymorphism. Runs in main.

// Ad-hoc polymorphism: one name `add`, behavior per type via a trait.
trait Add {
    fn add(self, other: Self) -> Self;
}
impl Add for i32 {
    fn add(self, other: i32) -> i32 { self + other }
}
impl Add for String {
    fn add(self, other: String) -> String { self + &other }
}

// Parametric polymorphism: one definition works for any type T.
fn identity<T>(x: T) -> T { x }

// Subtype polymorphism: trait object dispatches to concrete impl.
trait Animal {
    fn speak(&self) -> String;
}
struct Dog;
struct Cat;
impl Animal for Dog {
    fn speak(&self) -> String { "Woof".to_string() }
}
impl Animal for Cat {
    fn speak(&self) -> String { "Meow".to_string() }
}

fn main() {
    println!("Ad-hoc (overloading): same name, chosen by argument types.");
    println!("  add(2, 3) = {}", 2i32.add(3));
    println!("  add(\"foo\", \"bar\") = {}", String::from("foo").add(String::from("bar")));

    println!("Parametric (generics): one definition, any type.");
    println!("  identity(42) = {}", identity(42));
    println!("  identity(\"hi\") = {}", identity("hi"));

    println!("Subtype (dynamic dispatch): base ref calls overridden method.");
    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for a in &zoo {
        println!("  {}", a.speak());
    }
}
