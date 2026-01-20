// Sort a linked list in O(n log n) time, O(1) extra space.
// Bottom-up (iterative) merge sort using raw pointers; no recursion stack -> constant space.
use std::ptr;

struct Node {
    val: i64,
    next: *mut Node,
}

unsafe fn split(mut head: *mut Node, size: usize) -> *mut Node {
    let mut i = 1;
    while !head.is_null() && i < size {
        head = (*head).next;
        i += 1;
    }
    if head.is_null() {
        return ptr::null_mut();
    }
    let rest = (*head).next;
    (*head).next = ptr::null_mut();
    rest
}

unsafe fn merge(mut a: *mut Node, mut b: *mut Node, mut tail: *mut Node) -> *mut Node {
    while !a.is_null() && !b.is_null() {
        if (*a).val <= (*b).val {
            (*tail).next = a;
            a = (*a).next;
        } else {
            (*tail).next = b;
            b = (*b).next;
        }
        tail = (*tail).next;
    }
    (*tail).next = if !a.is_null() { a } else { b };
    while !(*tail).next.is_null() {
        tail = (*tail).next;
    }
    tail
}

unsafe fn sort_list(head: *mut Node) -> *mut Node {
    if head.is_null() || (*head).next.is_null() {
        return head;
    }
    let mut n = 0;
    let mut p = head;
    while !p.is_null() {
        n += 1;
        p = (*p).next;
    }
    let dummy = Box::into_raw(Box::new(Node { val: 0, next: head }));
    let mut size = 1;
    while size < n {
        let mut cur = (*dummy).next;
        let mut tail = dummy;
        while !cur.is_null() {
            let left = cur;
            let right = split(left, size);
            cur = split(right, size);
            tail = merge(left, right, tail);
        }
        size *= 2;
    }
    let result = (*dummy).next;
    drop(Box::from_raw(dummy));
    result
}

fn main() {
    unsafe {
        let vals = [4i64, 1, -3, 99];
        let dummy = Box::into_raw(Box::new(Node { val: 0, next: ptr::null_mut() }));
        let mut t = dummy;
        for &v in vals.iter() {
            let node = Box::into_raw(Box::new(Node { val: v, next: ptr::null_mut() }));
            (*t).next = node;
            t = node;
        }
        let head = (*dummy).next;
        drop(Box::from_raw(dummy));
        let sorted = sort_list(head);
        let mut parts: Vec<String> = Vec::new();
        let mut p = sorted;
        while !p.is_null() {
            parts.push((*p).val.to_string());
            p = (*p).next;
        }
        println!("{}", parts.join(" -> "));
        // free
        let mut p = sorted;
        while !p.is_null() {
            let next = (*p).next;
            drop(Box::from_raw(p));
            p = next;
        }
    }
}
