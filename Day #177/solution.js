// Rotate singly linked list right by k: form ring, break at (len - k%len). Time O(n), Space O(1).
'use strict';

class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function rotateRight(head, k) {
  if (!head || !head.next || k === 0) return head;
  let length = 1, tail = head;
  while (tail.next) { tail = tail.next; length++; }
  k %= length;
  if (k === 0) return head;
  tail.next = head; // make ring
  let steps = length - k;
  let newTail = head;
  for (let i = 0; i < steps - 1; i++) newTail = newTail.next;
  const newHead = newTail.next;
  newTail.next = null;
  return newHead;
}

function build(vals) {
  let head = null;
  for (let i = vals.length - 1; i >= 0; i--) head = new Node(vals[i], head);
  return head;
}

function toStr(head) {
  const parts = [];
  while (head) { parts.push(head.val); head = head.next; }
  return parts.join(" -> ");
}

console.log(toStr(rotateRight(build([7, 7, 3, 5]), 2)));
console.log(toStr(rotateRight(build([1, 2, 3, 4, 5]), 3)));
