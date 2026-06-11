// Min word distance: single pass tracking last-seen index of each word; on each
// hit, update min with |i-j|-1 (words strictly between). Time O(n), Space O(1).
"use strict";

function minWordDistance(text, w1, w2) {
  let last1 = -1, last2 = -1, best = Infinity;
  for (let i = 0; i < text.length; i++) {
    if (text[i] === w1) {
      last1 = i;
      if (last2 !== -1) best = Math.min(best, Math.abs(last1 - last2) - 1);
    }
    if (text[i] === w2) {
      last2 = i;
      if (last1 !== -1) best = Math.min(best, Math.abs(last1 - last2) - 1);
    }
  }
  return best;
}

const text = "dog cat hello cat dog dog hello cat world".split(" ");
console.log(minWordDistance(text, "hello", "world"));
