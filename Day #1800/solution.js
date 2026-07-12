// Palindrome integer check by reversing the number arithmetically (no string). Negatives are not palindromes. O(log10(n)).
function isPalindrome(x) {
  if (x < 0) return false;
  const original = x;
  let reversed = 0;
  while (x > 0) {
    reversed = reversed * 10 + (x % 10);
    x = Math.floor(x / 10);
  }
  return reversed === original;
}

console.log(isPalindrome(121)); // true
