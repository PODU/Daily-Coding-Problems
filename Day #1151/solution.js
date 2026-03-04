// Day 1151: Palindrome linked list (singly).
// Find middle via slow/fast, reverse 2nd half, compare. O(n) time, O(1) space.
class Node {
  constructor(val) { this.val = val; this.next = null; }
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
  const dummy = new Node(0);
  let t = dummy;
  for (const x of vals) { t.next = new Node(x); t = t.next; }
  return dummy.next;
}

console.log(isPalindrome(build([1, 4, 3, 4, 1])) ? "True" : "False"); // True
console.log(isPalindrome(build([1, 4])) ? "True" : "False");          // False
