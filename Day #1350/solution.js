// Remove consecutive nodes summing to zero: dummy head, prefix-sum -> last node map;
// repeated prefix means a zero-sum span to splice out. Time O(n), Space O(n).
class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function removeZeroSum(head) {
  const dummy = new Node(0);
  dummy.next = head;
  const seen = new Map();
  let prefix = 0;
  for (let cur = dummy; cur; cur = cur.next) {
    prefix += cur.val;
    seen.set(prefix, cur); // last node achieving this prefix sum
  }
  prefix = 0;
  for (let cur = dummy; cur; cur = cur.next) {
    prefix += cur.val;
    cur.next = seen.get(prefix).next; // skip zero-sum span
  }
  return dummy.next;
}

function build(vals) {
  let head = null, tail = null;
  for (const v of vals) {
    const n = new Node(v);
    if (!head) head = tail = n;
    else { tail.next = n; tail = n; }
  }
  return head;
}

let head = build([3, 4, -7, 5, -6, 6]);
head = removeZeroSum(head);
const parts = [];
for (let c = head; c; c = c.next) parts.push(c.val);
console.log(parts.join(" -> "));
