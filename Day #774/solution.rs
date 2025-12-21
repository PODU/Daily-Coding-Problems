// Day 774: Implement readN(n) on top of a read7() primitive.
// Buffer leftover chars from read7 between calls. O(n) per readN call.
struct Reader {
    file: Vec<u8>,
    pos: usize,
    buf: Vec<u8>,
}

impl Reader {
    fn new(s: &str) -> Self {
        Reader { file: s.bytes().collect(), pos: 0, buf: Vec::new() }
    }

    fn read7(&mut self) -> Vec<u8> {
        let end = (self.pos + 7).min(self.file.len());
        let s = self.file[self.pos..end].to_vec();
        self.pos = end;
        s
    }

    fn read_n(&mut self, n: usize) -> String {
        let mut out: Vec<u8> = Vec::new();
        while out.len() < n {
            if self.buf.is_empty() {
                self.buf = self.read7();
                if self.buf.is_empty() {
                    break;
                }
            }
            let take = (n - out.len()).min(self.buf.len());
            out.extend_from_slice(&self.buf[..take]);
            self.buf.drain(..take);
        }
        String::from_utf8(out).unwrap()
    }
}

fn main() {
    let mut r = Reader::new("Hello world");
    println!("\"{}\", \"{}\", \"{}\"", r.read_n(7), r.read_n(7), r.read_n(7));
    // "Hello w", "orld", ""
}
