// Day 966: Deep clone a linked list where each node has a random pointer.
// Approach: interleave copies, set randoms, split. Time O(n), Space O(1) extra.

class Node {
  constructor(val) { this.val = val; this.next = null; this.random = null; }
}

function cloneList(head) {
  if (!head) return null;
  for (let p = head; p; p = p.next.next) {
    const c = new Node(p.val);
    c.next = p.next;
    p.next = c;
  }
  for (let p = head; p; p = p.next.next)
    if (p.random) p.next.random = p.random.next;
  const newHead = head.next;
  for (let p = head; p; p = p.next) {
    const c = p.next;
    p.next = c.next;
    if (c.next) c.next = c.next.next;
  }
  return newHead;
}

const a = new Node(1), b = new Node(2), c = new Node(3);
a.next = b; b.next = c;
a.random = c; b.random = a; c.random = b;

for (let p = cloneList(a); p; p = p.next)
  console.log(`val=${p.val} random=${p.random ? p.random.val : -1}`);
