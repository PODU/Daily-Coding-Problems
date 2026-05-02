// Egg drop: DP on trials. f(t,e)=f(t-1,e-1)+f(t-1,e)+1 = max floors with t trials, e eggs.
// Smallest t with f(t,N)>=k. Time O(N*answer), Space O(N).
'use strict';

// Minimum worst-case trials for N eggs and k floors.
function eggDrop(N, k) {
  if (k === 0) return 0;
  const f = new Array(N + 1).fill(0);
  let t = 0;
  while (f[N] < k) {
    t++;
    for (let e = N; e >= 1; e--) {
      f[e] = f[e] + f[e - 1] + 1;
    }
  }
  return t;
}

console.log(eggDrop(1, 5));
