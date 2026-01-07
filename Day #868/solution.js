// Day 868: Can any permutation of the string form a palindrome?
// Approach: count chars; palindrome possible iff at most one char has an odd count.
// Time: O(n), Space: O(alphabet).
'use strict';

function canFormPalindrome(s) {
  const cnt = new Map();
  for (const c of s) cnt.set(c, (cnt.get(c) || 0) + 1);
  let odd = 0;
  for (const v of cnt.values()) odd += v & 1;
  return odd <= 1;
}

console.log(canFormPalindrome('carrace')); // true
console.log(canFormPalindrome('daily'));   // false
