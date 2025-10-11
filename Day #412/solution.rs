// Day 412: Nth term of the look-and-say sequence via run-length encoding.
// Build each term from the previous by counting consecutive runs. O(N * L) time where L = term length, O(L) space.
fn look_and_say(n: usize) -> String {
    let mut cur = String::from("1");
    for _ in 1..n {
        let bytes = cur.as_bytes();
        let m = bytes.len();
        let mut next = String::new();
        let mut i = 0;
        while i < m {
            let mut j = i;
            while j < m && bytes[j] == bytes[i] {
                j += 1;
            }
            next.push_str(&(j - i).to_string());
            next.push(bytes[i] as char);
            i = j;
        }
        cur = next;
    }
    cur
}

fn main() {
    let n = 4;
    println!("{}", look_and_say(n));
}
