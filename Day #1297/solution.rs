// Day 1297: Implement readN(n) on top of a read7() primitive.
// Keep a leftover buffer of unused chars; refill via read7 until n chars (or EOF). O(n) amortized.
struct Reader {
    file: Vec<u8>,
    pos: usize,
}

impl Reader {
    fn new(content: &str) -> Self {
        Reader { file: content.as_bytes().to_vec(), pos: 0 }
    }
    fn read7(&mut self) -> String { // up to 7 chars, "" at EOF
        let end = (self.pos + 7).min(self.file.len());
        let s = String::from_utf8(self.file[self.pos..end].to_vec()).unwrap();
        self.pos = end;
        s
    }
}

struct NReader {
    r: Reader,
    buf: String,
}

impl NReader {
    fn new(r: Reader) -> Self {
        NReader { r, buf: String::new() }
    }
    fn read_n(&mut self, n: usize) -> String {
        while self.buf.len() < n {
            let chunk = self.r.read7();
            if chunk.is_empty() {
                break;
            }
            self.buf.push_str(&chunk);
        }
        let take = n.min(self.buf.len());
        let out: String = self.buf[..take].to_string();
        self.buf = self.buf[take..].to_string();
        out
    }
}

fn main() {
    let mut nr = NReader::new(Reader::new("Hello world"));
    println!("'{}'", nr.read_n(5));  // 'Hello'
    println!("'{}'", nr.read_n(4));  // ' wor'
    println!("'{}'", nr.read_n(10)); // 'ld'
}
