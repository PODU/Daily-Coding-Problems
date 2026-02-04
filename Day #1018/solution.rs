// Clock angle: O(1) time, O(1) space.
// minute=mm*6, hour=(hh%12)*30+mm*0.5, diff=|h-m|, angle=min(diff,360-diff), rounded.
fn clock_angle(hh: i32, mm: i32) -> i32 {
    let minute = mm as f64 * 6.0;
    let hour = (hh % 12) as f64 * 30.0 + mm as f64 * 0.5;
    let diff = (hour - minute).abs();
    let angle = diff.min(360.0 - diff);
    angle.round() as i32
}

fn main() {
    println!("{:02}:{:02} -> {}", 3, 30, clock_angle(3, 30));
    println!("{:02}:{:02} -> {}", 12, 0, clock_angle(12, 0));
}
