// Day 153: Min words separating two words. Single pass tracking last seen index
// of each word; answer is min(|i-j|-1). Time O(n), Space O(1).
'use strict';

function minDistance(words, a, b) {
  let lastA = -1, lastB = -1, best = Infinity;
  for (let i = 0; i < words.length; i++) {
    if (words[i] === a) {
      lastA = i;
      if (lastB !== -1) best = Math.min(best, Math.abs(lastA - lastB) - 1);
    }
    if (words[i] === b) {
      lastB = i;
      if (lastA !== -1) best = Math.min(best, Math.abs(lastA - lastB) - 1);
    }
  }
  return best;
}

const text = 'dog cat hello cat dog dog hello cat world';
console.log(minDistance(text.split(/\s+/), 'hello', 'world'));
