// First N regular (Hamming) numbers via 3-pointer merge of {2,3,5} multiples. Time O(N), Space O(N).
function regular(n) {
  const h = new Array(n).fill(1);
  let i2 = 0, i3 = 0, i5 = 0;
  for (let i = 1; i < n; i++) {
    const n2 = h[i2] * 2, n3 = h[i3] * 3, n5 = h[i5] * 5;
    const m = Math.min(n2, n3, n5);
    h[i] = m;
    if (m === n2) i2++;
    if (m === n3) i3++;
    if (m === n5) i5++;
  }
  return h;
}

console.log('[' + regular(10).join(', ') + ']');
