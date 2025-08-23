// Day 157: A permutation is a palindrome iff at most one char has odd count.
// Track parity via a set of odd-count chars. Time O(n), Space O(alphabet).
'use strict';

function canFormPalindrome(s) {
  const odd = new Set();
  for (const c of s) {
    if (odd.has(c)) odd.delete(c);
    else odd.add(c);
  }
  return odd.size <= 1;
}

console.log(canFormPalindrome('carrace')); // true
console.log(canFormPalindrome('daily'));   // false
