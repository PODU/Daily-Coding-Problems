// Remove all consecutive nodes summing to zero using prefix sums + hash map.
// A prefix sum seen before means the span between is zero-sum and is excised.
// Time O(n), Space O(n).
class Node {
  constructor(val, next = null) {
    this.val = val;
    this.next = next;
  }
}

function removeZeroSum(head) {
  const dummy = new Node(0, head);
  const seen = new Map();
  let s = 0;
  for (let p = dummy; p; p = p.next) {
    s += p.val;
    seen.set(s, p);
  }
  s = 0;
  for (let p = dummy; p; p = p.next) {
    s += p.val;
    p.next = seen.get(s).next;
  }
  return dummy.next;
}

let head = null;
for (const v of [3, 4, -7, 5, -6, 6].reverse()) head = new Node(v, head);
head = removeZeroSum(head);
const out = [];
for (let p = head; p; p = p.next) out.push(p.val);
console.log(out.join(" "));
