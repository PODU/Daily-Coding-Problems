// Next permutation of the digit array: find rightmost ascending pair, swap with
// next-greater suffix digit, reverse suffix. Time O(d), Space O(d).

function nextPermutation(num) {
  const s = String(num).split('');
  const n = s.length;
  let i = n - 2;
  while (i >= 0 && s[i] >= s[i + 1]) i--;
  if (i < 0) return -1; // no next permutation
  let j = n - 1;
  while (s[j] <= s[i]) j--;
  [s[i], s[j]] = [s[j], s[i]];
  for (let l = i + 1, r = n - 1; l < r; l++, r--) {
    [s[l], s[r]] = [s[r], s[l]];
  }
  return parseInt(s.join(''), 10);
}

console.log(nextPermutation(48975));
