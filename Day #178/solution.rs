// Monte Carlo simulation of two dice stopping games; average rolls. O(trials) time, O(1) space.
// Theory: E[rolls "5 then 6"]=36, E[rolls "5 then 5"]=42. Exact sim values depend on RNG/seed.
struct Lcg { s: u64 }
impl Lcg {
    fn die(&mut self) -> i32 {
        self.s = self.s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.s >> 33) % 6) as i32 + 1
    }
}

fn trial(r: &mut Lcg, second: i32) -> i64 {
    let mut rolls = 0i64;
    let mut prev = 0;
    loop {
        let c = r.die();
        rolls += 1;
        if prev == 5 && c == second { return rolls; }
        prev = c;
    }
}

fn main() {
    let mut r = Lcg { s: 12345 };
    let t = 100000i64;
    let mut s1 = 0i64;
    let mut s2 = 0i64;
    for _ in 0..t { s1 += trial(&mut r, 6); }
    for _ in 0..t { s2 += trial(&mut r, 5); }
    let e1 = s1 as f64 / t as f64;
    let e2 = s2 as f64 / t as f64;
    println!("Game 1 (five then six) expected rolls: {:.2}", e1);
    println!("Game 2 (five then five) expected rolls: {:.2}", e2);
    if e1 < e2 {
        println!("Alice should play Game 1 (five then six), it has lower expected cost.");
    } else {
        println!("Alice should play Game 2 (five then five), it has lower expected cost.");
    }
}
