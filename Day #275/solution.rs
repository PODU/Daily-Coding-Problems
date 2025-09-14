// Day 275: Nth term of look-and-say (term 1 = "1").
// Build each term by run-length encoding the previous. Time O(N * len), Space O(len).
fn look_and_say(n: usize) -> String {
    let mut cur = String::from("1");
    for _ in 1..n {
        let bytes = cur.as_bytes();
        let mut nxt = String::new();
        let mut i = 0;
        while i < bytes.len() {
            let mut j = i;
            while j < bytes.len() && bytes[j] == bytes[i] {
                j += 1;
            }
            nxt.push_str(&(j - i).to_string());
            nxt.push(bytes[i] as char);
            i = j;
        }
        cur = nxt;
    }
    cur
}

fn main() {
    println!("{}", look_and_say(5)); // 111221
}
