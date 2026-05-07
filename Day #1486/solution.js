// Day 1486: Partition a linked list around pivot k (stable).
// Approach: build two sublists (< k and >= k), then concatenate. O(n) time, O(1) extra space.

class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function partition(head, k) {
  const lessDummy = new Node(0), geDummy = new Node(0);
  let less = lessDummy, ge = geDummy;
  for (let cur = head; cur; cur = cur.next) {
    if (cur.val < k) { less.next = cur; less = cur; }
    else { ge.next = cur; ge = cur; }
  }
  ge.next = null;
  less.next = geDummy.next;
  return lessDummy.next;
}

function build(vals) {
  let head = null, tail = null;
  for (const v of vals) {
    const n = new Node(v);
    if (!head) head = tail = n; else { tail.next = n; tail = n; }
  }
  return head;
}

let head = partition(build([5, 1, 8, 0, 3]), 3);
const out = [];
while (head) { out.push(head.val); head = head.next; }
console.log(out.join(" -> "));
