// Day 1677: Linked-list palindrome. Singly: find middle, reverse 2nd half, compare.
// Doubly: two pointers from both ends. Time O(n), Space O(1).
class Node {
  constructor(val) { this.val = val; this.next = null; this.prev = null; }
}

function build(values) {
  let head = null, tail = null;
  for (const x of values) {
    const n = new Node(x);
    if (!head) head = tail = n;
    else { tail.next = n; n.prev = tail; tail = n; }
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

console.log(isPalindrome(build([1, 4, 3, 4, 1]))); // true
console.log(isPalindrome(build([1, 4])));          // false
