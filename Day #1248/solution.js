// Concatenation substring indices via sliding window over wordLen offsets with hashmap counts. O(n*wordLen) time, O(m) space.

function findSubstring(s, words) {
  const res = [];
  if (words.length === 0 || s.length === 0) return res;
  const wordLen = words[0].length;
  const numWords = words.length;
  const windowLen = wordLen * numWords;
  if (s.length < windowLen) return res;

  const need = new Map();
  for (const w of words) need.set(w, (need.get(w) || 0) + 1);

  for (let offset = 0; offset < wordLen; ++offset) {
    const window = new Map();
    let count = 0;
    let left = offset;
    for (let right = offset; right + wordLen <= s.length; right += wordLen) {
      const word = s.substr(right, wordLen);
      if (need.has(word)) {
        window.set(word, (window.get(word) || 0) + 1);
        count++;
        while (window.get(word) > need.get(word)) {
          const lw = s.substr(left, wordLen);
          window.set(lw, window.get(lw) - 1);
          count--;
          left += wordLen;
        }
        if (count === numWords) {
          res.push(left);
          const lw = s.substr(left, wordLen);
          window.set(lw, window.get(lw) - 1);
          count--;
          left += wordLen;
        }
      } else {
        window.clear();
        count = 0;
        left = right + wordLen;
      }
    }
  }
  res.sort((a, b) => a - b);
  return res;
}

function main() {
  const s = "dogcatcatcodecatdog";
  const words = ["cat", "dog"];
  const res = findSubstring(s, words);
  console.log("[" + res.join(", ") + "]");
}

main();
