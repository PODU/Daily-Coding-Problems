// 2D iterator with lazy outer/inner pointers (no flatten/clone).
// next() & has_next() amortized O(1), Space O(1) extra.
struct Iterator2D<'a> {
    data: &'a Vec<Vec<i32>>,
    outer: usize,
    inner: usize,
}

impl<'a> Iterator2D<'a> {
    fn new(data: &'a Vec<Vec<i32>>) -> Self {
        Iterator2D { data, outer: 0, inner: 0 }
    }
    fn has_next(&mut self) -> bool {
        while self.outer < self.data.len() && self.inner >= self.data[self.outer].len() {
            self.outer += 1;
            self.inner = 0;
        }
        self.outer < self.data.len()
    }
    fn next(&mut self) -> i32 {
        if !self.has_next() {
            panic!("no more elements");
        }
        let v = self.data[self.outer][self.inner];
        self.inner += 1;
        v
    }
}

fn main() {
    let data = vec![vec![1, 2], vec![3], vec![], vec![4, 5, 6]];
    let mut it = Iterator2D::new(&data);
    let mut out = Vec::new();
    while it.has_next() {
        out.push(it.next().to_string());
    }
    println!("{}", out.join(", "));
}
