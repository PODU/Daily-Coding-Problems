// Day 104: Linked-list palindrome (singly or doubly). Find middle, reverse second
// half, compare both halves. O(n) time, O(1) extra space.
class Node {
  constructor(val, next = null) { this.val = val; this.next = next; }
}

function isPalindrome(head) {
  let slow = head, fast = head;
  while (fast && fast.next) { slow = slow.next; fast = fast.next.next; }
  let prev = null;
  while (slow) { const nx = slow.next; slow.next = prev; prev = slow; slow = nx; }
  let l = head, r = prev;
  while (r) { if (l.val !== r.val) return false; l = l.next; r = r.next; }
  return true;
}

function build(vals) {
  let head = null;
  for (let i = vals.length - 1; i >= 0; i--) head = new Node(vals[i], head);
  return head;
}

console.log(isPalindrome(build([1, 4, 3, 4, 1]))); // true
console.log(isPalindrome(build([1, 4])));          // false
