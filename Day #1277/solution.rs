// Day 1277: Curried add_subtract — alternately + then - successive args.
// Currying emulated via chained .next(x); .value reads the running result. O(1)/call.
#[derive(Clone, Copy)]
struct AddSub {
    value: i64,
    idx: i32, // args consumed so far
}

fn add_subtract(x: i64) -> AddSub {
    AddSub { value: x, idx: 1 }
}

impl AddSub {
    fn next(self, x: i64) -> AddSub {
        let nv = if self.idx % 2 == 1 { self.value + x } else { self.value - x };
        AddSub { value: nv, idx: self.idx + 1 }
    }
    fn value(self) -> i64 {
        self.value
    }
}

fn main() {
    println!("{}", add_subtract(7).value());                            // 7
    println!("{}", add_subtract(1).next(2).next(3).value());            // 0
    println!("{}", add_subtract(-5).next(10).next(3).next(9).value());  // 11
}
