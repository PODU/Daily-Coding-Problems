// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
fn clock_angle(hh: i32, mm: i32) -> i32 {
    let minute = mm as f64 * 6.0;
    let hour = (hh % 12) as f64 * 30.0 + mm as f64 * 0.5;
    let diff = (hour - minute).abs();
    let diff = diff.min(360.0 - diff);
    (diff + 0.5).floor() as i32
}

fn main() {
    println!("{}", clock_angle(12, 30));
    println!("{}", clock_angle(3, 15));
}
