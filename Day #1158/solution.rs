// Bottom-up iterative merge sort on a singly linked list. O(n log n) time, O(1) auxiliary space.
// Uses index-based arena (Vec of nodes) so links are rewired in place without extra allocation per pass.

struct Node {
    val: i32,
    next: Option<usize>,
}

// Take `n` nodes starting at `head`; cut after them, return the index of the remainder.
fn split(nodes: &mut Vec<Node>, head: Option<usize>, n: usize) -> Option<usize> {
    let mut cur = head;
    let mut i = 1;
    while let Some(idx) = cur {
        if i >= n {
            break;
        }
        cur = nodes[idx].next;
        i += 1;
    }
    match cur {
        None => None,
        Some(idx) => {
            let second = nodes[idx].next;
            nodes[idx].next = None;
            second
        }
    }
}

// Merge sorted lists a, b onto tail; return new tail index.
fn merge(nodes: &mut Vec<Node>, mut a: Option<usize>, mut b: Option<usize>, tail: usize) -> usize {
    let mut cur = tail;
    while let (Some(ai), Some(bi)) = (a, b) {
        if nodes[ai].val <= nodes[bi].val {
            nodes[cur].next = Some(ai);
            a = nodes[ai].next;
        } else {
            nodes[cur].next = Some(bi);
            b = nodes[bi].next;
        }
        cur = nodes[cur].next.unwrap();
    }
    nodes[cur].next = if a.is_some() { a } else { b };
    while let Some(nxt) = nodes[cur].next {
        cur = nxt;
    }
    cur
}

fn sort_list(nodes: &mut Vec<Node>, head: Option<usize>) -> Option<usize> {
    let n = {
        let mut c = 0;
        let mut p = head;
        while let Some(idx) = p {
            c += 1;
            p = nodes[idx].next;
        }
        c
    };
    if n <= 1 {
        return head;
    }

    // Dummy node appended to arena.
    let dummy = nodes.len();
    nodes.push(Node { val: 0, next: head });

    let mut size = 1;
    while size < n {
        let mut cur = nodes[dummy].next;
        let mut tail = dummy;
        while let Some(left) = cur {
            let right = split(nodes, Some(left), size);
            cur = split(nodes, right, size);
            tail = merge(nodes, Some(left), right, tail);
        }
        size <<= 1;
    }
    nodes[dummy].next
}

fn main() {
    let vals = [4, 1, -3, 99];
    let mut nodes: Vec<Node> = Vec::new();
    let mut head: Option<usize> = None;
    let mut tail: Option<usize> = None;
    for &v in vals.iter() {
        let idx = nodes.len();
        nodes.push(Node { val: v, next: None });
        match tail {
            None => head = Some(idx),
            Some(t) => nodes[t].next = Some(idx),
        }
        tail = Some(idx);
    }

    let mut cur = sort_list(&mut nodes, head);

    let mut parts: Vec<String> = Vec::new();
    while let Some(idx) = cur {
        parts.push(nodes[idx].val.to_string());
        cur = nodes[idx].next;
    }
    println!("{}", parts.join(" -> "));
}
