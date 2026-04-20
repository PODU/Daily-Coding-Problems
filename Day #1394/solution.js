// Reverse singly linked list in-place: iterative 3-pointer (prev/cur/next). O(n) time, O(1) space.

class Node {
    constructor(val) {
        this.val = val;
        this.next = null;
    }
}

function reverseList(head) {
    let prev = null, cur = head;
    while (cur) {
        const next = cur.next;
        cur.next = prev;
        prev = cur;
        cur = next;
    }
    return prev;
}

function main() {
    let head = new Node(1);
    head.next = new Node(2);
    head.next.next = new Node(3);
    head.next.next.next = new Node(4);
    head.next.next.next.next = new Node(5);

    head = reverseList(head);

    const parts = [];
    for (let p = head; p; p = p.next) parts.push(p.val);
    console.log(parts.join(" -> "));
}

main();
