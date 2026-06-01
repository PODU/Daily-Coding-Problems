// readN using read7: buffer leftover chars from read7 between calls; pull until n satisfied or EOF.
// Time O(n) per readN call.
struct Reader {
    file: Vec<char>,
    pos: usize,    // internal pointer for read7
    buf: Vec<char>, // leftover chars buffered between readN calls
}

impl Reader {
    fn new(file: &str) -> Self {
        Reader { file: file.chars().collect(), pos: 0, buf: Vec::new() }
    }

    // read7 primitive: up to 7 chars, advances pointer, "" at EOF
    fn read7(&mut self) -> String {
        let end = (self.pos + 7).min(self.file.len());
        let res: String = self.file[self.pos..end].iter().collect();
        self.pos = end;
        res
    }

    // readN: read exactly n chars (or fewer at EOF), buffering leftovers
    fn read_n(&mut self, n: usize) -> String {
        let mut out: Vec<char> = Vec::new();
        while out.len() < n {
            if self.buf.is_empty() {
                let chunk = self.read7();
                if chunk.is_empty() {
                    break; // EOF
                }
                self.buf = chunk.chars().collect();
            }
            let take = (n - out.len()).min(self.buf.len());
            out.extend(self.buf.drain(0..take));
        }
        out.into_iter().collect()
    }
}

fn main() {
    let mut r1 = Reader::new("Hello world");
    println!("read7: \"{}\"", r1.read7());
    println!("read7: \"{}\"", r1.read7());
    println!("read7: \"{}\"", r1.read7());

    let mut r2 = Reader::new("Hello world");
    println!("readN(5): \"{}\"", r2.read_n(5));
    println!("readN(100): \"{}\"", r2.read_n(100));
    println!("readN(3): \"{}\"", r2.read_n(3));
}
