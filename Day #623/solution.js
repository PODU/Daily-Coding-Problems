// Full text justification: greedily pack words per line, distribute spaces evenly
// with extra spaces favoring left gaps; last/single word left-justified. Time O(total chars).
function justify(words, k) {
  const res = [];
  const n = words.length;
  let i = 0;
  while (i < n) {
    let j = i;
    let lineLen = words[i].length;
    while (j + 1 < n && lineLen + 1 + words[j + 1].length <= k) {
      j++;
      lineLen += 1 + words[j].length;
    }
    const gaps = j - i;
    let line;
    if (gaps === 0) {
      line = words[i] + ' '.repeat(k - words[i].length);
    } else {
      let totalSpaces = k;
      for (let w = i; w <= j; w++) totalSpaces -= words[w].length;
      const base = Math.floor(totalSpaces / gaps);
      const extra = totalSpaces % gaps;
      line = '';
      for (let w = i; w <= j; w++) {
        line += words[w];
        if (w < j) {
          const sp = base + (w - i < extra ? 1 : 0);
          line += ' '.repeat(sp);
        }
      }
    }
    res.push(line);
    i = j + 1;
  }
  return res;
}

const words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"];
for (const line of justify(words, 16)) console.log('"' + line + '"');
