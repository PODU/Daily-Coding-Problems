// Day 131: Deep clone a linked list with next + random pointers.
// Interleaving trick (weave copies, set randoms, unweave). O(n) time, O(1) extra space.
class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
    this.random = null;
  }
}

function clone(head) {
  if (!head) return null;
  for (let c = head; c; c = c.next.next) {
    const cp = new Node(c.val);
    cp.next = c.next;
    c.next = cp;
  }
  for (let c = head; c; c = c.next.next)
    if (c.random) c.next.random = c.random.next;
  const newHead = head.next;
  for (let c = head; c; c = c.next) {
    const cp = c.next;
    c.next = cp.next;
    if (cp.next) cp.next = cp.next.next;
  }
  return newHead;
}

const n = [];
for (let v = 1; v <= 5; v++) n.push(new Node(v));
for (let i = 0; i < 4; i++) n[i].next = n[i + 1];
n[0].random = n[2];
n[1].random = n[0];
n[2].random = n[4];
n[3].random = n[1];
n[4].random = n[4];

const copy = clone(n[0]);
let separate = true;
let o = n[0], c = copy;
while (c) {
  if (c === o) separate = false;
  console.log(`node ${c.val} -> random ${c.random ? c.random.val : 0}`);
  o = o.next;
  c = c.next;
}
console.log("deep copy verified: " + separate);
