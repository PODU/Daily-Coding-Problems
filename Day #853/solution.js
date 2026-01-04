// Day 853: smallest distance (number of words between) between two words in a text.
// Single pass tracking last index of each word. distance = |i-j| - 1. O(n) time, O(1) space.
function minDistance(text, w1, w2) {
  const words = text.split(" ");
  let p1 = -1, p2 = -1, best = Infinity;
  for (let i = 0; i < words.length; i++) {
    if (words[i] === w1) p1 = i;
    if (words[i] === w2) p2 = i;
    if (p1 !== -1 && p2 !== -1) best = Math.min(best, Math.abs(p1 - p2) - 1);
  }
  return best;
}

const text = "dog cat hello cat dog dog hello cat world";
console.log(minDistance(text, "hello", "world")); // 1
