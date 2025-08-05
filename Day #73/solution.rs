// Reverse singly linked list in place: iterate with prev/curr/next pointers. Time O(n), Space O(1).
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

fn main() {
    // Build 1->2->3->4->5
    let mut head: Option<Box<ListNode>> = None;
    for v in (1..=5).rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }

    head = reverse_list(head);

    let mut vals: Vec<String> = Vec::new();
    let mut cur = &head;
    while let Some(node) = cur {
        vals.push(node.val.to_string());
        cur = &node.next;
    }
    println!("{}", vals.join(" "));
}
