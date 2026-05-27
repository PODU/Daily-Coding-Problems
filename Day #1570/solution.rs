// Look-and-say: build each term by run-length encoding the previous. Time O(total digits), space O(len).
fn look_and_say(n: usize) -> String {
    let mut cur = String::from("1");
    for _ in 1..n {
        let bytes = cur.as_bytes();
        let len = bytes.len();
        let mut next = String::new();
        let mut i = 0;
        while i < len {
            let mut j = i;
            while j < len && bytes[j] == bytes[i] {
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
    println!("{}", look_and_say(4)); // 1, 11, 21, 1211
}
