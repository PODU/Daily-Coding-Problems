// Day 1752: Remove kth-from-last node of a singly linked list in ONE pass, O(1) space.
// Two pointers spaced k apart; when fast reaches end, slow is just before the target. O(n) time.
// Assumption (no README example): list 1->2->3->4->5, k=2 removes value 4 -> "1 2 3 5".
// Uses index-based pointers over a Vec arena to model a singly linked list safely.

struct Node {
    val: i32,
    next: Option<usize>,
}

struct List {
    nodes: Vec<Node>,
    head: Option<usize>,
}

impl List {
    fn from(values: &[i32]) -> Self {
        let mut nodes = Vec::new();
        let mut head = None;
        let mut prev: Option<usize> = None;
        for &v in values {
            let idx = nodes.len();
            nodes.push(Node { val: v, next: None });
            match prev {
                Some(p) => nodes[p].next = Some(idx),
                None => head = Some(idx),
            }
            prev = Some(idx);
        }
        List { nodes, head }
    }

    // Single pass with two index pointers spaced k apart over a dummy-prefixed view.
    fn remove_kth_last(&mut self, k: usize) {
        // `None` represents the dummy node sitting before head.
        let mut fast: Option<usize> = self.head; // dummy.next == head; advance k-1 more below
        // We model dummy via Option chaining: start fast at head, then move k-1 to keep gap k from dummy.
        // Simpler: track positions explicitly.
        // fast starts at dummy(None). Move k steps.
        fast = None;
        let mut steps = 0;
        while steps < k {
            fast = match fast {
                None => self.head,
                Some(i) => self.nodes[i].next,
            };
            steps += 1;
        }
        let mut slow: Option<usize> = None; // dummy
        while let Some(f) = fast {
            if self.nodes[f].next.is_none() {
                break;
            }
            fast = self.nodes[f].next;
            slow = match slow {
                None => self.head,
                Some(i) => self.nodes[i].next,
            };
        }
        // unlink slow.next
        let target = match slow {
            None => self.head,
            Some(i) => self.nodes[i].next,
        };
        if let Some(t) = target {
            let after = self.nodes[t].next;
            match slow {
                None => self.head = after,
                Some(i) => self.nodes[i].next = after,
            }
        }
    }

    fn to_string(&self) -> String {
        let mut parts = Vec::new();
        let mut cur = self.head;
        while let Some(i) = cur {
            parts.push(self.nodes[i].val.to_string());
            cur = self.nodes[i].next;
        }
        parts.join(" ")
    }
}

fn main() {
    let mut list = List::from(&[1, 2, 3, 4, 5]);
    list.remove_kth_last(2);
    println!("{}", list.to_string()); // 1 2 3 5
}
