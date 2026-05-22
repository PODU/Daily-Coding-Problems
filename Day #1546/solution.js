// Day 1546: Stable partition of a linked list around pivot k.
// Build two sublists (< k) and (>= k) preserving order, then splice. Time O(n), Space O(1).
"use strict";

class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function partition(head, k) {
  const lessD = new Node(0), geD = new Node(0);
  let l = lessD, g = geD;
  for (let c = head; c; c = c.next) {
    if (c.val < k) { l.next = c; l = c; }
    else           { g.next = c; g = c; }
  }
  g.next = null;
  l.next = geD.next;
  return lessD.next;
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
  return parts.join(" -> ");
}

let head = build([5, 1, 8, 0, 3]);
head = partition(head, 3);
console.log(toStr(head));
