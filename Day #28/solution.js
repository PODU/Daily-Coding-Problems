// Text justification: greedy line packing, distribute spaces with extras to LEFT gaps.
// Time: O(total chars), Space: O(total chars) for output.
function justify(words, k) {
  const res = [];
  const n = words.length;
  let i = 0;
  while (i < n) {
    let j = i, lineLen = words[i].length;
    while (j + 1 < n && lineLen + 1 + words[j + 1].length <= k) {
      j++;
      lineLen += 1 + words[j].length;
    }
    const gaps = j - i;
    let line;
    if (gaps === 0) {
      line = words[i] + " ".repeat(k - words[i].length);
    } else {
      let totalChars = 0;
      for (let w = i; w <= j; w++) totalChars += words[w].length;
      const totalSpaces = k - totalChars;
      const base = Math.floor(totalSpaces / gaps);
      const extra = totalSpaces % gaps;
      const parts = [];
      for (let w = i; w <= j; w++) {
        parts.push(words[w]);
        if (w < j) parts.push(" ".repeat(base + (w - i < extra ? 1 : 0)));
      }
      line = parts.join("");
    }
    res.push(line);
    i = j + 1;
  }
  return res;
}

const words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
for (const line of justify(words, 16)) console.log(line);
