// Day 963: Find intersecting node of two singly linked lists.
// Approach: two pointers swap heads; meet at intersection. Time O(M+N), Space O(1).

class Node {
  constructor(val) { this.val = val; this.next = null; }
}

function getIntersection(a, b) {
  if (!a || !b) return null;
  let p = a, q = b;
  while (p !== q) {
    p = p === null ? b : p.next;
    q = q === null ? a : q.next;
  }
  return p;
}

const n8 = new Node(8);
n8.next = new Node(10);
const a = new Node(3); a.next = new Node(7); a.next.next = n8;
const b = new Node(99); b.next = new Node(1); b.next.next = n8;

console.log("the node with value " + getIntersection(a, b).val);
