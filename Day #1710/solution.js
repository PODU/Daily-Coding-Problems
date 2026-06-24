// Deep clone list w/ random ptr: interleave clones, wire randoms, unweave. O(n) time, O(1) extra.
class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
    this.random = null;
  }
}

function copyRandomList(head) {
  if (!head) return null;
  let c = head;
  while (c) {
    const cl = new Node(c.val);
    cl.next = c.next;
    c.next = cl;
    c = cl.next;
  }
  c = head;
  while (c) {
    c.next.random = c.random ? c.random.next : null;
    c = c.next.next;
  }
  const newHead = head.next;
  c = head;
  while (c) {
    const cl = c.next;
    c.next = cl.next;
    cl.next = cl.next ? cl.next.next : null;
    c = c.next;
  }
  return newHead;
}

const n1 = new Node(1), n2 = new Node(2), n3 = new Node(3), n4 = new Node(4);
n1.next = n2; n2.next = n3; n3.next = n4;
n1.random = n3;
n2.random = n1;
n3.random = n3;
n4.random = n2;

let c = copyRandomList(n1);
while (c) {
  console.log(`node ${c.val}, random ${c.random.val}`);
  c = c.next;
}
