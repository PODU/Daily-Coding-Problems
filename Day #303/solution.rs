// Day 303: Angle between clock hands. O(1).
// Bonus: hands overlap (angle 0) every 12/11 hours (~65.45 min apart).
fn clock_angle(h: i32, m: i32) -> i64 {
    let hr = (h % 12) as f64 * 30.0 + m as f64 * 0.5;
    let mn = m as f64 * 6.0;
    let mut diff = (hr - mn).abs();
    diff = diff.min(360.0 - diff);
    diff.round() as i64
}

fn main() {
    println!("{}", clock_angle(3, 30)); // 75
}
