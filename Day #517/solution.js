// Intersection: two pointers switch lists after end; meet at the join. O(M+N) time, O(1) space.
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
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

const shared = new Node(8, new Node(10));
const A = new Node(3, new Node(7, shared));
const B = new Node(99, new Node(1, shared));
console.log("The node with value " + getIntersection(A, B).val);
