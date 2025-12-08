// Day 715: Linked-list palindrome. Find middle (slow/fast), reverse second half,
// compare. Works for singly linked in O(n) time, O(1) space.
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function isPalindrome(head) {
  if (!head || !head.next) return true;
  let slow = head, fast = head;
  while (fast.next && fast.next.next) { slow = slow.next; fast = fast.next.next; }
  let prev = null, cur = slow.next;
  while (cur) { const nx = cur.next; cur.next = prev; prev = cur; cur = nx; }
  let p1 = head, p2 = prev;
  while (p2) { if (p1.val !== p2.val) return false; p1 = p1.next; p2 = p2.next; }
  return true;
}

function build(vals) {
  let head = null;
  for (let i = vals.length - 1; i >= 0; i--) head = new Node(vals[i], head);
  return head;
}

console.log(isPalindrome(build([1, 4, 3, 4, 1])) ? "True" : "False");
console.log(isPalindrome(build([1, 4])) ? "True" : "False");
