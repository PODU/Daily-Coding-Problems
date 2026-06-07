// Day 1626: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. Time O(max(m,n)), Space O(max(m,n)).
class Node {
  constructor(val, next = null) {
    this.val = val;
    this.next = next;
  }
}

function addLists(a, b) {
  const dummy = new Node(0);
  let tail = dummy, carry = 0;
  while (a || b || carry) {
    let sum = carry;
    if (a) { sum += a.val; a = a.next; }
    if (b) { sum += b.val; b = b.next; }
    carry = Math.floor(sum / 10);
    tail.next = new Node(sum % 10);
    tail = tail.next;
  }
  return dummy.next;
}

function build(vals) {
  const dummy = new Node(0);
  let t = dummy;
  for (const x of vals) { t.next = new Node(x); t = t.next; }
  return dummy.next;
}

let r = addLists(build([9, 9]), build([5, 2]));
const parts = [];
while (r) { parts.push(r.val); r = r.next; }
console.log(parts.join(" -> "));
