// Day 575: 2D iterator over an array of arrays without flattening/cloning.
// Track (row, col); advance over empty rows. next/has_next amortized O(1).

struct Iterator2D<'a> {
    data: &'a [Vec<i32>],
    row: usize,
    col: usize,
}

impl<'a> Iterator2D<'a> {
    fn new(data: &'a [Vec<i32>]) -> Self {
        let mut it = Iterator2D { data, row: 0, col: 0 };
        it.skip_empty();
        it
    }
    fn skip_empty(&mut self) {
        while self.row < self.data.len() && self.col >= self.data[self.row].len() {
            self.row += 1;
            self.col = 0;
        }
    }
    fn has_next(&self) -> bool {
        self.row < self.data.len()
    }
    fn next(&mut self) -> i32 {
        assert!(self.has_next(), "no more elements");
        let v = self.data[self.row][self.col];
        self.col += 1;
        self.skip_empty();
        v
    }
}

fn main() {
    let data = vec![vec![1, 2], vec![3], vec![], vec![4, 5, 6]];
    let mut it = Iterator2D::new(&data);
    let mut out: Vec<String> = Vec::new();
    while it.has_next() {
        out.push(it.next().to_string());
    }
    println!("{}", out.join(", ")); // 1, 2, 3, 4, 5, 6
}
