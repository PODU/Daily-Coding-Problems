// Day 1432: Deep clone a linked list with a random pointer.
// Approach: interleave cloned nodes, wire randoms, then split. Time: O(n), Space: O(1) extra.

class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
    this.random = null;
  }
}

function cloneList(head) {
  if (!head) return null;
  for (let cur = head; cur; cur = cur.next.next) {
    const copy = new Node(cur.val);
    copy.next = cur.next;
    cur.next = copy;
  }
  for (let cur = head; cur; cur = cur.next.next) {
    if (cur.random) cur.next.random = cur.random.next;
  }
  const newHead = head.next;
  for (let cur = head; cur; cur = cur.next) {
    const copy = cur.next;
    cur.next = copy.next;
    if (copy.next) copy.next = copy.next.next;
  }
  return newHead;
}

const a = new Node(1), b = new Node(2), c = new Node(3);
a.next = b; b.next = c;
a.random = c; b.random = a; c.random = c;

const cloned = cloneList(a);
let ok = true;
let p = a, q = cloned;
while (p) {
  if (q === p) ok = false;
  if (q.val !== p.val) ok = false;
  if ((p.random === null) !== (q.random === null)) ok = false;
  if (p.random && q.random.val !== p.random.val) ok = false;
  p = p.next; q = q.next;
}
console.log(ok
  ? "Clone verified: values and random targets match, nodes distinct"
  : "Clone FAILED");
