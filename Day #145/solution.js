// Swap every two adjacent nodes in a singly linked list via iterative pointer rewiring. O(n) time, O(1) space.

class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function swapPairs(head) {
  const dummy = new Node(0);
  dummy.next = head;
  let prev = dummy;
  while (prev.next && prev.next.next) {
    const a = prev.next;
    const b = a.next;
    a.next = b.next;
    b.next = a;
    prev.next = b;
    prev = a;
  }
  return dummy.next;
}

let head = new Node(1);
head.next = new Node(2);
head.next.next = new Node(3);
head.next.next.next = new Node(4);
head = swapPairs(head);
const parts = [];
for (let c = head; c; c = c.next) parts.push(c.val);
console.log(parts.join(" -> ")); // 2 -> 1 -> 4 -> 3
