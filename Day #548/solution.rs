// Clock hand angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take abs diff, fold >180, round. O(1) time/space.
// Fun fact: the hands overlap (angle 0) 22 times a day, not 24.

fn clock_angle(t: &str) -> i64 {
    let mut it = t.split(':');
    let hh: i64 = it.next().unwrap().parse().unwrap();
    let mm: i64 = it.next().unwrap().parse().unwrap();
    let minute = mm as f64 * 6.0;
    let hour = (hh % 12) as f64 * 30.0 + mm as f64 * 0.5;
    let mut diff = (hour - minute).abs();
    if diff > 180.0 {
        diff = 360.0 - diff;
    }
    diff.round() as i64
}

fn main() {
    println!("{}", clock_angle("12:00"));
    println!("{}", clock_angle("3:30"));
    println!("{}", clock_angle("9:00"));
}
