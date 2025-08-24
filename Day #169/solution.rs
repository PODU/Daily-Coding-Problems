// Bottom-up (iterative) merge sort on a singly linked list. O(n log n) time, O(1) space.
// Uses an arena (Vec) with index links to satisfy O(1) extra space (no per-merge allocation).

struct Arena {
    val: Vec<i32>,
    next: Vec<i32>, // -1 == null
}

impl Arena {
    fn new() -> Self { Arena { val: Vec::new(), next: Vec::new() } }
    fn push(&mut self, v: i32) -> i32 {
        self.val.push(v);
        self.next.push(-1);
        (self.val.len() - 1) as i32
    }
}

fn length(a: &Arena, mut h: i32) -> i32 {
    let mut n = 0;
    while h != -1 { n += 1; h = a.next[h as usize]; }
    n
}

fn split(a: &mut Arena, head: i32, n: i32) -> i32 {
    let mut cur = head;
    let mut i = 1;
    while cur != -1 && i < n { cur = a.next[cur as usize]; i += 1; }
    if cur == -1 { return -1; }
    let rest = a.next[cur as usize];
    a.next[cur as usize] = -1;
    rest
}

fn merge(a: &mut Arena, mut x: i32, mut y: i32) -> (i32, i32) {
    let mut head = -1;
    let mut tail = -1;
    while x != -1 && y != -1 {
        let pick;
        if a.val[x as usize] <= a.val[y as usize] { pick = x; x = a.next[x as usize]; }
        else { pick = y; y = a.next[y as usize]; }
        if head == -1 { head = pick; } else { a.next[tail as usize] = pick; }
        tail = pick;
    }
    let rest = if x != -1 { x } else { y };
    if head == -1 { head = rest; } else { a.next[tail as usize] = rest; }
    if tail == -1 { tail = head; }
    while tail != -1 && a.next[tail as usize] != -1 { tail = a.next[tail as usize]; }
    (head, tail)
}

fn sort_list(a: &mut Arena, head: i32) -> i32 {
    let n = length(a, head);
    let mut start = head;
    let mut size = 1;
    while size < n {
        // dummy via local variables
        let mut new_head = -1;
        let mut new_tail = -1;
        let mut cur = start;
        while cur != -1 {
            let left = cur;
            let right = split(a, left, size);
            cur = split(a, right, size);
            let (m_head, m_tail) = merge(a, left, right);
            if new_head == -1 { new_head = m_head; } else { a.next[new_tail as usize] = m_head; }
            new_tail = m_tail;
        }
        start = new_head;
        size <<= 1;
    }
    start
}

fn main() {
    let vals = [4, 1, -3, 99];
    let mut a = Arena::new();
    let mut head = -1;
    let mut tail = -1;
    for &v in vals.iter() {
        let idx = a.push(v);
        if head == -1 { head = idx; } else { a.next[tail as usize] = idx; }
        tail = idx;
    }
    head = sort_list(&mut a, head);
    let mut parts: Vec<String> = Vec::new();
    let mut p = head;
    while p != -1 {
        parts.push(a.val[p as usize].to_string());
        p = a.next[p as usize];
    }
    println!("{}", parts.join(" -> "));
}
