// Palindrome permutation: toggle chars in a set; a permutation is a palindrome
// iff at most one char has an odd count (set size <= 1). Time O(n), Space O(alphabet).
function canFormPalindrome(s) {
  const odd = new Set();
  for (const c of s) {
    if (odd.has(c)) odd.delete(c);
    else odd.add(c);
  }
  return odd.size <= 1;
}

const s = "carrace";
console.log(canFormPalindrome(s) ? "true" : "false");
