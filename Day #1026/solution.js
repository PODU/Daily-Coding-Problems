// Day 1026: Full text justification.
// Greedy line packing; distribute extra spaces evenly, leftover from the left.
// Time O(total chars), Space O(total chars).
function justify(words, k) {
  const res = [];
  const n = words.length;
  let i = 0;
  while (i < n) {
    let j = i;
    let lineLen = words[i].length;
    while (j + 1 < n && lineLen + 1 + words[j + 1].length <= k) {
      lineLen += 1 + words[j + 1].length;
      j++;
    }
    const cnt = j - i + 1;
    if (cnt === 1) {
      res.push(words[i] + " ".repeat(k - words[i].length));
    } else {
      let totalChars = 0;
      for (let w = i; w <= j; w++) totalChars += words[w].length;
      const spaces = k - totalChars;
      const gaps = cnt - 1;
      const base = Math.floor(spaces / gaps);
      const extra = spaces % gaps;
      let line = "";
      for (let w = i; w <= j; w++) {
        line += words[w];
        if (w < j) line += " ".repeat(base + (w - i < extra ? 1 : 0));
      }
      res.push(line);
    }
    i = j + 1;
  }
  return res;
}

const words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
for (const line of justify(words, 16)) console.log(line);
