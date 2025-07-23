// Intersection of two linked lists: two-pointer switch trick.
// Time O(M+N), Space O(1).
class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function getIntersection(a, b) {
  let pa = a, pb = b;
  while (pa !== pb) {
    pa = pa === null ? b : pa.next;
    pb = pb === null ? a : pb.next;
  }
  return pa;
}

const shared = new Node(8);
shared.next = new Node(10);

const a = new Node(3);
a.next = new Node(7);
a.next.next = shared;

const b = new Node(99);
b.next = new Node(1);
b.next.next = shared;

console.log(getIntersection(a, b).val);
