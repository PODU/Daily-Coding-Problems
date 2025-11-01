// readN(n) on top of read7(): buffer leftover chars between calls.
// Time O(n) per readN call, Space O(1) extra (small buffer).
struct Reader {
    content: Vec<char>,
    pos: usize,    // read7 pointer
    buf: Vec<char>, // leftover unconsumed chars
}

impl Reader {
    fn new(s: &str) -> Self {
        Reader { content: s.chars().collect(), pos: 0, buf: Vec::new() }
    }

    fn read7(&mut self) -> Vec<char> {
        let end = (self.pos + 7).min(self.content.len());
        let chunk = self.content[self.pos..end].to_vec();
        self.pos = end;
        chunk
    }

    fn read_n(&mut self, n: usize) -> String {
        while self.buf.len() < n {
            let chunk = self.read7();
            if chunk.is_empty() {
                break;
            }
            self.buf.extend(chunk);
        }
        let take = n.min(self.buf.len());
        let out: String = self.buf[..take].iter().collect();
        self.buf.drain(..take);
        out
    }
}

fn main() {
    let mut r = Reader::new("Hello world");
    println!("\"{}\"", r.read_n(7));
    println!("\"{}\"", r.read_n(7));
    println!("\"{}\"", r.read_n(7));
}
