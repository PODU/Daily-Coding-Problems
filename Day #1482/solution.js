// Day 1482: Integer palindrome without converting to a string.
// Reverse the number arithmetically and compare. Time O(digits), Space O(1).

function isPalindrome(x) {
  if (x < 0) return false;
  let rev = 0, n = x;
  while (n > 0) {
    rev = rev * 10 + (n % 10);
    n = Math.floor(n / 10);
  }
  return rev === x;
}

for (const v of [121, 888, 678]) {
  console.log(`${v} -> ${isPalindrome(v) ? "palindrome" : "not a palindrome"}`);
}
