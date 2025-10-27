// Day 503: Sort a singly linked list using bottom-up (iterative) merge sort.
// Nodes live in an index "arena" (simulated pointers) so the sort itself uses
// O(1) auxiliary space and runs in O(n log n) time.

struct Arena {
    // (value, next index); usize::MAX means null.
    nodes: Vec<(i64, usize)>,
}

const NIL: usize = usize::MAX;

impl Arena {
    fn new() -> Self {
        Arena { nodes: Vec::new() }
    }
    fn push(&mut self, val: i64) -> usize {
        self.nodes.push((val, NIL));
        self.nodes.len() - 1
    }
    fn val(&self, i: usize) -> i64 {
        self.nodes[i].0
    }
    fn next(&self, i: usize) -> usize {
        self.nodes[i].1
    }
    fn set_next(&mut self, i: usize, n: usize) {
        self.nodes[i].1 = n;
    }

    fn length(&self, mut head: usize) -> usize {
        let mut n = 0;
        while head != NIL {
            n += 1;
            head = self.next(head);
        }
        n
    }

    // Split off `size` nodes from head; cut there and return the rest index.
    fn split(&mut self, head: usize, size: usize) -> usize {
        let mut cur = head;
        let mut i = 1;
        while cur != NIL && i < size {
            cur = self.next(cur);
            i += 1;
        }
        if cur == NIL {
            return NIL;
        }
        let rest = self.next(cur);
        self.set_next(cur, NIL);
        rest
    }

    // Merge two sorted lists after `tail`; return new tail index.
    fn merge(&mut self, mut a: usize, mut b: usize, mut tail: usize) -> usize {
        while a != NIL && b != NIL {
            if self.val(a) <= self.val(b) {
                self.set_next(tail, a);
                a = self.next(a);
            } else {
                self.set_next(tail, b);
                b = self.next(b);
            }
            tail = self.next(tail);
        }
        self.set_next(tail, if a != NIL { a } else { b });
        while self.next(tail) != NIL {
            tail = self.next(tail);
        }
        tail
    }

    fn sort(&mut self, head: usize) -> usize {
        if head == NIL || self.next(head) == NIL {
            return head;
        }
        let n = self.length(head);
        let dummy = self.push(0);
        self.set_next(dummy, head);
        let mut size = 1;
        while size < n {
            let mut tail = dummy;
            let mut cur = self.next(dummy);
            while cur != NIL {
                let left = cur;
                let right = self.split(left, size);
                cur = self.split(right, size);
                tail = self.merge(left, right, tail);
            }
            size *= 2;
        }
        self.next(dummy)
    }

    fn to_string(&self, mut head: usize) -> String {
        let mut parts = Vec::new();
        while head != NIL {
            parts.push(self.val(head).to_string());
            head = self.next(head);
        }
        parts.join(" -> ")
    }
}

fn main() {
    let vals = [4, 1, -3, 99];
    let mut arena = Arena::new();
    let mut head = NIL;
    let mut tail = NIL;
    for &v in vals.iter() {
        let idx = arena.push(v);
        if head == NIL {
            head = idx;
        } else {
            arena.set_next(tail, idx);
        }
        tail = idx;
    }
    let sorted = arena.sort(head);
    println!("{}", arena.to_string(sorted)); // -3 -> 1 -> 4 -> 99
}
