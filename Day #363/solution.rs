// Day 363: Curried add_subtract that alternately adds/subtracts arguments.
// A builder whose `add` returns the next link carrying the running value.
// Time O(k) per chain of k args, Space O(1).

#[derive(Clone, Copy)]
struct AddSubtract {
    value: i64,
    count: i64,
}

fn add_subtract(first: i64) -> AddSubtract {
    AddSubtract { value: first, count: 1 }
}

impl AddSubtract {
    fn add(self, x: i64) -> AddSubtract {
        let delta = if self.count % 2 == 1 { x } else { -x }; // arg1 adds, arg2 subtracts
        AddSubtract { value: self.value + delta, count: self.count + 1 }
    }
    fn val(self) -> i64 {
        self.value
    }
}

fn main() {
    println!("{}", add_subtract(7).val());
    println!("1 + 2 - 3 -> {}", add_subtract(1).add(2).add(3).val());
    println!("-5 + 10 - 3 + 9 -> {}", add_subtract(-5).add(10).add(3).add(9).val());
}
