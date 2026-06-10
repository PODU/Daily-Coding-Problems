// Swap every two adjacent nodes in a singly linked list via iterative pointer swaps (Box-based).
// Time: O(n), Space: O(1) extra (excluding the list itself).

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn swap_pairs(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    // If there are at least two nodes, swap them, then recurse on the rest.
    match head {
        Some(mut first) => {
            match first.next.take() {
                Some(mut second) => {
                    let rest = second.next.take();
                    first.next = swap_pairs(rest);
                    second.next = Some(first);
                    Some(second)
                }
                None => {
                    head = Some(first);
                    head
                }
            }
        }
        None => None,
    }
}

fn main() {
    let head = Some(Box::new(Node {
        val: 1,
        next: Some(Box::new(Node {
            val: 2,
            next: Some(Box::new(Node {
                val: 3,
                next: Some(Box::new(Node { val: 4, next: None })),
            })),
        })),
    }));

    let head = swap_pairs(head);

    let mut parts: Vec<String> = Vec::new();
    let mut cur = &head;
    while let Some(node) = cur {
        parts.push(node.val.to_string());
        cur = &node.next;
    }
    println!("{}", parts.join(" -> "));
}
