// Rearrange linked list values to low->high->low->high. One pass swapping
// adjacent values to enforce the alternating relation. Time O(n), Space O(1).
'use strict';

class Node { constructor(val) { this.val = val; this.next = null; } }

function zigzag(head) {
  let low = true; // current pair should satisfy a <= b
  for (let c = head; c && c.next; c = c.next, low = !low) {
    if (low ? c.val > c.next.val : c.val < c.next.val) {
      [c.val, c.next.val] = [c.next.val, c.val];
    }
  }
}

function build(vals) {
  let head = null, tail = null;
  for (const v of vals) {
    const n = new Node(v);
    if (!head) head = tail = n; else { tail.next = n; tail = n; }
  }
  return head;
}

function toStr(head) {
  const parts = [];
  for (let c = head; c; c = c.next) parts.push(c.val);
  return parts.join(' -> ');
}

const h = build([1, 2, 3, 4, 5]);
zigzag(h);
console.log(toStr(h)); // 1 -> 3 -> 2 -> 5 -> 4
