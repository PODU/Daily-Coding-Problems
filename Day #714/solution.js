// Day 714: Swap every two adjacent nodes in a singly linked list. Iterative
// pointer rewiring with a dummy head. Time O(n), space O(1).
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function swapPairs(head) {
  const dummy = new Node(0, head);
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

let head = new Node(1, new Node(2, new Node(3, new Node(4))));
head = swapPairs(head);
const out = [];
for (let c = head; c; c = c.next) out.push(c.val);
console.log(out.join(" -> "));
