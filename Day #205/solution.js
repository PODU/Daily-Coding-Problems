// Day 205: Next permutation of an integer's digits.
// Standard next-permutation: find rightmost ascent, swap with next-larger suffix digit, reverse suffix.
// Time: O(d), Space: O(d) for digits.
function nextPermutation(n) {
  const s = String(n).split("");
  let i = s.length - 2;
  while (i >= 0 && s[i] >= s[i + 1]) i--;
  if (i < 0) return n; // already the largest permutation
  let j = s.length - 1;
  while (s[j] <= s[i]) j--;
  [s[i], s[j]] = [s[j], s[i]];
  for (let l = i + 1, r = s.length - 1; l < r; l++, r--) [s[l], s[r]] = [s[r], s[l]];
  return Number(s.join(""));
}

console.log(nextPermutation(48975)); // 49578
