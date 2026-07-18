// Min swaps to seat couples together. Union couples in each seat-pair; answer is
// N - (#connected components). O(N * alpha(N)).
function minSwaps(row) {
  const n = row.length / 2; // number of couples
  const par = Array.from({ length: n }, (_, i) => i);
  function find(x) {
    while (par[x] !== x) { par[x] = par[par[x]]; x = par[x]; }
    return x;
  }
  let comps = n;
  for (let i = 0; i < n; i++) {
    const ra = find(Math.floor(row[2 * i] / 2));
    const rb = find(Math.floor(row[2 * i + 1] / 2));
    if (ra !== rb) { par[ra] = rb; comps--; }
  }
  return n - comps;
}

const row = [0, 2, 1, 3]; // couples are (0,1) and (2,3)
console.log(minSwaps(row)); // 1
