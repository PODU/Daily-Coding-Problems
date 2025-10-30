// Sevenish: nth value = sum of 7^i for each set bit i of n. O(log n) per query.
function sevenish(n) {
  let sum = 0, pow7 = 1;
  while (n > 0) {
    if (n & 1) sum += pow7;
    pow7 *= 7;
    n >>= 1;
  }
  return sum;
}

// First few sevenish numbers: 1, 7, 8, 49, ...
console.log([1, 2, 3, 4].map(sevenish).join(", "));
