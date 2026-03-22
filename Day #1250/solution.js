// Palindrome permutation: count char parities; valid iff <=1 char has odd count. O(n) time, O(1) space.
function canPermutePalindrome(s) {
  const cnt = new Map();
  for (const c of s) cnt.set(c, (cnt.get(c) || 0) + 1);
  let odd = 0;
  for (const v of cnt.values()) if (v & 1) odd++;
  return odd <= 1;
}

console.log(canPermutePalindrome("carrace"));
console.log(canPermutePalindrome("daily"));
