// Day 1180: Swap every two adjacent nodes in a singly linked list.
// Iterative pointer rewiring with a dummy head. Time O(N), Space O(1).

class ListNode {
    constructor(val) { this.val = val; this.next = null; }
}

function swapPairs(head) {
    const dummy = new ListNode(0);
    dummy.next = head;
    let prev = dummy;
    while (prev.next && prev.next.next) {
        const a = prev.next, b = a.next;
        a.next = b.next;
        b.next = a;
        prev.next = b;
        prev = a;
    }
    return dummy.next;
}

function build(vals) {
    const dummy = new ListNode(0);
    let t = dummy;
    for (const x of vals) { t.next = new ListNode(x); t = t.next; }
    return dummy.next;
}

function toStr(h) {
    const parts = [];
    for (; h; h = h.next) parts.push(h.val);
    return parts.join(" -> ");
}

console.log(toStr(swapPairs(build([1, 2, 3, 4])))); // 2 -> 1 -> 4 -> 3
