// Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
// Bonus: hands overlap (angle 0) 11 times per 12 hours, at t = (12/11)*k hours for k=0..10.

fn clock_angle(hh: i32, mm: i32) -> i64 {
    let minute_angle = mm as f64 * 6.0;
    let hour_angle = (hh % 12) as f64 * 30.0 + mm as f64 * 0.5;
    let diff = (hour_angle - minute_angle).abs();
    let angle = diff.min(360.0 - diff);
    angle.round() as i64
}

fn main() {
    let t = "3:15";
    let parts: Vec<&str> = t.split(':').collect();
    let hh: i32 = parts[0].trim().parse().unwrap();
    let mm: i32 = parts[1].trim().parse().unwrap();
    println!("{}", clock_angle(hh, mm));
}
