// Permutation-palindrome check: a permutation can be a palindrome iff at most one
// character has an odd frequency. Toggle membership in a set as we scan.
// Time: O(n); Space: O(alphabet).

function canPermutePalindrome(s) {
  const odd = new Set();
  for (const ch of s) {
    if (odd.has(ch)) odd.delete(ch);
    else odd.add(ch);
  }
  return odd.size <= 1;
}

console.log(canPermutePalindrome("carrace"));
console.log(canPermutePalindrome("daily"));
