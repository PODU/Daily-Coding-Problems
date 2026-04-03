// Day 1291: Next permutation of an integer's digits.
// Standard next-permutation: scan from right, swap, reverse suffix. O(D) time, O(D) space.
function nextPermutation(num) {
  const s = num.split("");
  const n = s.length;
  let i = n - 2;
  while (i >= 0 && s[i] >= s[i + 1]) i--;
  if (i < 0) return num; // already largest
  let j = n - 1;
  while (s[j] <= s[i]) j--;
  [s[i], s[j]] = [s[j], s[i]];
  for (let l = i + 1, r = n - 1; l < r; l++, r--) [s[l], s[r]] = [s[r], s[l]];
  return s.join("");
}

console.log(nextPermutation("48975")); // 49578
