// Egg drop (min worst-case trials): find smallest m such that with N eggs we can
// cover k floors. f(m, N) = sum_{i=1..N} C(m,i); smallest m with f(m,N) >= k.
// Time: O(m * N) where m is the answer; Space: O(1).

function eggDrop(n, k) {
  let m = 0;
  let covered = 0;
  while (covered < k) {
    m++;
    let sum = 0;
    let term = 1; // term = C(m, i)
    for (let i = 1; i <= n; i++) {
      term = (term * (m - i + 1)) / i; // C(m, i)
      sum += term;
      if (sum >= k) break;
    }
    covered = sum;
  }
  return m;
}

console.log(eggDrop(1, 5)); // expected 5
