// Look-and-say sequence: start "1"; each term describes digit runs of previous.
// Iteratively build N terms by run-length encoding. Time O(N * L), Space O(L).

fn look_and_say(n: usize) -> String {
    let mut cur = vec!['1'];
    for _ in 1..n {
        let mut next: Vec<char> = Vec::new();
        let mut j = 0;
        while j < cur.len() {
            let mut count = 1;
            while j + count < cur.len() && cur[j + count] == cur[j] {
                count += 1;
            }
            for c in count.to_string().chars() {
                next.push(c);
            }
            next.push(cur[j]);
            j += count;
        }
        cur = next;
    }
    cur.into_iter().collect()
}

fn main() {
    let n = 5; // terms: 1, 11, 21, 1211, 111221
    println!("{}", look_and_say(n));
}
