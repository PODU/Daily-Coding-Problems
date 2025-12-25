// Day 800: Rearrange list values into low->high->low... (wiggle).
// One pass: at even idx ensure a<=next, at odd idx ensure a>=next; swap if not.
// Time O(N), Space O(1).

class Node {
  constructor(val) {
    this.val = val;
    this.next = null;
  }
}

function wiggle(head) {
  let less = true;
  for (let cur = head; cur && cur.next; cur = cur.next) {
    if (less ? cur.val > cur.next.val : cur.val < cur.next.val) {
      [cur.val, cur.next.val] = [cur.next.val, cur.val];
    }
    less = !less;
  }
}

let head = new Node(1);
let c = head;
for (const v of [2, 3, 4, 5]) {
  c.next = new Node(v);
  c = c.next;
}
wiggle(head);
const out = [];
for (let p = head; p; p = p.next) out.push(p.val);
console.log(out.join(" -> ")); // 1 -> 3 -> 2 -> 5 -> 4
