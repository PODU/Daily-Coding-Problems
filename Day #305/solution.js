// Day 305: Remove consecutive nodes summing to zero. Prefix-sum + hashmap. O(N).
class Node { constructor(val) { this.val = val; this.next = null; } }

function removeZeroSum(head) {
  const dummy = new Node(0);
  dummy.next = head;
  const seen = new Map();
  let prefix = 0;
  for (let cur = dummy; cur; cur = cur.next) { prefix += cur.val; seen.set(prefix, cur); }
  prefix = 0;
  for (let cur = dummy; cur; cur = cur.next) { prefix += cur.val; cur.next = seen.get(prefix).next; }
  return dummy.next;
}

let head = null, tail = null;
for (const v of [3, 4, -7, 5, -6, 6]) {
  const n = new Node(v);
  if (!head) head = tail = n; else { tail.next = n; tail = n; }
}
head = removeZeroSum(head);
const out = [];
for (let c = head; c; c = c.next) out.push(c.val);
console.log(out.join(" ")); // 5
