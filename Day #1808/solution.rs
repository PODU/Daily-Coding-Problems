// Curried add_subtract: each call alternates +/- on the running total.
// Rust has no overloadable call for chaining values, so we use a fluent builder.
// Time: O(1) per call. Space: O(1).
#[derive(Clone, Copy)]
struct AddSub {
    total: i64,
    sign: i64, // sign applied to the NEXT argument
}

impl AddSub {
    fn apply(self, y: i64) -> AddSub {
        AddSub { total: self.total + self.sign * y, sign: -self.sign }
    }
    fn value(self) -> i64 {
        self.total
    }
}

fn add_subtract(x: i64) -> AddSub {
    AddSub { total: x, sign: 1 }
}

fn main() {
    println!("{}", add_subtract(7).value());                          // 7
    println!("{}", add_subtract(1).apply(2).apply(3).value());        // 0
    println!("{}", add_subtract(-5).apply(10).apply(3).apply(9).value()); // 11
}
