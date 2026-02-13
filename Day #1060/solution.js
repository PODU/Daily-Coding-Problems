// Debounce: wrap f so it runs only after N ms of silence; each call resets the timer.
// Uses setTimeout/clearTimeout. Time: O(1) per call, Space: O(1).
"use strict";

function debounce(f, n) {
  let timer = null;
  return function (...args) {
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => {
      timer = null;
      f.apply(this, args);
    }, n);
  };
}

// Demo: burst of 5 calls, then wait > N ms -> f runs ONCE.
let calls = 0;
const debounced = debounce((x) => {
  calls++;
  console.log(`f called with ${x}; total calls = ${calls}`);
}, 100);

for (let i = 1; i <= 5; i++) debounced(i);

setTimeout(() => {
  console.log(`done; f ran ${calls} time(s)`);
}, 300);
