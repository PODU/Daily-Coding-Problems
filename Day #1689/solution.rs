// Sort a singly linked list via bottom-up (iterative) merge sort.
// O(n log n) time, O(1) extra space (no recursion).

type Link = Option<Box<Node>>;
struct Node { val: i32, next: Link }

fn length(mut h: &Link) -> usize {
    let mut n = 0;
    while let Some(node) = h {
        n += 1;
        h = &node.next;
    }
    n
}

// Cut list to first n nodes; return (first chunk, remainder).
fn split(mut head: Link, n: usize) -> (Link, Link) {
    if head.is_none() {
        return (None, None);
    }
    {
        let mut cur = head.as_mut().unwrap();
        for _ in 1..n {
            if cur.next.is_none() {
                break;
            }
            cur = cur.next.as_mut().unwrap();
        }
        let rest = cur.next.take();
        (head, rest)
    }
}

fn merge(mut a: Link, mut b: Link) -> Link {
    let mut dummy = Box::new(Node { val: 0, next: None });
    let mut tail = &mut dummy;
    while a.is_some() && b.is_some() {
        let take_a = a.as_ref().unwrap().val <= b.as_ref().unwrap().val;
        if take_a {
            let mut node = a.take().unwrap();
            a = node.next.take();
            tail.next = Some(node);
        } else {
            let mut node = b.take().unwrap();
            b = node.next.take();
            tail.next = Some(node);
        }
        tail = tail.next.as_mut().unwrap();
    }
    tail.next = if a.is_some() { a } else { b };
    dummy.next
}

fn sort_list(mut head: Link) -> Link {
    let n = length(&head);
    let mut size = 1;
    while size < n {
        let mut new_dummy = Box::new(Node { val: 0, next: None });
        let mut new_tail = &mut new_dummy;
        let mut cur = head.take();
        while cur.is_some() {
            let (left, rest) = split(cur, size);
            let (right, rest2) = split(rest, size);
            cur = rest2;
            new_tail.next = merge(left, right);
            while new_tail.next.is_some() {
                new_tail = new_tail.next.as_mut().unwrap();
            }
        }
        head = new_dummy.next.take();
        size <<= 1;
    }
    head
}

fn main() {
    let vals = [4, 1, -3, 99];
    let mut head: Link = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(Node { val: v, next: head }));
    }
    head = sort_list(head);
    let mut parts: Vec<String> = Vec::new();
    let mut cur = &head;
    while let Some(node) = cur {
        parts.push(node.val.to_string());
        cur = &node.next;
    }
    println!("{}", parts.join(" -> "));
}
