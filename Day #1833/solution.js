// Reconstruct a permutation of [0..N] from +/- signs. Two-pointer, O(N).
// '+' takes the current low, '-' takes the current high; produces a valid order.
function reconstruct(signs) { // '+'/'-' constraints (leading None dropped)
  const L = signs.length + 1, N = L - 1;
  const res = new Array(L);
  let low = 0, high = N;
  for (let j = 0; j < signs.length; j++) {
    if (signs[j] === "+") res[j] = low++;
    else res[j] = high--;
  }
  res[L - 1] = low;
  return res;
}

// input [None, +, +, -, +] -> constraints (+, +, -, +)
const signs = ["+", "+", "-", "+"];
console.log("[" + reconstruct(signs).join(", ") + "]"); // e.g. [0, 1, 4, 2, 3]
