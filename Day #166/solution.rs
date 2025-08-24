// 2D iterator: track row/col indices, advance() skips empty subarrays. O(1) amortized per next/has_next, O(1) extra space.

struct Iterator2D<'a> {
    data: &'a Vec<Vec<i32>>,
    row: usize,
    col: usize,
}

impl<'a> Iterator2D<'a> {
    fn new(data: &'a Vec<Vec<i32>>) -> Self {
        let mut it = Iterator2D { data, row: 0, col: 0 };
        it.advance();
        it
    }
    fn advance(&mut self) {
        while self.row < self.data.len() && self.col >= self.data[self.row].len() {
            self.row += 1;
            self.col = 0;
        }
    }
    fn has_next(&mut self) -> bool {
        self.advance();
        self.row < self.data.len()
    }
    fn next(&mut self) -> i32 {
        if !self.has_next() {
            panic!("no more elements");
        }
        let v = self.data[self.row][self.col];
        self.col += 1;
        v
    }
}

fn main() {
    let arr: Vec<Vec<i32>> = vec![vec![1, 2], vec![3], vec![], vec![4, 5, 6]];
    let mut it = Iterator2D::new(&arr);
    let mut out: Vec<String> = Vec::new();
    while it.has_next() {
        out.push(it.next().to_string());
    }
    println!("{}", out.join(", "));
}
