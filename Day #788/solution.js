// Integer palindrome check by arithmetic reversal (no string). O(log n) time, O(1) space. Negatives -> false.
function isPalindrome(n) {
  if (n < 0) return false;
  let rev = 0, x = n;
  while (x > 0) {
    rev = rev * 10 + (x % 10);
    x = Math.floor(x / 10);
  }
  return rev === n;
}

console.log(isPalindrome(121));
