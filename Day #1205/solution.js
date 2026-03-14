// Day 1205: Add two numbers stored as reversed-digit linked lists.
// Traverse both lists with a running carry. Time O(max(m,n)), Space O(max(m,n)).
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function build(ds) {
  const dummy = new Node(0);
  let t = dummy;
  for (const d of ds) { t.next = new Node(d); t = t.next; }
  return dummy.next;
}

function addLists(a, b) {
  const dummy = new Node(0);
  let t = dummy, carry = 0;
  while (a || b || carry) {
    const s = carry + (a ? a.val : 0) + (b ? b.val : 0);
    carry = Math.floor(s / 10);
    t.next = new Node(s % 10); t = t.next;
    if (a) a = a.next;
    if (b) b = b.next;
  }
  return dummy.next;
}

let s = addLists(build([9, 9]), build([5, 2]));
const out = [];
for (let p = s; p; p = p.next) out.push(p.val);
console.log(out.join(" -> ")); // 4 -> 2 -> 1
