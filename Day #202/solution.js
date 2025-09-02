// Day 202: Integer palindrome check without string conversion.
// Reverse the second half of the digits and compare with the first half.
// Time: O(log10 n), Space: O(1).
function isPalindrome(x) {
  if (x < 0) return false;
  if (x !== 0 && x % 10 === 0) return false; // trailing zero, not 0 itself
  let rev = 0;
  while (x > rev) { rev = rev * 10 + (x % 10); x = Math.floor(x / 10); }
  return x === rev || x === Math.floor(rev / 10);
}

console.log(isPalindrome(121)); // true
console.log(isPalindrome(888)); // true
console.log(isPalindrome(678)); // false
