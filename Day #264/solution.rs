// Day 264: De Bruijn sequence B(k, n) over a character set.
// Approach: Fredricksen-Kessler-Maiorana algorithm — concatenate Lyndon words
// whose length divides n, generated via Duval-style recursion.
// Time O(k^n) (size of the output), Space O(n).

struct DeBruijn {
    alphabet: Vec<char>,
    n: usize,
    k: usize,
    a: Vec<usize>,
    sequence: String,
}

impl DeBruijn {
    fn new(alphabet: Vec<char>, n: usize) -> Self {
        let k = alphabet.len();
        DeBruijn { alphabet, n, k, a: vec![0; k * n], sequence: String::new() }
    }

    fn db(&mut self, t: usize, p: usize) {
        if t > self.n {
            if self.n % p == 0 {
                for i in 1..=p {
                    self.sequence.push(self.alphabet[self.a[i]]);
                }
            }
        } else {
            self.a[t] = self.a[t - p];
            self.db(t + 1, p);
            for j in (self.a[t - p] + 1)..self.k {
                self.a[t] = j;
                self.db(t + 1, t);
            }
        }
    }

    fn build(&mut self) -> String {
        self.db(1, 1);
        self.sequence.clone()
    }
}

fn main() {
    // C = {0, 1}, k = 3
    let mut d = DeBruijn::new(vec!['0', '1'], 3);
    println!("{}", d.build());
}
