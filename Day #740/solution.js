// First N regular (5-smooth / Hamming) numbers via 3-pointer dynamic programming.
// Each number is min of next multiples by 2, 3, 5.
// Time: O(N), Space: O(N).

function regularNumbers(n) {
  const h = new Array(n);
  h[0] = 1;
  let i2 = 0, i3 = 0, i5 = 0;
  for (let i = 1; i < n; i++) {
    const n2 = h[i2] * 2, n3 = h[i3] * 3, n5 = h[i5] * 5;
    const nx = Math.min(n2, n3, n5);
    h[i] = nx;
    if (nx === n2) i2++;
    if (nx === n3) i3++;
    if (nx === n5) i5++;
  }
  return h;
}

console.log(regularNumbers(10).join(" ")); // 1 2 3 4 5 6 8 9 10 12
