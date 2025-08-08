// Day 82: readN(n) built on read7() by buffering leftover characters between calls.
// Time O(n) per call amortized, Space O(7) buffer.
struct Reader {
    file: Vec<char>,
    pos: usize,
    buffer: Vec<char>,
}

impl Reader {
    fn new(file: &str) -> Self {
        Reader { file: file.chars().collect(), pos: 0, buffer: Vec::new() }
    }

    // Returns up to 7 characters from the file, advancing the cursor.
    fn read7(&mut self) -> Vec<char> {
        let end = (self.pos + 7).min(self.file.len());
        let chunk = self.file[self.pos..end].to_vec();
        self.pos = end;
        chunk
    }

    fn read_n(&mut self, n: usize) -> String {
        let mut result: Vec<char> = Vec::new();
        while result.len() < n {
            if self.buffer.is_empty() {
                let chunk = self.read7();
                if chunk.is_empty() {
                    break; // EOF
                }
                self.buffer = chunk;
            }
            let take = self.buffer.len().min(n - result.len());
            result.extend(self.buffer.drain(0..take));
        }
        result.into_iter().collect()
    }
}

fn main() {
    let mut r = Reader::new("Hello world");
    println!("{:?}", r.read_n(7)); // "Hello w"
    println!("{:?}", r.read_n(7)); // "orld"
    println!("{:?}", r.read_n(7)); // ""
}
