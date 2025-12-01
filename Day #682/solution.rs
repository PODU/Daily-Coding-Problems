// add_subtract: chainable builder. add(x) alternates +/- ; value() reads result.
// O(n) time over number of args, O(1) extra space.

#[derive(Copy, Clone)]
struct AddSub {
    total: i64,
    sign: i64, // applied to next argument
}

fn add_subtract(first: i64) -> AddSub {
    AddSub { total: first, sign: 1 }
}

impl AddSub {
    fn add(self, x: i64) -> AddSub {
        AddSub { total: self.total + self.sign * x, sign: -self.sign }
    }
    fn value(self) -> i64 {
        self.total
    }
}

fn main() {
    println!("{}", add_subtract(7).value());                            // 7
    println!("{}", add_subtract(1).add(2).add(3).value());              // 0
    println!("{}", add_subtract(-5).add(10).add(3).add(9).value());     // 11
}
