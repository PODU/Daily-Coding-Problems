// Day 1627: Debounce f by N ms. Each call resets an N-ms timer; f fires
// only after a quiet gap of N ms. Time O(1) per call.
function debounce(f, n) {
  let timer = null;
  return (...args) => {
    if (timer) clearTimeout(timer);
    timer = setTimeout(() => { timer = null; f(...args); }, n);
  };
}

// Demo: 3 rapid calls collapse into a single invocation.
let calls = 0;
const g = debounce(() => calls++, 100);
g(); g(); g();
setTimeout(() => console.log(`f was called ${calls} time(s)`), 200);
