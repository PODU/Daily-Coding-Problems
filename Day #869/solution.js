// Day 869: Is a linked list a palindrome (works for singly linked; doubly trivial).
// Approach: find middle (slow/fast), reverse second half, compare both halves. O(1) space.
// Time: O(n), Space: O(1).
'use strict';

class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
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

function isPalindrome(head) {
  let slow = head, fast = head;
  while (fast && fast.next) { slow = slow.next; fast = fast.next.next; }
  let prev = null;
  while (slow) { const nx = slow.next; slow.next = prev; prev = slow; slow = nx; }
  let a = head, b = prev;
  while (b) { if (a.val !== b.val) return false; a = a.next; b = b.next; }
  return true;
}

console.log(isPalindrome(build([1, 4, 3, 4, 1])) ? 'True' : 'False'); // True
console.log(isPalindrome(build([1, 4])) ? 'True' : 'False');         // False
