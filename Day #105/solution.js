// Day 105: Debounce. Each call clears the pending timer and sets a fresh N-ms one,
// so f only fires after N ms with no further calls. O(1) per call.
function debounce(f, n) {
  let timer = null;
  return (...args) => {
    clearTimeout(timer);
    timer = setTimeout(() => f(...args), n);
  };
}

let calls = 0;
const d = debounce(() => { calls++; console.log("f called"); }, 100);
d(); d(); d();                                            // 3 rapid calls
setTimeout(() => console.log("Total calls to f:", calls), 300); // 1
