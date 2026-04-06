// Look-and-say: build each term by run-length encoding the previous one.
// Time O(sum of term lengths), Space O(length of Nth term).

fn look_and_say(n: usize) -> String {
    let mut s = String::from("1");
    for _ in 1..n {
        let bytes = s.as_bytes();
        let mut next = String::new();
        let mut j = 0;
        while j < bytes.len() {
            let mut k = j;
            while k < bytes.len() && bytes[k] == bytes[j] {
                k += 1;
            }
            next.push_str(&(k - j).to_string());
            next.push(bytes[j] as char);
            j = k;
        }
        s = next;
    }
    s
}

fn main() {
    println!("{}", look_and_say(5)); // 111221
}
